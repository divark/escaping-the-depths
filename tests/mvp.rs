use std::path::PathBuf;

use cucumber::{World, given, then, when};

mod mock_game;
use mock_game::*;

use escaping_the_depths::{game_logic::room_generating::ExplorerState, *};

#[given(regex = r"a ([0-9]+)x([0-9]+) cave room,")]
fn make_cave_room(game: &mut MockGame, width: usize, height: usize) {
    game.spawn_room(width, height);
}

#[given(regex = r"[an?|the] (.+) placed at coordinates ([0-9]+), ([0-9]+),")]
fn place_object(game: &mut MockGame, object_name: String, object_x: usize, object_y: usize) {
    let object_type = parse_object_type(object_name);
    game.place(object_type, object_x, object_y);
}

#[given(
    regex = r"some piece of treasure worth ([0-9]+) points placed on coordinates ([0-9]+), ([0-9]+),"
)]
fn place_treasure(
    game: &mut MockGame,
    treasure_point_value: usize,
    treasure_x: usize,
    treasure_y: usize,
) {
    let treasure = RoomObject::Treasure(treasure_point_value);
    game.place(treasure, treasure_x, treasure_y);
}

#[given(regex = r"the explorer's initial health set to ([0-9]+) out of ([0-9]+),")]
fn set_explorer_health(game: &mut MockGame, current_health: usize, total_health: usize) {
    game.set_explorer_health(current_health, total_health);
}

#[when(regex = r"the explorer is on Tile ([0-9]+), ([0-9]+),")]
fn move_explorer_to_tile(game: &mut MockGame, desired_x: usize, desired_y: usize) {
    game.place(RoomObject::Explorer, desired_x, desired_y);
}

#[when(regex = r"a viewer clicks with UV coordinates ([0-9]+.[0-9]+), ([0-9]+.[0-9]+),")]
fn simulate_click(game: &mut MockGame, uv_x: f32, uv_y: f32) {
    game.click(uv_x, uv_y);
}

#[when("the explorer has finished exiting,")]
fn explorer_finished_exiting(game: &mut MockGame) {
    game.wait_for_explorer_to_finish_exiting();
}

#[when("the explorer has left the room,")]
fn explorer_left_the_room(game: &mut MockGame) {
    game.wait_for_explorer_to_wander_again();
}

#[when(regex = r"the explorer has reached Tile ([0-9]+), ([0-9]+),")]
fn wait_until_explorer_reached_tile(game: &mut MockGame, desired_x: usize, desired_y: usize) {
    let expected_tile_location = LogicalCoordinates::new(desired_x, desired_y);
    game.wait_for_explorer_to_reach(expected_tile_location);
}

#[when("the game over timer has elapsed,")]
fn wait_for_game_over_time_to_finish(game: &mut MockGame) {
    game.wait_for_game_over_timer_to_finish();
}

#[then("the exit door will be opened.")]
fn verify_exit_door_opened(game: &mut MockGame) {
    let expected_door_state = ExitDoorState::Open;
    let actual_door_state = game.get_door_state();
    assert_eq!(expected_door_state, actual_door_state);
}

#[then(regex = r"the current score will be ([0-9]+) points.")]
fn verify_current_score(game: &mut MockGame, expected_current_score: usize) {
    let actual_current_score = game.get_current_score();
    assert_eq!(expected_current_score, actual_current_score);
}

#[then(regex = r"the trap at Tile ([0-9]+), ([0-9]+) will be disarmed.")]
fn verify_specific_trap_disarmed(game: &mut MockGame, tile_x: usize, tile_y: usize) {
    let expected_trap_state = TrapState::Unarmed;
    let actual_trap_state = game.get_trap_at(tile_x, tile_y);
    assert_eq!(expected_trap_state, actual_trap_state);
}

#[then(regex = r"the explorer's goal is to reach Tile ([0-9]+), ([0-9]+).")]
fn verify_explorer_target(game: &mut MockGame, expected_tile_x: usize, expected_tile_y: usize) {
    let expected_target_position = LogicalCoordinates::new(expected_tile_x, expected_tile_y);
    let actual_target_position = game.get_explorer_destination_overall();
    assert_eq!(expected_target_position, actual_target_position);
}

#[then(regex = r"the explorer's health should be ([0-9]+) out of ([0-9]+).")]
fn verify_explorer_health(
    game: &mut MockGame,
    expected_current_health: usize,
    expected_total_health: usize,
) {
    let actual_health = game.get_explorer_health();

    let actual_current_health = actual_health.get_current_health();
    assert_eq!(
        expected_current_health, actual_current_health,
        "Current health mismatch"
    );

    let actual_total_health = actual_health.get_total_health();
    assert_eq!(
        expected_total_health, actual_total_health,
        "Total health mismatch"
    );
}

#[then("the explorer will be passed out.")]
fn verify_explorer_passed_out(game: &mut MockGame) {
    let expected_explorer_state = ExplorerState::Dead;
    let actual_explorer_state = game.get_explorer_state();
    assert_eq!(expected_explorer_state, actual_explorer_state);
}

#[then(regex = r"the explorer should be on Tile ([0-9]+), ([0-9]+).")]
fn verify_explorer_position(game: &mut MockGame, expected_tile_x: usize, expected_tile_y: usize) {
    let expected_position = LogicalCoordinates::new(expected_tile_x, expected_tile_y);
    let actual_position = game.get_explorer_position();
    assert_eq!(expected_position, actual_position);
}

#[then("the explorer should be visiting all other tiles in the room.")]
fn verify_explorer_visiting_all_other_tiles(game: &mut MockGame) {
    let expected_tiles_to_visit = game.get_all_tiles();
    let actual_tiles_to_visit = game.get_explorer_tiles_to_be_visited();
    assert_eq!(expected_tiles_to_visit, actual_tiles_to_visit);
}

#[then(regex = r"the current room count should be ([0-9]+).")]
fn verify_current_room_number(game: &mut MockGame, expected_room_number: usize) {
    let actual_room_number = game.get_current_room_number();
    assert_eq!(expected_room_number, actual_room_number);
}

fn main() {
    let mut feature_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    feature_path.push("tests/features/mvp.feature");

    futures::executor::block_on(MockGame::run(feature_path));
}
