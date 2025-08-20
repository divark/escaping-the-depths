use std::path::{Path, PathBuf};

use bevy::{audio::PlaybackMode, prelude::*};
use rand::Rng;

#[derive(Resource, Default, Debug)]
pub struct BackgroundPlayer {
    songs_found: Vec<PathBuf>,
}

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Bundle)]
pub struct BackgroundMusicBundle {
    song_to_play: AudioPlayer,
    song_settings: PlaybackSettings,
    label: BackgroundMusic,
}

impl BackgroundMusicBundle {
    pub fn new(song_to_play_path: PathBuf, asset_server: &AssetServer) -> Self {
        let song_to_play = AudioPlayer::new(asset_server.load(song_to_play_path));
        let song_settings = PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        };

        Self {
            song_to_play,
            song_settings,
            label: BackgroundMusic,
        }
    }
}

/// Returns a sorted list of files found in a depth-first fashion, excluding directories.
fn explore_all_files(starting_directory: &Path) -> Vec<PathBuf> {
    let mut found_files = Vec::new();

    let mut files_to_explore = vec![starting_directory.to_path_buf()];
    while let Some(file_to_explore) = files_to_explore.pop() {
        if file_to_explore.is_file() {
            found_files.push(file_to_explore.to_path_buf());
            continue;
        }

        let new_files_to_explore = file_to_explore
            .read_dir()
            .expect("explore_all_files: Could not find files in directory");
        for new_file_to_explore in new_files_to_explore {
            files_to_explore.push(new_file_to_explore.unwrap().path());
        }
    }

    // Since my text editor shows files being sorted,
    // the code should do the same to avoid confusion.
    found_files.sort_by(|item1, item2| {
        let case_insensitive_item1 = item1.as_os_str().to_ascii_lowercase();
        let case_insensitive_item2 = item2.as_os_str().to_ascii_lowercase();

        case_insensitive_item1.cmp(&case_insensitive_item2)
    });

    found_files
}

/// Returns a PathBuf that excludes the assets directory from some path to be processed.
fn split_off_assets_from_path(mut path_to_process: PathBuf) -> PathBuf {
    let mut processed_path = PathBuf::new();

    let mut path_components_found = vec![path_to_process.file_name().unwrap().to_os_string()];

    let assets_path = PathBuf::from("assets");
    while let Some(path_parent) = path_to_process.parent() {
        let current_file = path_parent.file_name().unwrap().to_os_string();
        if current_file == assets_path.as_os_str() {
            break;
        }

        path_components_found.push(current_file);
        path_to_process.pop();
    }

    while let Some(path_component) = path_components_found.pop() {
        processed_path.push(path_component);
    }

    processed_path
}

impl BackgroundPlayer {
    pub fn new(song_root_folder: &Path) -> Self {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut song_folder = project_root;
        song_folder.push(song_root_folder);

        let songs_found = explore_all_files(&song_folder);

        Self { songs_found }
    }

    pub fn contains_song(&self, song_filename_to_find: String) -> bool {
        let found_song_filename = self
            .songs_found
            .iter()
            .map(|song_filepath| {
                song_filepath
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .any(|song_filename| song_filename == song_filename_to_find);

        found_song_filename
    }

    pub fn pick(&self, picked_song_idx: usize) -> PathBuf {
        let found_song = self
            .songs_found
            .get(picked_song_idx)
            .expect("pick: Could not find desired song at given idx.")
            .clone();

        split_off_assets_from_path(found_song)
    }

    pub fn pick_random(&self) -> PathBuf {
        let mut random_number_generator = rand::rng();
        let random_song_idx = random_number_generator.random_range(0..self.songs_found.len());

        self.pick(random_song_idx)
    }
}

pub fn play_background_music(
    background_music_player: Res<BackgroundPlayer>,
    music_playing: Query<Entity, With<BackgroundMusic>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if !music_playing.is_empty() {
        return;
    }

    let next_song_to_play = background_music_player.pick_random();
    let song_bundle = BackgroundMusicBundle::new(next_song_to_play, &asset_server);

    commands.spawn(song_bundle);
}
