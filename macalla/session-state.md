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
