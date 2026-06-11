---
name: ofm
description: OpenFoot Manager project skill — context for testing, implementing, and raising issues in the openfootmanager/openfootmanager repo.
---

# OpenFoot Manager

Tauri + React (frontend) + Rust (backend via `src-tauri`) football management game. Repo: `openfootmanager/openfootmanager`.

## Current status (as of 2026-06-11)

Building an automated functional test foundation (Luca doing QA for the upstream dev).

- **PR #195 OPEN, awaiting dev review** — `dv1sual:test/season-scenario-foundation` → upstream `develop`. Seeded world generation + invariant-based season scenario tests (Phase 0). Clean, test-only, no `.claude/`, no co-author trailer.
- **Branches:** `test/season-scenario-foundation` = the clean PR branch (off upstream develop, 4 files only). `feature/scenario-tests` = working branch with the `.claude/` skill + history (NEVER goes upstream).
- **Phase 0 done:** reproducible starting world + `full_season_holds_invariants` + reproducibility tests. All `ofm_core` tests green.
- **Phase 1 PROPOSED, blocked on dev sign-off:** full-season determinism via a `seed: u64` on `Game` + per-turn RNG from `(seed, date)`, threaded through the engine + ~44 turn-pipeline `rand::rng()` sites (engine first). Do NOT start until the dev blesses the "RNG on `Game`" direction. A tracking issue is on hold until the dev replies to #195.
- **Next action:** wait for dev's answer on PR #195, then decide Phase 1 / tracking issue.

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

- PRs target the `develop` branch via fork & pull. Origin is the `dv1sual` fork; upstream is `openfootmanager/openfootmanager`.
- Reference the related issue in the PR description.
- If there's no issue for a new feature, open one first.
- Open the PR off a branch cut from **current upstream `develop`** (it moves), not from a stale local base. Confirm upstream has not changed the files you touched.

**Commit messages:**
- **Never** add a `Co-Authored-By` trailer (no Claude/AI co-author lines). The author is Luca only.
- Imperative subject, conventional prefix where it fits (`test:`, `feat:`, `fix:`).

**Privacy — keep AI tooling out of the public repo:**
- The `.claude/` directory (this skill file included) must **NEVER** reach `openfootmanager/openfootmanager`. It lives on local/fork branches only.
- When preparing an upstream PR, branch off upstream `develop` and bring over **only** the relevant source/test files — never `.claude/`. Verify with `git status` before committing.

**Before opening a PR:**
- Rust: `cargo fmt` and `cargo clippy`. Caveat: the committed repo is **not** fully fmt-clean under recent rustfmt (e.g. `contracts.rs` import ordering), and the crate already carries ~40 pre-existing clippy warnings. Only format the files you changed and aim for **no new** warnings; do not sweep unrelated reformatting into your PR (revert stray `cargo fmt` edits to files you didn't touch).
- Frontend: `npm test`
- Backend: `cd src-tauri && cargo test --workspace`

**Code conventions:**
- Rust: descriptive names, strong types, docstrings on public functions
- Frontend: modular components, TailwindCSS for styling, no `any` types

## Automated test foundation (scenario tests)

Functional/end-to-end coverage lives at the `ofm_core` layer, not the UI. The command layer is thin plumbing, so testing `ofm_core` directly exercises the real game logic. UI E2E (tauri-driver) is a thin future smoke layer, not the primary coverage.

Key file: `src-tauri/crates/ofm_core/tests/scenario_tests.rs`.

- **Seeded worlds:** `generate_world_data_seeded(seed)` / `generate_world_seeded(seed)` give a reproducible starting world. The shipped "New Game" path still uses entropy (`generate_world_data`). `make_scenario_game(seed)` is the shared fixture builder.
- **Determinism status:** the *starting world* is reproducible; the *season trajectory* is NOT yet. The match engine (`crates/engine/src/engine/mod.rs`) and ~44 turn-pipeline sites still call `rand::rng()` (ambient). So scenario assertions must be **outcome-independent invariants**, never exact post-season values.
- **Why HashMap order mattered:** name pools are a `HashMap`; sorting `country_codes` (`sorted_country_codes`) was required for seeded gen to be reproducible. Watch for other HashMap-iteration-order dependencies.
- **Path to full determinism (not done):** put a `seed: u64` on `Game`, derive a per-turn RNG from `(seed, date)`, and thread `&mut rng` through the engine + turn subsystems instead of `rand::rng()`. This keeps save/load trivial (persist one u64). Get maintainer sign-off before doing this — it's a cross-crate change.
- Invariant helpers to reuse/extend: `assert_game_invariants` (referential integrity, 3-1-0 points maths, goals-for == goals-against, finished fixtures carry results, finances in range).

## Testing a PR

1. Check out the branch / confirm it's merged into `develop`.
2. Run `npm run tauri dev` and wait for the window to open.
3. For each feature in the PR, find the shortest path to exercise it in the running app.
4. Report findings inline as you go — PASS, FAIL, or a named bug.
