use std::{collections::HashSet, path::PathBuf};

use cucumber::{World, given, then, when};
use escaping_the_depths::core_logic::setting::{
    LogicalCoordinates, WorldTileDimensions, exclude_tiles_at_and_around,
};

#[derive(World, Default, Debug)]
struct QOLEnvironment {
    player_location: LogicalCoordinates,
    room_dimensions: WorldTileDimensions,
    excluded_tiles: HashSet<LogicalCoordinates>,
}

#[given(regex = r"a player's location of ([0-9]+), ([0-9]+),")]
fn given_player_location(qol_environment: &mut QOLEnvironment, player_x: usize, player_y: usize) {
    qol_environment.player_location = LogicalCoordinates::new(player_x, player_y);
}

#[given(regex = r"a room size of ([0-9]+)x([0-9]+),")]
fn given_room_size(qol_environment: &mut QOLEnvironment, room_width: usize, room_height: usize) {
    qol_environment.room_dimensions = WorldTileDimensions::new(room_width, room_height);
}

#[when("the excluded locations are identified from the player's location,")]
fn calculate_excluded_locations(qol_environment: &mut QOLEnvironment) {
    let player_location = &qol_environment.player_location;
    let room_dimensions = &qol_environment.room_dimensions;
    qol_environment.excluded_tiles = exclude_tiles_at_and_around(player_location, room_dimensions);
}

#[then(regex = r"there should be ([0-9]+) excluded locations.")]
fn verify_num_excluded_locations(
    qol_environment: &mut QOLEnvironment,
    expected_num_excluded_locations: usize,
) {
    let actual_num_excluded_locations = qol_environment.excluded_tiles.len();
    assert_eq!(
        expected_num_excluded_locations,
        actual_num_excluded_locations
    );
}

#[then(regex = r"location ([0-9]+), ([0-9]+) should be marked as excluded.")]
fn verify_location_excluded(
    qol_environment: &mut QOLEnvironment,
    excluded_x: usize,
    excluded_y: usize,
) {
    let excluded_location = LogicalCoordinates::new(excluded_x, excluded_y);
    let location_excluded = qol_environment.excluded_tiles.contains(&excluded_location);
    assert!(location_excluded);
}

fn main() {
    let mut feature_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    feature_path.push("tests/features/qol.feature");

    futures::executor::block_on(QOLEnvironment::run(feature_path));
}
