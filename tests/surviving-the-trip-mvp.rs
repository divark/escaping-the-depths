use std::path::PathBuf;

use cucumber::{World, given, then, when};

mod mock_game;
use mock_game::*;

fn main() {
    let mut feature_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    feature_path.push("tests/features/surviving-the-trip-mvp.feature");

    futures::executor::block_on(MockGame::run(feature_path));
}
