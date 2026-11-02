# Session State — Archaeology

> Backward-looking detail: thread-by-thread work, decisions and reasoning, ruled-out
> paths, commits, memories banked. The forward-looking brief lives in CLAUDE.md ->
> Session State. Consult this file on demand when depth is needed.

## 2026-10-31 — Scoping and fork setup

**Done**
- Scoped the project from a brainstorming conversation. Goal: build anarlog from source
  on an Apple Silicon Mac as a hands-on refresher on native app builds, and use it to
  record and transcribe webinars.
- Created the public fork `macalla` of fastrepl/anarlog (default branch only), set up
  `main` to track `upstream/main` (at upstream `5e9c98edb0`), created the `macalla` branch
  as the fork's default, and committed the project files there (`8cf4082df7`).
- Inventoried the local toolchain (see TODO.md → T-BUILD-01 detail).

**Decisions**
- **Macalla is the fork itself**, not a workspace repo holding a fork. It mirrors the
  plain "fork and clone" workflow while still carrying project context.
- **Branch model:** `main` mirrors upstream, project files live on `macalla`, upstream PR
  branches come from `main`. This keeps upstream PRs possible without deleting or
  editing upstream's files.
- **Project docs live in `macalla/`,** because upstream's `docs/` is their published
  product-docs site.
- **Upstream's AI config is left byte-identical.** Use AGENTS.md and CONTRIBUTING.md.
  Ignore the maintainer skills and the GitButler, gh-enterprise and issue-tracker
  sections. Decline the Expo plugin and MCP server.
- **Public-repo rule:** nothing work-related in committed files, including names of
  video-conferencing products.
- **Track 1 scope:** GPU on-device transcription first, a third-party API as the
  fallback.
- **Name:** *Macalla*, Irish for "echo" (Scottish Gaelic *mac-talla*). It fits an app that
  catches what's said and plays it back. It was chosen over a few Scots "Mac-" names.

**Ruled out**
- **A workspace repo with the fork in a subdirectory or submodule.** It would have
  separated project context from the code the operator actually works in.
- **Resetting history and creating a fresh repo.** It would have broken the fork
  relationship and any chance of upstream PRs.
- **Deleting upstream's agent files.** It would have diverged upstream files and made
  PRs messy. Ignoring them costs nothing.
- **Apple Speech as the primary engine.** It's on-device but doesn't run on the GPU.

## 2026-10-31 to 2026-11-01 — Build, first recordings, daily use

**Done**
- **Toolchain (T-BUILD-01),** sandboxed per repo with direnv:
  - keg-only Homebrew rustup (honours `rust-toolchain.toml`, 1.94.0);
  - process-compose release binary in `.tools/bin`;
  - Homebrew pnpm auto-switches to 11.1.1;
  - cmake installed but unlinked, added via `.envrc`;
  - Homebrew bash unlinked after direnv pulled it in.
- **First launch (T-BUILD-02).** Needed cmake (`knf-rs-sys`, `whisper-rs-sys`) and a
  manual `swift package --disable-keychain resolve`. Clicking Deny on SwiftPM's keychain
  prompt had broken the build (bad advice from the agent, corrected).
- **Bundled app (T-BUILD-03, T-BUILD-08).**
  - Dev mode attributed the mic prompt to the editor; the grant was reset with `tccutil`.
  - Switched to an ad-hoc-signed debug bundle, re-signed with an identifier requirement
    so permission grants survive rebuilds.
  - The CLI is exposed as `macalla` through a wrapper, because the CLI picks its database
    by the name it's invoked as.
- **Global agent config.**
  - Branch `remove-agent-hook-installer` (`54d8bdeeb2`, from `main`) removes the dead hook
    installer that wrote into coding-agent config. It's merged into `macalla` and is the
    T-PROJ-03 PR candidate.
  - `3d080366b3` disables agent-skill installation in this fork only.
- **Cloud fallback blocked (`d18c16b54b`).** An empty transcription URL resolved to the
  Deepgram adapter and dialled its cloud. It now resolves to after-recording mode, and
  the live adapter refuses an empty URL. Unit test added; 143 listener-core tests pass.
- **Local transcription (T-BUILD-04).** Soniqo Parakeet Batch (CoreML, about 640 MB,
  downloaded from Hugging Face without asking). A 52-second test took about 3 seconds.
- **Long test (T-BUILD-05).** A 48-minute recorded video produced 6,940 words.
  - Every 29.5-second block on the system channel produced text.
  - Transcription took about 2 minutes after stop, plus about 2 minutes of speaker labelling.
  - Speaker bleed added about 480 garbled mic-channel words.
- **Two-way video call.** Headset, mixed and speaker audio were all tried.
  - The floating bar is excluded from capture: unseen by the participant and absent from
    screenshots.
  - The tray icon (note title plus level meter) was seen in a full-screen share, so the
    operator turned it off.
- **CLI checked.** `list`, `get` and `export --format markdown` all work; the operator
  also tested export from the UI.
- **Icons.** Blueprint icons rotated 180° across all copies, and the Pro lock on the
  icon picker removed (`MACALLA_UNLOCK_APP_ICONS`). Rebuilt and re-signed.
- **Intelligence.** Opening the page selected Apple's on-device model, which back-filled
  summaries for old notes. The 48-minute note failed: context exceeded.

**Decisions**
- **Transcription.**
  - Soniqo Parakeet Batch, after recording only; live transcript isn't needed for now.
  - oMLX (on this Mac or the coming Mac Studio) is the long-term single local speech engine.
  - Another project prototypes live transcription and hands it over when proven.
  - Ollama and oMLX never run at the same time.
- **The app must never write to global coding-agent config,** not even via dead code.
- **Upstream PR is preferred over a forum post,** but ranks below local functionality.
- **Name.** `macalla` is the CLI name via a wrapper; the app itself is not renamed.
- **Upsell.** "Get Pro" removal is deferred (Unscheduled); the icon picker is already
  unlocked.
- **Daily use.** The tray icon stays off and the floating bar stays on. Speaker bleed is
  accepted for speakerphone meetings.

**Ruled out / corrected**
- **Recording from dev mode:** TCC attributes it to the editor.
- **Keeping Tauri's cdhash-pinned ad-hoc signature:** grants would be lost on every rebuild.
- **Silence trimming as the cause of transcript misalignment.** The real cause: Soniqo
  batch keeps only each block's text and synthesizes word timings from the block start
  (Unscheduled: real timestamps).
- **A Settings button for the hook installer.** Earlier the agent claimed one existed; no
  UI calls the installer.
- **Pasting a custom icon via Finder's Get Info:** it would modify the signed bundle.
