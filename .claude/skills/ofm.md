---
name: ofm
description: OpenFoot Manager project skill — context for testing, implementing, and raising issues in the openfootmanager/openfootmanager repo.
---

# OpenFoot Manager

Tauri + React (frontend) + Rust (backend via `src-tauri`) football management game. Repo: `openfootmanager/openfootmanager`.

## Running the app

```
npm run tauri dev
```

Starts Vite dev server on `localhost:1420` and compiles the Rust backend. The window title shows the current version (e.g. `Openfoot Manager v0.2.1-alpha`).

## Project structure

- `src/` — React frontend (components, pages, store, i18n)
- `src-tauri/` — Rust backend (Tauri commands, `ofm_core`, `db`, `domain`, `engine` crates)
- `src-tauri/crates/ofm_core/` — core game logic (contracts, transfers, training, etc.)
- `src-tauri/crates/db/` — SQLite persistence and migrations
- `src/components/` — reusable UI components
- `src/pages/` — top-level route pages

## Raising GitHub issues

Use `gh issue create --repo openfootmanager/openfootmanager`.

**Format** (based on #190 as canonical template):

```
## Description

One short paragraph. What is broken and where. No technical implementation details.

## Steps to reproduce

Numbered steps to reach the broken state.

## Current behaviour

What you see.

## Expected behaviour

What you should see.

## Seen on

OpenFoot Manager vX.X.X-alpha

## Screen

[image if any]
```

**Rules:**
- QA tone: describe what happens, not how to fix it. No code references, no component names, no "the fix is".
- No double dashes or em dashes as separators (no `--`, no `—`)
- No AI filler: "It is worth noting", "This ensures that", "It should be noted"
- Short sentences. One idea per sentence.

## Contributing

- PRs target the `develop` branch via fork & pull.
- Reference the related issue in the PR description.
- If there's no issue for a new feature, open one first.

**Before opening a PR:**
- Rust: `cargo fmt` and `cargo clippy` (no warnings)
- Frontend: `npm test`
- Backend: `cd src-tauri && cargo test --workspace`

**Code conventions:**
- Rust: descriptive names, strong types, docstrings on public functions
- Frontend: modular components, TailwindCSS for styling, no `any` types

## Testing a PR

1. Check out the branch / confirm it's merged into `develop`.
2. Run `npm run tauri dev` and wait for the window to open.
3. For each feature in the PR, find the shortest path to exercise it in the running app.
4. Report findings inline as you go — PASS, FAIL, or a named bug.
