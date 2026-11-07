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
- [x] T-BUILD-03 Grant macOS microphone and system-audio permissions and confirm both are captured (resolved 2026-11-01: granted to the bundled app; mic and system audio both transcribed in a test recording)
- [x] T-BUILD-04 Set up GPU on-device transcription (Soniqo or whisper.cpp) and confirm it transcribes (resolved 2026-11-01: Soniqo Parakeet Batch, after recording; 52 s test transcribed locally in ~3 s with no network transcription attempt)
- [x] T-BUILD-05 Test: record and transcribe a recorded webinar played from YouTube (resolved 2026-11-01: a 48-minute recorded video transcribed completely on-device, 6,940 words with no missing blocks, about 4 minutes after stop including speaker labels; headset, mixed and speaker audio also tried in a two-way video call)
- [ ] T-BUILD-06 Final check: record and transcribe a live webinar
- [ ] T-BUILD-07 Configure a third-party transcription API as the fallback and confirm it transcribes
- [x] T-BUILD-08 Build an ad-hoc-signed `.app` bundle with the embedded `anarlog` CLI, and confirm the CLI lists and reads recent meetings (resolved 2026-11-01: bundle in daily use; `macalla` wrapper in `~/.local/bin` lists, reads and exports meetings to Markdown; rebuild steps in `macalla/build-notes.md`)
- [ ] T-BUILD-09 On-demand transcription and summaries (fork change): nothing runs automatically after a recording, upload or relaunch

## 2. Item detail

### BUILD
- **T-BUILD-06** — The tool is in daily use. What's left is a live webinar, recorded with
  the floating bar on and the tray icon off. Known limitations to expect: synthesized word
  timings (playback highlight and click-to-seek drift; text is fine), and far-end bleed
  into the mic channel when using a speaker rather than headphones. Worth trying on a
  speaker call: select the speakerphone's own mic, whose hardware echo cancellation
  should cut the bleed.
- **T-BUILD-07** — Upstream supports bring-your-own-key transcription providers (Settings
  → Transcription). The key stays in the app's local settings, never in the repo. A
  hosted provider receives the meeting audio. Provider choice and the meaning of
  "fallback" are open questions (section 3).
- **T-BUILD-09** — Switch `macallaFork.onDemandProcessing` in
  `apps/desktop/src/macalla.ts`. Confirmed in the app: nothing runs after a stop,
  Re-transcribe and Generate summary work on demand. One quirk left: right after a stop
  the note looks blank (no Memos/Transcript switcher, templates shown) until you open
  another note and come back. Commit `8576136607` (refresh the audio checks when a
  capture ends) is meant for this but isn't built yet. Next: rebuild, re-sign, and check
  whether the quirk is gone; close if so. Screenshots: `collateral/20261106 minor UI bug/`.
  Keeps local transcription and summaries from competing for the GPU. Doesn't apply with
  audio retention "none" (no saved audio to transcribe later).

## 3. Open questions
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
- Summaries through a bigger local model. Apple's on-device model (now selected) has a fixed 4K-token context and fails on long meetings
  (a 48-minute transcript did). The model will come from the operator's separate
  local-model project; connect it here as an OpenAI-compatible provider. Never run
  alongside Ollama. Automatic summaries go off with T-BUILD-09. In-app chunked
  summarisation ruled out (2026-11-05).
- Remember the last note view (Summary, Memos or Transcript) when switching notes; today
  every note opens on the summary. Opened notes start with `view: null`
  (`store/zustand/tabs/schema.ts`), and `useCurrentNoteTab` (`session/components/shared.tsx`)
  then falls back to the first summary. Small fork change; next after T-BUILD-09.
- Let the note title be edited while recording too (header input is still hidden while
  live; one-line change in `outer-header/index.tsx`). Renaming after recording is done.
- Contribute a fix upstream (branch from `main`, never `macalla`).
- Remove the upsell UI (e.g. "Get Pro" on the Summary tab) from the Macalla build. Done
  so far: the app-icon picker is unlocked.
- Real word timestamps for Soniqo Parakeet batch transcripts: keep the model's own
  timings instead of synthesizing them (`listener2-core` batch/simple/local.rs keeps only
  text; `transcribe-soniqo` responses.rs estimates timing). Upstream-worthy.
- Connect to a local OpenAI-compatible speech server (e.g. oMLX, on this Mac or another
  machine on the home network; both count as local) so one model serves every tool. Needs
  an adapter change: today any localhost URL is treated as Deepgram-compatible. Live
  transcription would come from another project once proven on the target hardware.
- Start/stop recording from a script (e.g. an `anarlog://` deep link callable with `open`),
  a candidate for the small-change learning item above. First check whether calendar-based
  "Start when meeting begins" / "Stop when meeting ends" already covers webinars (auto-stop
  may rely on the mic, which a listen-only webinar doesn't use).
- Direct calendar connection that bypasses the vendor's cloud (ICS feed or the app's own
  OAuth). Try first: add the calendar account to macOS Internet Accounts and use the
  app's local Apple Calendar provider, which never touches the vendor's servers.

## 6. Archive
Closed items whose resolution no longer informs current work.
