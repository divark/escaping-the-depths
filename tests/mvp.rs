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

#[when(regex = r"the explorer is on Tile ([0-9]+), ([0-9]+),")]
fn move_explorer_to_tile(game: &mut MockGame, desired_x: usize, desired_y: usize) {
    game.move_explorer_to(desired_x, desired_y);
}

#[then(regex = r"the exit door at Tile ([0-9]+), ([0-9]+) will be opened.")]
fn verify_exit_door_opened(game: &mut MockGame, exit_door_x: usize, exit_door_y: usize) {
    let expected_door_state = ExitDoorState::Open;
    let actual_door_state = game.get_door_state(exit_door_x, exit_door_y);
    assert_eq!(expected_door_state, actual_door_state);
}

fn main() {
    futures::executor::block_on(MockGame::run("tests/features/mvp.feature"));
}
