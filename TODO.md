# Macalla — ToDo

> The single durable task register. Committed to a **public** repo, so every line stays
> generic: no personal details, nothing work-related, no video-conferencing product
> names. Specifics live in `CLAUDE.local.md`. Build and resolution dates are fine.

## Rules
- **ID:** `T-<SLUG>-<nn>`. Permanent, never reused, never renumbered. Numbered per track.
- **Slugs** come from CLAUDE.md → Tracks, plus `PROJ` for project/repo plumbing that
  belongs to no track. A new track is added to CLAUDE.md first, by agreement, never
  invented here.
- **An item is work that finishes.** Anything that recurs, or would never close, is a
  Standing operation (section 4): no ID, no checkbox.
- **Track and deliverable are fixed at creation.** If the deliverable changes, close the
  item with a pointer and open a new one that references the old ID. Don't retitle.
- **New items** append at the end of their track. Propose each one with a proposed ID
  and wait for agreement.
- **Closed items** stay in place, ticked: `(resolved YYYY-MM-DD: outcome)`. Move them to
  Archive only when the resolution no longer informs current work.
- **Item detail is current state only:** what's left, what it waits on, priority,
  dependencies, pointers. What happened and when goes in `macalla/session-state.md`.
- **Open questions live only here** (section 3). Requirements and design docs point to
  this section instead of keeping their own lists.
- **This is the only durable task list.** The in-session todo tool is scratch. At end of
  session, reconcile it into this file, and give every open or next-step line written to
  `macalla/session-state.md` an ID or an Unscheduled line, or drop it deliberately.
- **Refer to items by ID everywhere** (CLAUDE.md, docs, commit messages), never by
  position ("pickup item 3"). A commit that does an item's work names its ID.

## 1. Items

### PROJ — Project and repo plumbing
- [x] T-PROJ-01 Set a repo-local personal commit identity (hard gate before any commit) (resolved 2026-10-31: set in `.git/config`; verify with `git config --show-origin user.email` on any new clone)
- [x] T-PROJ-02 Create the fork, place it at the project path, and add the `macalla` branch with the project files (resolved 2026-10-31: fork public, `macalla` is the default branch, `main` tracks `upstream/main`, local-only files in `.git/info/exclude`)
- [ ] T-PROJ-03 Upstream PR: remove the dead coding-agent hook installer (branch `remove-agent-hook-installer`, cut from `main`)

### BUILD — Track 1: Own Build Recording a Webinar
- [x] T-BUILD-01 Install the pinned toolchain: rustup (Rust 1.94.0), pnpm 11.1.1, process-compose, Xcode Metal Toolchain (resolved 2026-10-31: sandboxed to this repo via a local-only direnv `.envrc`: keg-only Homebrew rustup gives 1.94.0 here while Homebrew `rust` stays the default elsewhere; process-compose 1.124.0 release binary in local-only `.tools/bin`; Homebrew pnpm auto-switches to 11.1.1; Metal Toolchain was already present)
- [x] T-BUILD-02 Install workspace dependencies and get `pnpm dev:desktop` to launch the app (resolved 2026-10-31: app launches to onboarding; needed cmake and a keychain-free Swift resolve, see `macalla/build-notes.md`)
- [ ] T-BUILD-03 Grant macOS microphone and system-audio permissions and confirm both are captured
- [ ] T-BUILD-04 Set up GPU on-device transcription (Soniqo or whisper.cpp) and confirm it transcribes
- [ ] T-BUILD-05 Test: record and transcribe a recorded webinar played from YouTube
- [ ] T-BUILD-06 Final check: record and transcribe a live webinar
- [ ] T-BUILD-07 Configure a third-party transcription API as the fallback and confirm it transcribes
- [ ] T-BUILD-08 Build an ad-hoc-signed `.app` bundle with the embedded `anarlog` CLI, and confirm the CLI lists and reads recent meetings

## 2. Item detail

### BUILD
- **T-BUILD-03** — Dev mode launches `target/debug/desktop` from a shell, and upstream has
  no code that disclaims the parent process, so macOS will likely attribute microphone and
  system-audio grants to the launching app (the editor or terminal), not to Anarlog. Check
  which app name the macOS prompt shows before granting. Onboarding also asks for
  Accessibility ("read meeting activity"); not needed for recording.
- **T-BUILD-08** — Dev mode can't hold macOS audio permissions (they go to the launching
  editor), so recording needs a bundle; T-BUILD-03 waits on this. `cargo xtask
  prepare-binaries` builds the CLI into `resources/cli/`; `tauri build --debug --bundles
  app` with a `--config` override (ad-hoc signing, no updater artifacts, CLI listed in
  `externalBin`) keeps upstream files untouched. On launch the app symlinks the CLI to
  `~/.local/bin/anarlog-dev` (dev identifier). The CLI picks its database by the name
  it's invoked as (`anarlog-dev` → the dev app's database), so a plain `macalla` symlink
  would read the wrong one; a `macalla` wrapper must exec `anarlog-dev` or pass
  `--db-path`. `meetings list` is newest-first.
- **T-BUILD-04** — Both GPU paths run on Metal: Soniqo (MLX, downloaded in-app) and
  "On-device file" (a whisper.cpp `.bin`). Apple Speech isn't a GPU path. Each model is
  labeled **Live** or **After recording** in Settings → Transcription; note which label
  each one has.
- **T-BUILD-05** — A YouTube webinar goes through the same system-audio capture as a live
  call, so it's a cheap, repeatable first test.
- **T-BUILD-07** — Upstream supports bring-your-own-key transcription providers (Settings
  → Transcription). The key stays in the app's local settings, never in the repo. A
  hosted provider receives the meeting audio. Provider choice and the meaning of
  "fallback" are open questions (section 3).

## 3. Open questions
- Does "own build running" mean dev mode (`pnpm dev:desktop`), or a bundled `.app`
  (ad-hoc signed, no paid developer account)? Resolves at T-BUILD-06.
- In dev mode, macOS may attach audio permissions to the launching terminal or editor
  rather than the app. Does that affect capture? Resolves at T-BUILD-03.
- Soniqo or whisper.cpp for GPU transcription? Pick on accuracy, live vs after-recording,
  and model size. Resolves at T-BUILD-04.
- Which third-party transcription API is the fallback? Resolves at T-BUILD-07.
- Does "fallback" mean switching provider by hand when the GPU path fails (which
  upstream appears to support), or automatic failover (which would be a code change,
  beyond Track 1)? Resolves at T-BUILD-07.

## 4. Standing operations
Recurring work: no IDs, no checkboxes, tracked by last-run date.

| Operation | Cadence / trigger | Last run | Notes |
|---|---|---|---|
| Sync upstream | Before a build session, or when upstream fixes are wanted | 2026-10-31 (already level) | Fast-forward `main` to `upstream/main`, then merge `main` into `macalla`. Re-check pinned toolchain versions after syncing. |

## 5. Unscheduled
Ideas that came up but aren't committed work. No ID until promoted into a track.
- Make one small change of our own to learn how the Rust backend, Tauri bridge and React
  frontend connect. Compare Claude Code's built-in subagents with the operator's own
  coding-subagent framework on the same change.
- Bring in a repo-scanning/query tool before touching the Rust audio or transcription
  internals, if the work gets that deep.
- Summaries through a local model (Ollama or LM Studio).
- Contribute a fix upstream (branch from `main`, never `macalla`).
- Start/stop recording from a script (e.g. an `anarlog://` deep link callable with `open`),
  a candidate for the small-change learning item above. First check whether calendar-based
  "Start when meeting begins" / "Stop when meeting ends" already covers webinars (auto-stop
  may rely on the mic, which a listen-only webinar doesn't use).

## 6. Archive
Closed items whose resolution no longer informs current work.
