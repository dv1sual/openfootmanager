---
name: ofm
description: OpenFoot Manager project skill — context for testing, implementing, and raising issues in the openfootmanager/openfootmanager repo.
---

# OpenFoot Manager

Tauri + React (frontend) + Rust (backend via `src-tauri`) football management game. Repo: `openfootmanager/openfootmanager`.

This is the single master skill. Keep it that way: when working on any branch, fold new guidelines into the right section here rather than starting a divergent copy.

## Work status (keep this current)

Living worklog — the source of truth for where each workstream stands (branches, PRs, issues, blockers). Update it whenever something opens, lands, or gets blocked. Convert relative dates to absolute. Last updated **2026-06-12**.

### Test foundation (QA for the upstream dev)

Building an automated functional test foundation.

- **PR #195 MERGED** (2026-06-12, merge commit `158b9dc`) into upstream `develop`. Seeded world generation + invariant-based season scenario tests (Phase 0). Issue **#196 auto-closed**. Phase 0 coverage is now upstream.
- **Branches:** `test/season-scenario-foundation` = the merged PR branch (can be deleted now). `feature/scenario-tests` = working branch with the `.claude/` skill + history (NEVER goes upstream).
- **Phase 0 done & merged:** reproducible starting world + `full_season_holds_invariants` + reproducibility tests. All `ofm_core` tests green.
- **Phase 1 PROPOSED, still blocked on dev sign-off:** full-season determinism via a `seed: u64` on `Game` + per-turn RNG from `(seed, date)`, threaded through the engine + ~44 turn-pipeline `rand::rng()` sites (engine first). The #195 merge does NOT bless this direction; do NOT start until the dev explicitly approves "RNG on `Game`". Tracking issue still to be opened once the direction is confirmed.
- **Next action:** the "Planned next tests" below are now UNBLOCKED (#195 landed) — pick one (start with Tier 1 #1/#2), branch off current upstream `develop`, rebase onto the merged `make_scenario_game` fixture. Separately, get the dev's call on the Phase 1 RNG direction.

### UI refinements

- **Branch `feature/ui-refinements`** (on the `dv1sual` fork, not upstream). Working branch for UI polish; no upstream PR yet.
- **Done & pushed:** player career history table now right-aligns the stat columns (header + body together) so values sit flush to the right edge with even spacing (commit `a108845`). Visually signed off by Luca.
- **Note:** this branch still carries the older divergent copy of this skill in history; the unified master now lives here.

### News overhaul (roadmap issue #11, 0.3.x-beta)

Decided to do a **UX review first** before any backend feature work (variety pass / rivalry news).

- **Issue #197 FILED** (upstream) — News team-filter dropdown shows the raw key `news.allTeams` instead of a readable label. Missing in all 9 locales. Reproduced with >1 team in the news. Quick fix: add the translated key everywhere. Not started.
- **BLOCKER for the empty-filter UX bug:** can't reproduce the "filter to an empty result shows a blank void" issue because news from the future is appearing (saw 6 July items while the game clock was 12 June). News does not look bounded by the current date. Needs its own investigation/ticket before the empty-state work can be tested. Not yet filed.
- **Backend feature work (variety pass, rivalry news, trash talk) is on hold** until the UX issues above are cleared.

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

## UI design direction (Luca's standing preference — apply by default, no need to ask)

Luca's bar for UI is **even, clean, neat, consistent**. Default to this for every frontend change:

- **Even spacing above all.** One uniform gap between repeated elements (attribute rows, table rows, stat boxes). Do NOT distribute / `space-between` to fill height — prefer a fixed even gap and accept empty space at the bottom of shorter blocks over uneven gaps.
- **Equal-size, aligned cards.** Cards side by side are the same height (`sm:auto-rows-fr`) and aligned. Keep a consistent column count (e.g. the player attributes grid is always 2 columns, even for goalkeepers).
- **Uniform page of cards.** Wide/tabular cards (stat strips, tables) span full width. Below the header row, stack data cards full-width in a single-column grid so the page reads as a uniform stack. NEVER leave a stray `col-span-*` on a card in a single-column grid — it spawns an implicit column and makes sibling cards unequal widths (this exact bug made one card narrower than the rest).
- **Align content to its header.** In a table, header and body cells share the same alignment. Keep a column's header and body cells aligned to each other; numeric columns may right-align (header and body together) so values run flush to the right edge with even spacing instead of leaving a dead gap before the border.
- **Even column widths.** Give a table's data columns equal widths so the spacing between them is uniform; a name/label column may take the remainder.
- **No jitter, no magic.** Nothing that shifts on re-render (no `Math.random()` for widths — derive a stable value). Document any arbitrary value (`max-w-[196px]`) with a short comment.
- **Responsive.** Grids adapt by breakpoint (`grid-cols-1 sm:grid-cols-2 ...`).

When unsure: tighter, more aligned, more uniform. Run the app (`npm run tauri dev`) and get visual sign-off rather than assuming.

## i18n (hard rule)

- Every user-facing string comes from `t("...")`. Never hardcode text, and never ship a key with no translation — a missing key renders the raw key (e.g. `common.clear`) in the UI, which is a visible bug.
- When adding a key, add it to ALL locale files in `src/i18n/locales/` (en, de, es, fr, it, pt, pt-BR, ru, zh-CN), actually translated, not copied from English.
- Unit suffixes follow the terse style of existing keys (`finances.perWeekSuffix = "/wk"`, `playerProfile.yearsSuffix = "y"`).
- Tests often mock `react-i18next` per file — when you rename or add a key used in a tested component, update that test's mock too.

## Automated test foundation (scenario tests)

Functional/end-to-end coverage lives at the `ofm_core` layer, not the UI. The command layer is thin plumbing, so testing `ofm_core` directly exercises the real game logic. UI E2E (tauri-driver) is a thin future smoke layer, not the primary coverage.

Key file: `src-tauri/crates/ofm_core/tests/scenario_tests.rs`.

- **Seeded worlds:** `generate_world_data_seeded(seed)` / `generate_world_seeded(seed)` give a reproducible starting world. The shipped "New Game" path still uses entropy (`generate_world_data`). `make_scenario_game(seed)` is the shared fixture builder.
- **Determinism status:** the *starting world* is reproducible; the *season trajectory* is NOT yet. The match engine (`crates/engine/src/engine/mod.rs`) and ~44 turn-pipeline sites still call `rand::rng()` (ambient). So scenario assertions must be **outcome-independent invariants**, never exact post-season values.
- **Why HashMap order mattered:** name pools are a `HashMap`; sorting `country_codes` (`sorted_country_codes`) was required for seeded gen to be reproducible. Watch for other HashMap-iteration-order dependencies.
- **Path to full determinism (not done):** put a `seed: u64` on `Game`, derive a per-turn RNG from `(seed, date)`, and thread `&mut rng` through the engine + turn subsystems instead of `rand::rng()`. This keeps save/load trivial (persist one u64). Get maintainer sign-off before doing this — it's a cross-crate change.
- Invariant helpers to reuse/extend: `assert_game_invariants` (referential integrity, 3-1-0 points maths, goals-for == goals-against, finished fixtures carry results, finances in range).

## Planned next tests (UNBLOCKED — #195 merged 2026-06-12)

Now clear to start. These all build on `make_scenario_game(seed)` + `assert_game_invariants` as merged into `develop`; branch off current upstream `develop` so you're on the final fixture. All are invariant-based (no determinism needed) and one self-contained PR each.

**Tier 1 (biggest gaps):**
1. **Multi-season rollover.** GAP: `process_end_of_season` is NOT in the daily `process_day` loop. It is gated by `end_of_season::is_season_complete(&game)` and orchestrated by the `advance_to_next_season` command (`src/commands/season.rs`). So the current full-season test plays all fixtures but never crosses a season boundary. New test: advance until `is_season_complete`, call `process_end_of_season`, then play into season 2. Assert: season number incremented; new fixtures all `Scheduled` (no stale results); standings reset; players aged one year (`apply_seasonal_aging`); retirements/youth intake keep referential integrity; invariants hold after N more days. Run 2 seasons.
2. **Save/load round-trip mid-season.** `Game` derives `Serialize`/`Deserialize`. Advance ~60 days, `serde_json` round-trip, assert invariants hold on the loaded game + key fields match (clock date, player count, standings, finances), then keep playing the loaded game for N days without panic.

**Tier 2 (targeted):**
3. **Squad/contract lifecycle integrity.** Extend the invariant lib: transfer-log entries reference real teams/players; transferred player's `team_id` == destination; expired contract never leaves a player both rostered and free; free agents (`team_id = None`) otherwise valid.
4. **Live-match-day path.** Exercise `create_live_match` + `finish_live_match_day` (the path used when the user plays their own match live; `process_day` skips it by auto-simming). Assert standings update for that match + invariants hold. No scenario coverage today.
5. **Injury recovery bounds.** Over a season: no injured player has absurd `days_remaining`; injuries progress to 0 and clear.

Suggested order: #1 and #2 first (largest holes, quick reviews), then #3-#5.

## Testing a PR

1. Check out the branch / confirm it's merged into `develop`.
2. Run `npm run tauri dev` and wait for the window to open.
3. For each feature in the PR, find the shortest path to exercise it in the running app.
4. Report findings inline as you go — PASS, FAIL, or a named bug.
