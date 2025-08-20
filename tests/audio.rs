use std::path::PathBuf;

use cucumber::{World, given, then, when};
use escaping_the_depths::stream_logic::background_music::BackgroundPlayer;

#[derive(Debug, World, Default)]
pub struct AudioTesting {
    audio_folder: PathBuf,
    background_player: BackgroundPlayer,

    picked_song: PathBuf,
}

#[given(regex = r"a song directory '([a-zA-Z0-9-_]+)',")]
fn given_song_directory(audio_testing: &mut AudioTesting, song_directory: String) {
    let mut project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut audio_root_folder = PathBuf::from("tests/assets/");
    audio_root_folder.push(song_directory);

    project_root.push(audio_root_folder);
    audio_testing.audio_folder = project_root;
}

#[when("the background player loads the songs from the directory,")]
fn when_background_player_loads_songs(audio_testing: &mut AudioTesting) {
    audio_testing.background_player = BackgroundPlayer::new(&audio_testing.audio_folder);
}

#[when(regex = r"song ([0-9]+) is picked from the background player,")]
fn when_song_picked(audio_testing: &mut AudioTesting, song_chosen: usize) {
    let picked_song = audio_testing.background_player.pick(song_chosen);
    audio_testing.picked_song = picked_song;
}

#[then(regex = r"the song '(.+)' is found in the background songs.")]
fn verify_song_exists(audio_testing: &mut AudioTesting, expected_background_song_filename: String) {
    let expected_background_song_found = audio_testing
        .background_player
        .contains_song(expected_background_song_filename);
    assert!(expected_background_song_found);
}

#[then(regex = r"the picked song should be '(.+)'.")]
fn verify_picked_song(
    audio_testing: &mut AudioTesting,
    expected_background_song_filepath: PathBuf,
) {
    let actual_background_song_filepath = audio_testing.picked_song.clone();
    assert_eq!(
        expected_background_song_filepath,
        actual_background_song_filepath
    );
}

fn main() {
    let mut feature_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    feature_path.push("tests/features/audio.feature");

    futures::executor::block_on(AudioTesting::run(feature_path));
}
