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

**Session close (2026-11-01)**
- `macalla` pushed to origin (level at `e1c132d426` before this closing commit).
  `remove-agent-hook-installer` remains local-only.
- Atlas overview created at `.atlas/overview.md` (local-only via `.git/info/exclude`,
  not `.gitignore`, to keep upstream's file untouched). The project is recorded as a
  personal tool; one marker (proof screenshots) is left open on purpose.

## 2026-11-05 — Note titles: rename after recording

**Done**
- **Summary context.** Confirmed the Apple on-device model's 4,096-token window is set by
  Apple, not by the app: `plugins/local-llm/swift-lib/src/lib.swift` only opens a
  `LanguageModelSession`, and `maximumResponseTokens` shares the same budget. About 20
  minutes of speech fits.
- **Rename after recording (fork-only change).** Upstream hid the header title input once
  a note had audio or a transcript, and also whenever the Summary/Transcript switcher
  showed (upstream `97302a87a3`). It now stays visible except while recording or
  finalizing (`apps/desktop/src/session/components/outer-header/index.tsx`). Tests
  updated plus one new case; 35/35 pass, typecheck clean. Bundle rebuilt and re-signed;
  the operator confirmed it works.
- **How titles work, for reference.** The AI Summary's first line is the session title,
  synced both ways (`note-input/enhanced/editor.tsx`). That was upstream's only rename
  path after recording, so notes with a failed or missing summary were stuck on
  Untitled. Clearing a title on a note with a summary refills it from the summary's
  first line (`repairMissingSessionTitle`).
- **Failed summaries after relaunch.** The "failed" status lives in memory
  (`useAITaskTask`), so relaunching cleared the errors. Those Summary tabs now show only
  the title line. Nothing was regenerated; a retry would fail the same way.

**Decisions**
- **Manual titles, not AI titles.** The operator names notes with a date-prefixed scheme
  and relabels processed notes with a `DONE - ` prefix, so auto-generated titles don't fit.
- **No auto-filled date prefix:** it would get in the way of the `DONE - ` relabel.
- **Bigger summary model:** comes from the operator's separate local-model project, not
  from this repo.

**Ruled out**
- **Raising Apple's context window:** not configurable.
- **Chunked (map-reduce) summarisation in the app:** the operator already has other ways
  to handle long transcripts.
- **Editing titles directly in the database:** not needed once the UI allows rename, and
  risky while the app runs.

## 2026-11-06 — Summary prompt sample; on-demand transcription and summaries

**Done**
- **Where the summary prompt lives.** "Enhance" is summary generation. Jinja templates in
  `crates/template-app/assets/` (`enhance.system.md.jinja`, `enhance.user.md.jinja`,
  `enhance.format.md.jinja`, `_macros.jinja`), rendered by minijinja in
  `crates/template-app/src/enhance.rs`. TypeScript appends Summary Mode, length guidance
  and preferred names (`store/zustand/ai-task/task-configs/enhance-workflow.ts`). Length
  policy in `crates/summary/src/lib.rs`.
- **Sample prompt for model testing.** Rendered the full prompt, with a length helper and
  the optional blocks, to `collateral/20261106-enhance-prompt-sample.md` (local-only) for
  the operator's oMLX tests. Questions about template, custom format, dictionary terms,
  session line and speaker labels were asked and not answered. Dropped deliberately
  unless the operator returns to them.
- **Model test results (from the operator's other project).** With this prompt, local
  Qwen and Gemma summaries have not passed the 8/10 key-facts bar against Claude Opus;
  the best scored 7.5. Debugging belongs to that project. One possible factor raised
  here: the prompt carries upstream's length cap (2–8 sections, output no longer than
  the transcript), tuned for Apple's 4K model.
- **GPU contention, answered.** Soniqo runs through one Swift actor, so transcriptions
  are effectively serialized; summaries (Apple's model) have no global lock and can
  overlap a transcription. A new recording can start while another note processes.
- **T-BUILD-09, on-demand processing (fork change, `6ba76ed5c6`).** New
  `apps/desktop/src/macalla.ts` with `macallaFork.onDemandProcessing = true`. Gates:
  - enhancer service: `requestAutoEnhance`, `queueAutoEnhanceIfSummaryEmpty` (returns
    `{type: "on_demand"}`) and `resumePendingAutoEnhance` return early; manual
    `enhance()` unchanged.
  - `useUploadFile.ts`: no automatic summary after an upload.
  - `capture-lifecycle.ts`: `transcriptionDeferred` skips the post-stop batch and the
    recovery request; the capture still finishes cleanly, audio stays marked
    "processing" so retention won't delete it. `stampSessionCaptureInterval`
    (`capture-completion.ts`) writes the session's start/end times, which upstream only
    wrote after a transcript.
  - Tests set the flag false in `src/test-setup.ts`; two new tests. Built, re-signed.
- **Post-stop audio check (`8576136607`).** First test: after a stop the Transcript tab
  said "No transcript available" with no Re-transcribe, though `audio.mp3` was saved.
  Cause: the `["audio", id, "exist"]` query was cached from before the recording and
  nothing refreshed it; upstream never shows that screen because auto-transcription
  fills the transcript. Fix: `session/index.tsx` invalidates the audio exist/url queries
  when the note's capture ends. Typecheck, lint, fmt clean. Not built yet: the app
  wouldn't quit by AppleScript and wasn't force-killed.
- **Operator test on the old build.** On-demand Re-transcribe and Generate summary both
  work. Quirk: right after a stop the note looks blank (no switcher, templates shown)
  until switching notes and back. Screenshots in `collateral/20261106 minor UI bug/`.
  Whether `8576136607` covers it is untested.

**Decisions**
- **Hardcoded fork constant, not a setting.** Simplest, and the operator wants it on.
- **Retention "none" isn't covered:** no saved audio to transcribe later.
- **Recording-import jobs still transcribe and summarize** (explicit user action).

**Known limitations (accepted)**
- After a manual Re-transcribe the transcript's start time is the click time (upstream
  does the same); session times are correct.
- The button says "Re-transcribe" even for a first transcription.
- Summary jobs left pending by earlier back-fills stay idle.

**Session close (2026-11-06)**
- `macalla` pushed to origin, including `6ba76ed5c6` and `8576136607`.
- Full desktop suite from `apps/desktop`: 4221 pass, 3 pre-existing failures (billing 2,
  app-icon 1), confirmed failing without this session's changes.

## 2026-11-07 — Summaries through oMLX

**Done**
- **oMLX connected for summaries, no code change.** The operator's other project handed
  over an interface test (local-only `collateral/20261107-macalla-omlx-lfm2-handoff.md`)
  that expected the localhost→Deepgram routing to block it. That rule is
  transcription-only. Summaries go through the LLM provider list, whose **Custom**
  provider is `createOpenAICompatible` with base URL plus Bearer key
  (`src/ai/hooks/useLLMConnection.ts`, default branch). The Tauri HTTP allowlist already
  covers `http://*.*.*.*:*/**`; model-name filters don't hide "Transcript" names (they
  block "transcribe"). Setup in `macalla/build-notes.md` → Day-to-day use.
- **Test with LFM2-2.6B:** worked, streamed progressively, no format error or retry.
  Claude scored the summary about 5/10, in line with the other project's 4–5.5.
- **Format check is early-only** (`shared/validate.ts`, `enhance-validator.ts`): first
  ~30 characters, strips any preamble before `#`, requires `# `, fuzzy-matches the
  template's first section title. That's why LFM2's mid-document drift caused no retry.
- **Thinking:** the Reasoning effort dropdown sends `reasoning_effort` for Custom; the
  operator confirmed oMLX ignores it and thinking models think by default (wanted).
  `@ai-sdk/openai-compatible` 2.0.41 passes unknown provider options into the request
  body, so an "Off" option sending `chat_template_kwargs.enable_thinking: false` would be
  a few lines in `src/ai/reasoning-effort.ts`. Not done.
- **T-BUILD-09 closed.** Rebuilt the debug bundle (frontend-only change, so no
  `prepare-binaries`; Rust incremental 36 s) and re-signed with the identifier
  requirement. The operator confirmed a stopped recording shows the Memos/Transcript
  switcher immediately, so `8576136607` fixed the post-stop quirk.
- **Report back** to the other project written to local-only `collateral/` (20261107).

**Decisions**
- **No chunked-summarisation test.** Long meetings that need an accurate summary go to an
  off-machine model, chosen per meeting. Transcription stays local.
- **Transcription stays on Soniqo Parakeet.** oMLX would run the same or a weaker model
  (Whisper), and its OpenAI transcription endpoint has no diarization, so speaker labels
  would be lost. oMLX speech stays an Unscheduled consolidation idea.
- **Thinking stays on by default;** no fork change for it.
- **Default summary model: Qwen 3.5 on oMLX,** which now runs as an always-on service.
  Apple's model is the fallback when oMLX is down (manual switch; the app has no
  automatic fallback).

**Ruled out**
- Chunk-and-merge (in-app was already out on 2026-11-05; the proxy idea was dropped too).

**Session close (2026-11-07)**
- Committed TODO, build notes, this file and the CLAUDE.md brief (`b8efc8e9e0`), then the
  Qwen 3.5 default and the T-BUILD-09 close as follow-up commits. Pushed to origin.

## 2026-11-10 to 2026-11-12 — Summary formats and prompt testing

**Done**
- **Mapped the summary selections.**
  - **Auto** (default): rules in `crates/template-app/assets/enhance.format.md.jinja`;
    the in-app override is the `auto_summary_prompt` setting (Templates tab → Auto).
  - **Gallery templates** ("by Anarlog"): fetched at runtime from
    `anarlog.so/api/templates` (`shared/ui/resource-list/hooks.ts`); not in this repo.
  - **User templates** ("Mine"): local SQLite `templates` table, title + description +
    sections (`apps/desktop/src/templates/queries.ts`).
  - All share `enhance.system.md.jinja` / `enhance.user.md.jinja`; a template only adds
    a numbered section list.
- **Found three sources of bloat:**
  - the default format asks for "at least 3 detailed bullet points" per section;
  - with a template selected, the Auto override is skipped
    (`getFormatOverride`, `enhance-transform.ts`), so the default format still applies;
  - the length cap equals the transcript's own length (`crates/summary/src/lib.rs`).
- **Direct oMLX test.** The operator's five-section prompt on a short test transcript
  gave output longer than the input (1.08×) and passed on speech-to-text errors as fact.
  A three-section prompt (summary paragraph, to-dos, dates) gave about 0.25×, but still
  missed one stated decision.
- **Thinking off works per request** on oMLX via
  `chat_template_kwargs: {enable_thinking: false}`: 3 s instead of 43 s on the short
  transcript. Thinking was about 94% of output tokens.
- **Built a local-only harness,** `lab/summary-test/` (`run.sh`, `prompts/`, `runs/`):
  prompt × transcript × thinking on/off × N runs, logging size, tokens and time to
  `results.tsv`. Added `lab/` to `.git/info/exclude`. Smoke test passed. Full runs are
  the operator's, after a restart.

**Decisions**
- **`collateral/` is for operator input only.** Tooling Claude builds goes in `lab/`.
- **Short format via the Auto override,** not a user template, because only the
  override replaces the default "detailed bullets" rules. Not applied in the app yet.

**Session close (2026-11-12)**
- Committed this entry, TODO.md and the CLAUDE.md brief; pushed to origin.

## 2026-11-13 — Thinking Off and the default summary template

**Done**
- **T-BUILD-11 closed (fork change):** confirmed in the app by the operator. Reasoning effort → Off on
  the Custom provider sends `chat_template_kwargs: {enable_thinking: false}`
  (`src/ai/reasoning-effort.ts`, dropdown in `settings/ai/llm/select.tsx`). Trigger:
  with thinking on, short and medium prompts never returned on oMLX.
  - Custom only: hosted OpenAI-compatible APIs may reject the unknown field; LM Studio
    and Ollama untested. A saved "off" behaves and displays as Default elsewhere.
  - Plain English "Off" (no `<Trans>`), so the ~200 locale catalog files stay identical
    to upstream and merges stay clean.
  - `@ai-sdk/openai-compatible` 2.0.41 forwards unknown provider-option keys verbatim,
    so the snake_case key reaches the body as-is. A wire-level test asserts it.
  - Checks: typecheck clean, oxfmt/oxlint clean, full suite only the 3 known failures.
    eslint's `exhaustive-deps` error in `select.tsx` predates this change (upstream).
  - Rebuilt the debug bundle (frontend only) and re-signed with the identifier
    requirement.
- **T-BUILD-06 closed:** the operator has recorded many live webinars across several
  video-call platforms and audio devices, including back-to-back sessions; on-demand
  processing suits that.
- **Upstream review and sync:** `upstream/main` was 5 commits ahead (product-docs
  articles, CI action pinning #8185), no app code. Fast-forwarded `main`, merged into
  `macalla` (`f5b23d1bc8`), pushed both. Unmerged upstream branch
  `devin/…-advanced-pro-gates` (2026-11-10) would gate custom template authoring,
  scheduled recording and advanced Chat helpers behind Pro; watch note on the Sync
  upstream row in TODO.md.
- **Renaming a user template:** not in the Favorite/Duplicate/Delete menu. The title at
  the top of the editor pane (Mine → select template) is a borderless text field;
  autosaves.

**Decisions**
- **Default summary format is the operator's own simplified user template,** "010
  Actionable Meeting", set as default in the app. Supersedes the 2026-11-12 "short format via the Auto override"
  plan: Auto stays unchanged. Revisit when the bigger local models arrive.
- **T-BUILD-07 kept** as written (cloud transcription fallback), after weighing narrowing
  or dropping it.
- **Priorities after Track 1's core:** T-BUILD-07, then T-BUILD-10, T-PROJ-03,
  T-PROJ-04; then agree a Track 2.

**Session close (2026-11-13)**
- Commits `e9c0c8356c` (T-BUILD-11 code), `1b8cb83dd6`, `2f57e3972c` (notes), merge
  `f5b23d1bc8`, plus this close. All pushed to origin.
