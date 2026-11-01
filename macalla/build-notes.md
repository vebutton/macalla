# Macalla — Build Notes

How the desktop app gets built from source on this Apple Silicon Mac, and the snags hit
along the way. Upstream's own instructions are in `CONTRIBUTING.md` and `AGENTS.md`; this
file only records what differs or what they leave out.

## Toolchain, sandboxed to this repo

The machine is shared with other projects, so nothing Macalla needs is put on the global
PATH. A local-only direnv `.envrc` (excluded from git) adds it while inside the repo:

| Tool | Source | How it's scoped |
|---|---|---|
| Rust 1.94.0 | Homebrew `rustup` (keg-only), honours `rust-toolchain.toml` | `.envrc` puts `/opt/homebrew/opt/rustup/bin` first; Homebrew `rust` stays the default elsewhere |
| cmake | Homebrew `cmake`, then `brew unlink cmake` | `.envrc` adds `/opt/homebrew/opt/cmake/bin` |
| process-compose | Release binary from GitHub, checksum verified | Lives in local-only `.tools/bin`, added by `.envrc` |
| pnpm | Homebrew `pnpm` (global) | Reads `packageManager` and runs 11.1.1 here |
| direnv | Homebrew `direnv` + `eval "$(direnv hook zsh)"` last in `~/.zshrc` | Inert outside folders with an allowed `.envrc` |

`.envrc` also sets `TURBO_TELEMETRY_DISABLED=1`. After any edit to `.envrc`, run
`direnv allow .`; direnv blocks a changed file until it is re-approved.

Non-interactive shells (e.g. an agent's shell) don't run the direnv hook. Use
`direnv exec . <command>` there.

### Side effects to watch
- direnv pulls in Homebrew `bash` as a dependency. It was unlinked (`brew unlink bash`) so
  `/usr/bin/env bash` still finds macOS's `/bin/bash`; direnv calls its own bash by full
  path. After a `brew upgrade`, check `which bash` and `which cmake` (outside the repo)
  haven't been re-linked.
- `pnpm install` downloads a headless Chrome to `~/.cache/puppeteer` (a dev dependency's
  postinstall), and pnpm keeps its package store in `~/Library/pnpm`.

## Running

```sh
pnpm install --frozen-lockfile
pnpm dev:desktop
```

Logs: `.process-compose/desktop/desktop.log` (JSON lines; process-compose labels
everything on stderr as `"level":"error"`, including Cargo progress, so grep the
message text, not the level). First full Rust build: ~1,800 crates.

## Snags and fixes

### cmake missing
`knf-rs-sys` (and `whisper-rs-sys`) build bundled C++ with CMake. Upstream's docs don't
list it as a prerequisite. Fix: cmake as above.

### Swift Package Manager and the keychain prompt
The Soniqo build script (`crates/transcribe-soniqo/build.rs`) runs
`swift package resolve`. SwiftPM looks in the macOS keychain for `github.com`
credentials before downloading the prebuilt `SpeechCore.xcframework`, which triggers a
"swift-package wants to use your confidential information" prompt. Clicking **Deny**
fails the build (`Failed to find credentials ... status -128`, then `fatalError`):
SwiftPM does not fall back to an anonymous download. Every dependency is public, so no
credential is needed.

Workaround (no code change): resolve once by hand with the keychain disabled, into the
scratch path the build script uses, then rebuild:

```sh
OUT=$(ls -d apps/desktop/src-tauri/target/debug/build/transcribe-soniqo-*/out)
direnv exec . swift package --disable-keychain \
  --package-path crates/transcribe-soniqo/swift-lib \
  --scratch-path "$OUT/swift-rs/soniqo-swift" resolve
```

This must be repeated if `target/` is cleaned or the crate's build hash changes (e.g.
after an upstream sync touches it). The proper fix is upstream passing
`--disable-keychain` in `build.rs`.

### Node version warning
Upstream's `engines` wants Node `^22.23.4 || ^24.22.1 || >=26.12.1`; this machine has
26.0.0. Warning only so far. If it bites, pin a newer Node for this repo via `fnm` in
`.envrc`.

## Dev-mode code signing and permissions
`apps/desktop/scripts/dev-runner.mjs` ad-hoc signs `target/debug/desktop` on every run
with the fixed identifier `com.hyprnote.dev` and a designated requirement on that
identifier alone, so macOS permission grants can survive rebuilds. Which app macOS
actually attributes a grant to, when the binary is launched from a terminal or editor,
is T-BUILD-03.

## Global agent config: nothing written
The Rust side can install a Claude Code `Stop` hook (and Codex / opencode equivalents)
into the user's global agent config. The hook runs `char claude notify`. History: in
March 2026 the CLI was called `char` and `claude notify` read the hook's JSON from stdin,
with a "TODO: write to app DB" (unfinished agent-activity logging). The subcommand was
removed days later and the CLI was rebuilt as `anarlog` in July, but the installer still
writes the dead command. No desktop UI calls the installer; on startup the app only
refreshes the hook if one is already present, so it's inert unless installed by hand.

Removed: branch `remove-agent-hook-installer` (cut from `main`, offered upstream as
T-PROJ-03) deletes the installer and the startup refresh; it is merged into `macalla`.

Settings → Developers → Skills writes an `anarlog` skill into `~/.claude/skills/` (and
the Codex, Cursor, opencode equivalents). That is a live, user-initiated feature, so it
is disabled only in this fork: `MACALLA_DISABLE_AGENT_SKILLS` in
`apps/desktop/src-tauri/src/commands.rs`, mirroring the App Store build.

## Bundled app (for recording)
Dev mode can't hold macOS audio permissions: the prompt names the launching editor or
terminal, not the app. Recording uses an ad-hoc-signed debug bundle instead:

```sh
direnv exec . cargo xtask prepare-binaries     # CLI + browser helper, release builds
cd apps/desktop && direnv exec . pnpm tauri build --debug --bundles app \
  --features direct-distribution \
  --config '{"bundle":{"createUpdaterArtifacts":false,"externalBin":["binaries/check-permissions","binaries/char-chrome-native-host","resources/cli/anarlog-cli"],"macOS":{"signingIdentity":"-"}}}'
cd ../.. && codesign --force --sign - --identifier com.hyprnote.dev \
  --requirements '=designated => identifier "com.hyprnote.dev"' \
  --entitlements apps/desktop/src-tauri/Entitlements.plist \
  "apps/desktop/src-tauri/target/debug/bundle/macos/Anarlog Dev.app"
```

- `createUpdaterArtifacts: false`: upstream signs updates with a private key we don't have.
- The re-sign matters: Tauri's ad-hoc signature pins the exact binary (`cdhash`), so every
  rebuild would look like a new app to macOS and lose its permissions. Pinning the
  requirement to the identifier (as upstream's dev runner does) keeps grants across builds.
- Launch from Finder or `open`, never from a terminal, so macOS treats the app as
  responsible for itself. Output: `apps/desktop/src-tauri/target/debug/bundle/macos/`.

## Blocked: silent cloud fallback for live transcription
With no live-capable model configured (e.g. Soniqo's model still downloading), upstream
starts the session anyway with an empty server URL, which resolves to the Deepgram adapter
and dials Deepgram's cloud (401 without a key). Macalla guards this in `listener-core`: an
empty URL resolves to after-recording mode (`resolve_transcription_mode`), and the live
adapter refuses an empty URL. Batch always names an explicit provider, so it has no
equivalent silent fallback.
