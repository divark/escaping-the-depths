use std::path::PathBuf;

use cucumber::{World, given, then, when};

mod mock_game;
use mock_game::*;

use surviving_the_trip::core_logic::{
    CampersState,
    progressing::HungerBar,
    setting::{ChangeMap, WorldTileDimensions},
};

/// Returns a CampersState parsed from an expected string in the form of
/// "alive" or "dead".
fn parse_campers_state(camper_state_string: String) -> CampersState {
    match camper_state_string.as_str() {
        "alive" => CampersState::Alive,
        "dead" => CampersState::Dead,
        _ => panic!("parse_campers_state: Invalid camper state provided."),
    }
}

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

#[given(regex = r"a hunger bar set to (\d+)% full,")]
fn given_initial_hunger_bar(game: &mut MockGame, initial_percentage: usize) {
    let mut hunger_bar = game.get_resource_mut::<HungerBar>();
    hunger_bar.set_percentage(initial_percentage);
}

#[given(regex = r"the hunger bar decreases by (\d+)% every second,")]
fn given_hunger_bar_decrease_rate(game: &mut MockGame, percentage_to_decrease: usize) {
    let mut hunger_bar = game.get_resource_mut::<HungerBar>();
    hunger_bar.set_percentage_decrease(percentage_to_decrease);
}

#[when("the campsite map is rendered,")]
fn load_campsite_map(game: &mut MockGame) {
    let tiled_map_path = game.tiled_map_path.clone();
    let spawn_map_request = ChangeMap::new(tiled_map_path);
    game.broadcast(spawn_map_request);
    game.tick();
}

#[when(regex = r"(\d+) seconds have passed,")]
fn tick_per_second(game: &mut MockGame, seconds_to_pass: usize) {
    for _i in 0..seconds_to_pass {
        game.tick();
    }
}

#[then(regex = r"the size of the map should be (\d+) by (\d+) by (\d+).")]
fn verify_size_of_map(
    game: &mut MockGame,
    expected_width: usize,
    expected_height: usize,
    expected_depth: usize,
) {
    let expected_map_size =
        WorldTileDimensions::new(expected_width, expected_height, expected_depth);
    let actual_map_size = *game.get_one::<WorldTileDimensions>();
    assert_eq!(expected_map_size, actual_map_size);
}

#[then(regex = r"the hunger bar should be at (\d+)%.")]
fn verify_hunger_bar_current_percentage(game: &mut MockGame, expected_current_percentage: usize) {
    let hunger_bar = game.get_resource::<HungerBar>();
    let actual_current_percentage = hunger_bar.get_current_percentage();
    assert_eq!(expected_current_percentage, actual_current_percentage);
}

#[then(regex = r"all campers should be ([a-zA-Z ]+).")]
fn verify_camper_state(game: &mut MockGame, expected_campers_state_string: String) {
    // The game runs systems concurrently, in any order. Because of that,
    // we ask the game to advance by one frame to ensure it detected that
    // the hunger bar reached zero.
    game.tick();

    let expected_campers_state = parse_campers_state(expected_campers_state_string);
    let actual_campers_state = *game.get_game_state::<CampersState>().get();
    assert_eq!(expected_campers_state, actual_campers_state);
}

fn main() {
    let mut feature_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    feature_path.push("tests/features/surviving-the-trip-mvp.feature");

    futures::executor::block_on(MockGame::run(feature_path));
}
