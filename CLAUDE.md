# Macalla — Project Context for Claude Code

> *Macalla* is Irish for "echo" (Scottish Gaelic *mac-talla*): an app that catches
> what's said and plays it back.

## User
The operator — an experienced technologist re-learning native app builds by working in a
real codebase they didn't write. Comfortable with Claude Code, Git and the macOS command
line; hasn't forked and built someone else's app in a long time. Committed files never
name the user: personal details live in `CLAUDE.local.md`.

## This Project
Macalla is a personal fork of [fastrepl/anarlog](https://github.com/fastrepl/anarlog), an
open-source, on-device meeting recorder and transcriber (Tauri 2 + Rust + React). The goal
is learning: build the app from source on an Apple Silicon Mac and use it for real,
recording webinars without a meeting bot. Transcription runs on the local GPU, with a
third-party transcription API as the fallback. Changes to the app come later, if at all.

- **Full requirements:** [macalla/requirements.md](macalla/requirements.md)
- **Task register:** [TODO.md](TODO.md) — the only durable task list. Read it before
  planning any work.
- **Internal details:** `CLAUDE.local.md` — not committed (listed in `.git/info/exclude`),
  read it every session.

## Repo Model — Read Before Any Git Operation
- **This repo is a public GitHub fork.** Everything committed is public.
- **Nothing work-related is ever committed**: no employer, colleagues, customers, work
  meetings, and no names of video-conferencing products. Say "a live webinar" or "a
  video call". This covers code, docs, TODO.md and commit messages.
- **Branches:**
  - `main` mirrors `upstream/main` (fastrepl/anarlog). Fast-forward only. Never commit to it.
  - `macalla` (the fork's default branch) carries this project's files and any local
    changes. All project work happens here.
  - Upstream PR branches are cut from `main`, so they can never carry project files.
- **Remotes:** `origin` = the fork, `upstream` = fastrepl/anarlog.
- **Project files sit beside upstream's without colliding:** `CLAUDE.md`, `TODO.md` and
  the `macalla/` folder. Upstream's `docs/` is their published product-docs site, so
  project docs never go there. This is why requirements and session state live in
  `macalla/`, not `docs/`.
- **Local-only files** (`CLAUDE.local.md`, `collateral/`) are excluded via
  `.git/info/exclude`. Upstream's `.gitignore` is left untouched.
- **Commit identity:** repo-local personal identity only, set in `.git/config`. The global
  git identity must never author a commit here.

## Upstream's AI Files
Upstream ships its own agent config. None of it is deleted or edited, so upstream's files
stay byte-identical and PRs remain possible.
- **Use:** root `AGENTS.md` (workspace map, commands, CI-mirroring checks), nested
  per-component `AGENTS.md` files, `CONTRIBUTING.md`. Read the nearest `AGENTS.md` before
  changing a component. Don't copy their content here; point to it.
- **Ignore:** `AGENTS.md` sections on GitButler (they apply only on a `gitbutler/workspace`
  branch), the gh-enterprise helper, and "record decisions in Linear". Here, decisions
  go to `TODO.md` and `macalla/session-state.md`. Also ignore `.claude/skills/` (upstream
  maintainer operations: releases, newsletters, PR triage), `.codex/`, `.cursor/`.
- **Decline:** the expo plugin (`.claude/settings.json`) and the Expo MCP server
  (`.mcp.json`) when Claude Code offers them. They are for the mobile app, which is out of
  scope.

## Input / Output
- **Input:** the upstream source tree, plus audio from the mic and system output during a
  webinar.
- **Output:** a locally built Macalla/Anarlog desktop app that records and transcribes
  on-device. Build notes and findings go in `macalla/`.

## Interface
macOS desktop app (Tauri). Development happens through Claude Code in VSCode, with the
terminal for builds.

## Tech / Tooling
Upstream's stack, pinned by upstream. Don't pick different versions.
- **Rust 1.94.0** via `rust-toolchain.toml`, which needs **rustup**. Homebrew's `rust`
  ignores the pin.
- **Node 22+** (CI uses 22), **pnpm 11.1.1** (`package.json#packageManager`),
  **process-compose 1.122.0+**.
- **Full Xcode plus its Metal Toolchain.** Command Line Tools alone can't build the MLX
  shaders that Soniqo local transcription needs.
- Tauri 2 system prerequisites.
- Checkout path must contain no spaces (or set `CARGO_TARGET_DIR`).
- Coding subagents: Claude Code's built-in subagents by default.

## How to Work With This User
- The aim is learning: when a build step fails, explain *why* in a sentence before
  fixing it.
- Build first, change later. Don't propose code changes before Track 1 is done.
- Confirm before anything touching `main`, `upstream`, force-pushes, or the fork's
  GitHub settings.

## Environment
- First started: VSCode
- Date: 2026-10-31

---

## Tracks

### Track 1 — Own Build Recording a Webinar (BUILD)
- **Objective:** a from-source build of the desktop app running on the Apple Silicon Mac,
  recording a real webinar and transcribing it on the local GPU, with a third-party
  transcription API as the fallback. First test: a recorded webinar played from YouTube
  (same system-audio path). Final check: a live webinar.
- **Status:** Working tool in daily use (2026-11-01): bundled app records mic and system
  audio, transcribes on-device on demand (T-BUILD-09, closed 2026-11-07), summarizes
  via oMLX, CLI exports Markdown. Live webinars confirmed (T-BUILD-06, closed
  2026-11-13). Left: T-BUILD-07 (API fallback).
- **Depends on:** Nothing. This is the first track.

**Further tracks:** None yet. Candidates sit in `TODO.md` → Unscheduled.

---

## Session State

> Forward-looking operating context only. Backward-looking archaeology lives in
> `macalla/session-state.md`. Consult it on demand.

> Task state is not kept here: open items, progress and recurring work live in `TODO.md`.

**Last:** 2026-11-13. Added a thinking Off option for the Custom provider (T-BUILD-11,
closed): oMLX summaries now return fast. The operator's own simplified template
"010 Actionable Meeting" is the default; Auto unchanged. Closed T-BUILD-06 (many live
webinars). Synced upstream: web docs and CI only. The tool is in comfortable daily use.
Later: T-PROJ-04 detailed (staged holiday-planning call); triage sheets v1-v5 built with
a new personal `triage-sheet` skill. Last thread: why `target/` is 34 GB; found the
debug bundle runs Soniqo's Swift/MLX unoptimized; opened T-BUILD-12 (release build with
debug assertions on, timed); triage sheet v6.

**Next session — pick up here:**
1. Cold start: re-read CLAUDE.md and CLAUDE.local.md. Consult `macalla/session-state.md`
   on demand; `macalla/build-notes.md` has the rebuild steps and day-to-day settings.
2. **T-BUILD-12 first** (unless something new comes up): release build with debug
   assertions on, timed before and after. Steps in TODO.md → Item detail. Start with
   the baseline; the operator picks the recording to copy. Leave `target/` alone until
   then: the daily-use app lives in `target/debug/bundle`.
3. T-BUILD-07 (kept): the last open Track 1 item. Settle the open questions in TODO.md
   (which provider; manual switch or automatic failover). It is about *transcription*;
   off-machine *summaries* are already allowed per meeting.
4. Next fork change: remember the last note view (TODO.md → Unscheduled). Propose
   T-BUILD-10 and agree it before starting.
5. T-PROJ-03: the operator is testing first; then decide on the upstream PR from branch
   `remove-agent-hook-installer` (local only, not pushed).
6. On each upstream sync, check whether the Pro-gating branch has landed (TODO.md →
   Standing operations). If it has, template editing needs a fork unlock.
7. T-PROJ-04 demo video once things are stable. When Track 1 closes, agree a Track 2
   from the Unscheduled list.
8. Refresh `.atlas/overview.md` (local-only, excluded) when the project moves; one
   `[NEEDS]` left open by choice (proof screenshots).

**Continuity reminders:**
- The global git email is a work address. The repo-local identity is set, but re-check
  `git config --show-origin user.email` on any fresh clone before committing.
- Upstream's `.claude/skills/` load into every session here. Never invoke them; they are
  for upstream maintainers.
- Toolchain is per-repo via direnv (`.envrc`, local-only). Non-interactive shells need
  `direnv exec . <cmd>`; after editing `.envrc`, run `direnv allow .`. Never link
  Homebrew rustup, cmake or bash globally.
- Record only from the bundle (Finder or `open`), never `pnpm dev:desktop`: dev mode
  attributes audio permissions to the editor. Every rebuild must be re-signed with the
  identifier requirement or permissions are lost (exact steps in build-notes).
- Never let the app write to `~/.claude` or other global agent config; it's an absolute
  requirement. Keep `MACALLA_DISABLE_AGENT_SKILLS` and the removed hook installer.
- Transcription stays local on Soniqo Parakeet Batch (it keeps speaker labels; oMLX
  speech is consolidation-only, Unscheduled). No cloud transcription fallback without an
  explicit choice (T-BUILD-07). Summaries may go off-machine, chosen per meeting.
- oMLX needs its API key (never in the repo) and must never run alongside Ollama. It
  ignores Reasoning effort Low/Medium/High; Off works (T-BUILD-11) and is in use.
- Node 26.0.0 triggers an engine warning (wants ≥26.12.1 or 22/24); harmless so far.
- `reminders/` (local-only, in `.git/info/exclude`) holds disposable triage-sheet PDFs
  only. Make them with the personal `/triage-sheet` skill (`~/.claude/skills/`, not in
  this repo).
- `collateral/` holds only the operator's input to Claude. Tooling and output Claude
  builds go in `lab/` (local-only, in `.git/info/exclude`).
- Titles are manual: the operator relabels processed notes with a `DONE - ` prefix, so
  don't add AI auto-titling or an auto date prefix. The AI Summary's first line is the
  same title field as the header.
- Summary "failed" status is in-memory only; a relaunch clears it and leaves the Summary
  tab showing just the title. That isn't a fix.
- On-demand mode: tests force `macallaFork.onDemandProcessing = false`
  (`src/test-setup.ts`) so upstream's suites keep their meaning. The full desktop suite
  (run from `apps/desktop`) has 3 failures that predate this work: billing (2) and
  app-icon (1, from the icon-picker unlock).
- App logs: `~/Library/Logs/com.hyprnote.dev/app.log`; session audio:
  `~/Library/Application Support/com.hyprnote.dev/sessions/<id>/`. Read-only.

**Session continuity:** Next session is a cold start. Re-read CLAUDE.md before doing
anything. Sessions are never re-entered. End-of-session updates cover three layers: this
brief, `macalla/session-state.md`, and `TODO.md`.
Full archaeology: `macalla/session-state.md` — consult when prior-thread depth, decision
reasoning, or ruled-out paths are needed.
