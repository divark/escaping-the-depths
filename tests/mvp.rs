use std::path::PathBuf;

use cucumber::{World, given, then, when};

mod mock_game;
use mock_game::*;

use escaping_the_depths::*;

#[given(regex = r"a ([0-9]+)x([0-9]+) cave room,")]
fn make_cave_room(game: &mut MockGame, width: usize, height: usize) {
    let mut room_generator = TestRoomGenerator::new(width, height);

    let generated_room = room_generator.generate();
    game.spawn_room(generated_room);
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

#[when(regex = r"the explorer is on Tile ([0-9]+), ([0-9]+),")]
fn move_explorer_to_tile(game: &mut MockGame, desired_x: usize, desired_y: usize) {
    game.place(RoomObject::Explorer, desired_x, desired_y);
}

#[when(regex = r"a viewer clicks with UV coordinates ([0-9]+.[0-9]+), ([0-9]+.[0-9]+),")]
fn simulate_click(game: &mut MockGame, uv_x: f32, uv_y: f32) {
    game.click(uv_x, uv_y);
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

fn main() {
    let mut feature_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    feature_path.push("tests/features/mvp.feature");

    futures::executor::block_on(MockGame::run(feature_path));
}
