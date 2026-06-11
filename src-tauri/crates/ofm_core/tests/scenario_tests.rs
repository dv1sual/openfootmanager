use chrono::{TimeZone, Utc};
use domain::manager::Manager;
use ofm_core::clock::GameClock;
use ofm_core::game::Game;
use ofm_core::generator::{generate_world_data, repair_opening_youth_academies};
use ofm_core::turn;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a fully playable game from generated world data, with the manager
/// assigned to the first team in the world. Starts on 2026-07-01 (season start).
fn make_scenario_game() -> Game {
    let world = generate_world_data(None);

    let start = Utc.with_ymd_and_hms(2026, 7, 1, 0, 0, 0).unwrap();
    let clock = GameClock::new(start);

    let first_team = world
        .teams
        .first()
        .expect("generated world must have at least one team");

    let mut manager = Manager::new(
        "scenario-mgr".to_string(),
        "Scenario".to_string(),
        "Manager".to_string(),
        "1980-01-01".to_string(),
        "England".to_string(),
    );
    manager.hire(first_team.id.clone());

    let team_ids: Vec<String> = world.teams.iter().map(|t| t.id.clone()).collect();

    let mut game = Game::new(
        clock,
        manager,
        world.teams,
        world.players,
        world.staff,
        vec![],
    );

    game.available_staff_market_last_activity_date =
        Some(start.format("%Y-%m-%d").to_string());

    repair_opening_youth_academies(&mut game);

    game.league = Some(ofm_core::schedule::generate_league(
        "Scenario League",
        2026,
        &team_ids,
        start,
    ));

    ofm_core::season_context::refresh_game_context(&mut game);

    game
}

fn staff_market_last_rotation(game: &Game) -> Option<String> {
    game.available_staff_market_last_activity_date.clone()
}

// ---------------------------------------------------------------------------
// Scenarios
// ---------------------------------------------------------------------------

#[test]
fn full_season_completes_without_panic() {
    let mut game = make_scenario_game();

    for day in 0..365 {
        // If this panics, the day number in the error message will show where it broke.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            turn::process_day(&mut game);
        }))
        .unwrap_or_else(|_| panic!("process_day panicked on day {day}"));
    }
}

#[test]
fn staff_market_does_not_rotate_before_30_days() {
    let mut game = make_scenario_game();
    let initial_date = staff_market_last_rotation(&game);

    for _ in 0..29 {
        turn::process_day(&mut game);
    }

    assert_eq!(
        staff_market_last_rotation(&game),
        initial_date,
        "staff market should not rotate before 30 days have passed"
    );
}

#[test]
fn staff_market_rotates_after_30_days() {
    let mut game = make_scenario_game();
    let initial_date = staff_market_last_rotation(&game);

    // process_available_staff_market runs before clock.advance_days each turn.
    // The Nth call processes date start + (N-1) days, so the diff reaches 30
    // on call 31 (start + 30 days). We need 31 iterations, not 30.
    for _ in 0..31 {
        turn::process_day(&mut game);
    }

    assert_ne!(
        staff_market_last_rotation(&game),
        initial_date,
        "staff market should have rotated after 30 days"
    );
}
