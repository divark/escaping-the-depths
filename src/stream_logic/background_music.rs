use std::path::{Path, PathBuf};

use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct BackgroundPlayer {
    song_root: PathBuf,
    songs_found: Vec<PathBuf>,
}

/// Returns a list of files found in a depth-first fashion, excluding directories.
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

    found_files
}

impl BackgroundPlayer {
    pub fn new(song_root_folder: &Path) -> Self {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut song_folder = project_root;
        song_folder.push(song_root_folder);

        let found_song_root = song_root_folder
            .components()
            .next_back()
            .expect("Background Player: Could not find last component from given song path")
            .as_os_str();
        let song_root = PathBuf::from(found_song_root);
        let songs_found = explore_all_files(&song_folder);

        Self {
            song_root,
            songs_found,
        }
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

        found_song
    }
}
