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
- [ ] T-PROJ-01 Set a repo-local personal commit identity (hard gate before any commit)
- [ ] T-PROJ-02 Create the fork, place it at the project path, and add the `macalla` branch with the project files

### BUILD — Track 1: Own Build Recording a Webinar
- [ ] T-BUILD-01 Install the pinned toolchain: rustup (Rust 1.94.0), pnpm 11.1.1, process-compose, Xcode Metal Toolchain
- [ ] T-BUILD-02 Install workspace dependencies and get `pnpm dev:desktop` to launch the app
- [ ] T-BUILD-03 Grant macOS microphone and system-audio permissions and confirm both are captured
- [ ] T-BUILD-04 Set up GPU on-device transcription (Soniqo or whisper.cpp) and confirm it transcribes
- [ ] T-BUILD-05 Test: record and transcribe a recorded webinar played from YouTube
- [ ] T-BUILD-06 Final check: record and transcribe a live webinar
- [ ] T-BUILD-07 Configure a third-party transcription API as the fallback and confirm it transcribes

## 2. Item detail

### PROJ
- **T-PROJ-01** — The global git email is a work address and must never author a commit
  on this public fork. Set `user.name` / `user.email` in the repo's `.git/config`, using
  the personal address in `CLAUDE.local.md`. Runs inside T-PROJ-02, right after
  `git init` and before the first commit. Verify with `git config --show-origin
  user.email`.
- **T-PROJ-02** — On hold until the operator has a tar backup of the project directory.
  Then it needs an explicit go, because it's destructive (the current `.git` is
  replaced). Steps:
  1. `gh repo fork fastrepl/anarlog --fork-name macalla --clone=false`.
  2. At the project path: remove the old `.git` and the scaffolding that doesn't belong
     in the fork, then `git init`, add `origin` (fork) and `upstream`, fetch, and check
     out `main` tracking `upstream/main`.
  3. `git switch -c macalla`, commit `CLAUDE.md`, `TODO.md`, `macalla/`, push, and set
     `macalla` as the fork's default branch.
  4. Add `CLAUDE.local.md` and `collateral/` to `.git/info/exclude`.
  5. Scrub check before the first push: nothing personal or work-related in any
     committed file.

### BUILD
- **T-BUILD-01** — What's on the machine now: Homebrew `rust` 1.99.1 (no rustup), Node 26,
  no pnpm, no corepack, no process-compose, Xcode.app present but Metal Toolchain not
  yet checked. `brew install rustup` and make sure rustup's shims come before Homebrew
  `rust` on PATH. Then `rustc --version` inside the repo must report 1.94.0. Metal:
  `xcodebuild -downloadComponent MetalToolchain`. Also check the Tauri 2 macOS
  prerequisites.
- **T-BUILD-02** — `pnpm install --frozen-lockfile`, then `pnpm dev:desktop`. Needs no
  secrets. Desktop Vite uses port 1422 and the process-compose control port is 18081.
  Expect the first Rust build to be long.
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
| Sync upstream | Before a build session, or when upstream fixes are wanted | — | Fast-forward `main` to `upstream/main`, then merge `main` into `macalla`. Re-check pinned toolchain versions after syncing. |

## 5. Unscheduled
Ideas that came up but aren't committed work. No ID until promoted into a track.
- Make one small change of our own to learn how the Rust backend, Tauri bridge and React
  frontend connect. Compare Claude Code's built-in subagents with the operator's own
  coding-subagent framework on the same change.
- Bring in a repo-scanning/query tool before touching the Rust audio or transcription
  internals, if the work gets that deep.
- Summaries through a local model (Ollama or LM Studio).
- Contribute a fix upstream (branch from `main`, never `macalla`).

## 6. Archive
Closed items whose resolution no longer informs current work.
