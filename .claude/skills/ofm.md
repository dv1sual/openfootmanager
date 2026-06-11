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

## UI design direction (Luca's standing preference — apply by default, no need to ask)

Luca's bar for UI is **even, clean, neat, consistent**. Default to this for every frontend change:

- **Even spacing above all.** One uniform gap between repeated elements (attribute rows, table rows, stat boxes). Do NOT distribute / `space-between` to fill height — prefer a fixed even gap and accept empty space at the bottom of shorter blocks over uneven gaps.
- **Equal-size, aligned cards.** Cards side by side are the same height (`sm:auto-rows-fr`) and aligned. Keep a consistent column count (e.g. the player attributes grid is always 2 columns, even for goalkeepers).
- **Uniform page of cards.** Wide/tabular cards (stat strips, tables) span full width. Below the header row, stack data cards full-width in a single-column grid so the page reads as a uniform stack. NEVER leave a stray `col-span-*` on a card in a single-column grid — it spawns an implicit column and makes sibling cards unequal widths (this exact bug made one card narrower than the rest).
- **Align content to its header.** In a table, header and body cells share the same alignment (default left). Never mix left/right alignment within one table.
- **Even column widths.** Give a table's data columns equal widths so the spacing between them is uniform; a name/label column may take the remainder.
- **No jitter, no magic.** Nothing that shifts on re-render (no `Math.random()` for widths — derive a stable value). Document any arbitrary value (`max-w-[196px]`) with a short comment.
- **Responsive.** Grids adapt by breakpoint (`grid-cols-1 sm:grid-cols-2 ...`).

When unsure: tighter, more aligned, more uniform. Run the app (`npm run tauri dev`) and get visual sign-off rather than assuming.

## i18n (hard rule)

- Every user-facing string comes from `t("...")`. Never hardcode text, and never ship a key with no translation — a missing key renders the raw key (e.g. `common.clear`) in the UI, which is a visible bug.
- When adding a key, add it to ALL locale files in `src/i18n/locales/` (en, de, es, fr, it, pt, pt-BR, ru, zh-CN), actually translated, not copied from English.
- Unit suffixes follow the terse style of existing keys (`finances.perWeekSuffix = "/wk"`, `playerProfile.yearsSuffix = "y"`).
- Tests often mock `react-i18next` per file — when you rename or add a key used in a tested component, update that test's mock too.

## Testing a PR

1. Check out the branch / confirm it's merged into `develop`.
2. Run `npm run tauri dev` and wait for the window to open.
3. For each feature in the PR, find the shortest path to exercise it in the running app.
4. Report findings inline as you go — PASS, FAIL, or a named bug.
