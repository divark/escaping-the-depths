use std::path::PathBuf;

use cucumber::{World, given, then, when};

mod mock_game;
use mock_game::*;

use surviving_the_trip::core_logic::setting::{ChangeMap, WorldTileDimensions};

#[given(regex = r"a campsite map called '(.+)',")]
fn given_campsite_map_file(game: &mut MockGame, desired_map: String) {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let map_root = PathBuf::from("tests/assets/maps/");

    let mut tiled_map_path = PathBuf::new();
    tiled_map_path.push(project_root);
    tiled_map_path.push(map_root);
    tiled_map_path.push(desired_map);
    game.tiled_map_path = tiled_map_path;
}

#[when("the campsite map is rendered,")]
fn load_campsite_map(game: &mut MockGame) {
    let tiled_map_path = game.tiled_map_path.clone();
    let spawn_map_request = ChangeMap::new(tiled_map_path);
    game.broadcast(spawn_map_request);
    game.tick();
}

#[then(regex = r"the size of the map should be (\d+) by (\d+).")]
fn verify_size_of_map(game: &mut MockGame, expected_width: usize, expected_height: usize) {
    let expected_map_size = WorldTileDimensions::new(expected_width, expected_height);
    let actual_map_size = *game.get_one::<WorldTileDimensions>();
    assert_eq!(expected_map_size, actual_map_size);
}

fn main() {
    let mut feature_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    feature_path.push("tests/features/surviving-the-trip-mvp.feature");

    futures::executor::block_on(MockGame::run(feature_path));
}
