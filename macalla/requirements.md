# Macalla — Requirements

> Tasks and open questions live in [TODO.md](../TODO.md), not here. This doc is the spec
> the build is checked against. Project context is in [CLAUDE.md](../CLAUDE.md).

## Project-wide constraints
- **Public repo.** Nothing personal or work-related in any committed file or commit
  message, including names of video-conferencing products.
- **Upstream files stay byte-identical** unless a deliberate code change is being made.
  Project tooling is additive (`CLAUDE.md`, `TODO.md`, `macalla/`) and never deletes or
  edits upstream's AI config, `.gitignore` or `docs/`.
- **Branch model:** `main` mirrors upstream (fast-forward only); `macalla` is the default
  and working branch; upstream PR branches are cut from `main`.
- **Platform:** Apple Silicon Mac, macOS 15 or later (on-device transcription needs both).

## Track 1 — Own Build Recording a Webinar (BUILD)

### Definition of done
1. The desktop app builds from this fork's source and launches on the Mac.
2. It captures both the microphone and system audio during a session.
3. It transcribes on the local GPU, with no meeting bot.
4. A third-party transcription API is configured as the fallback and confirmed working.
5. A recorded webinar played from YouTube is recorded and transcribed end to end.
6. A live webinar is recorded and transcribed end to end.

Summaries, code changes and upstream contributions are not part of done (see TODO.md →
Unscheduled).

### Build prerequisites (pinned by upstream)
| Tool | Required | Source of the pin |
|---|---|---|
| Rust | 1.94.0, via rustup | `rust-toolchain.toml` |
| Node | 22+ (CI runs 22) | `CONTRIBUTING.md`, CI workflows |
| pnpm | 11.1.1 | `package.json#packageManager` |
| process-compose | 1.122.0+ | `CONTRIBUTING.md` |
| Xcode | Full app plus Metal Toolchain component | `CONTRIBUTING.md` (Soniqo/MLX shaders) |
| Tauri 2 system deps | macOS set | v2.tauri.app/start/prerequisites |

- No paid Apple developer account is needed to build and run locally. Signing only
  matters for distributing builds.
- Checkout path must not contain spaces (some native deps use Autotools).
- Rust builds and transcription models use a lot of disk space.

### Build and run
- Install: `pnpm install --frozen-lockfile`
- Desktop dev: `pnpm dev:desktop` (process-compose manages processes; shared UI builds
  through Turbo first). Needs no secrets for local-first use.
- Upstream's CI workflows (`.github/workflows/desktop_ci.yaml`) are the source of truth
  for checks, if code is ever changed.

### Transcription
- **Primary: the local GPU.** Two upstream paths run on Metal: **Soniqo** (MLX, downloaded
  in-app; needs the Xcode Metal Toolchain to build) and **On-device file** (a whisper.cpp
  `.bin` you supply). Apple Speech is also on-device but isn't a GPU path, so it isn't
  the primary.
- **Fallback: a third-party transcription API**, via upstream's bring-your-own-key
  providers (Settings → Transcription). API keys stay in the app's local settings and
  never go in the repo. A hosted provider receives the meeting audio.
- Models are labeled **Live** (text during the meeting) or **After recording**.
- One provider is selected at a time. Upstream's docs describe no automatic failover.
- Summaries can be routed to a local model (Ollama / LM Studio). Out of scope for done.

### Test plan
- **First test:** a recorded webinar on YouTube. It uses the same system-audio path as
  a live call, so it's repeatable and needs no scheduling.
- **Final check:** a live webinar.
- Record for each run: engine used, whether both audio sources appear in the transcript,
  rough accuracy, and any permission prompts encountered. Findings go in
  `macalla/session-state.md`.

### Upstream observations
- **Repo:** fastrepl/anarlog, a pnpm + Cargo monorepo. The desktop app is
  `apps/desktop/` (Tauri 2, React, TypeScript, Vite, Tailwind; Zustand + TanStack).
  Also `apps/web`, `apps/mobile`, `apps/api`, `apps/cli`, plus `crates/`, `plugins/` and
  `packages/`. Only the desktop app is in scope.
- **Licensing:** MIT overall, but `enterprise/` is a separate Cargo workspace under a
  commercial license (`LICENSE.enterprise`). Community packages must not depend on it.
  Nothing in Track 1 touches it.
- **Agent config:** upstream ships `AGENTS.md` (root and per-component), `.claude/`,
  `.agents/`, `.codex/`, `.cursor/` and `.mcp.json`. CLAUDE.md → *Upstream's AI Files*
  says what's used, ignored or declined.
- **Product docs:** docs.anarlog.so (the source is upstream's `docs/` folder, a Mintlify
  site).
