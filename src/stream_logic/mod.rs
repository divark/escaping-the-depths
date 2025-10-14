pub mod background_music;
pub mod networking;

use std::{path::PathBuf, time::Duration};

use background_music::{BackgroundPlayer, play_background_music, play_game_over_song};
use bevy::{prelude::*, window::WindowResolution};

use crate::{
    core_logic::{CoreLogic, GameOverTime, GameState, MovementTime},
    stream_logic::networking::{TwitchClickListener, map_twitch_clicks_to_uv},
};

pub struct StreamLogic;

impl Plugin for StreamLogic {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280, 720)
                            .with_scale_factor_override(1.0),
                        decorations: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        // This section serves as preferences on how fast aspects like
        // moving entities and when to restart the game should happen.
        let movement_time = MovementTime::new(Duration::from_secs(1));
        let game_over_time = GameOverTime::new(Duration::from_secs(10));

        // This section deals with how rooms are created in the game as the
        // explorer navigates the depths.
        let core_logic = CoreLogic::new(movement_time, game_over_time);
        app.add_plugins(core_logic);

        app.insert_state(GameState::Start);

        // This section deals with all of the sounds and music heard during the game.
        let music_path = PathBuf::from("assets/background_music/");
        #[cfg(debug_assertions)]
        let background_music_path = {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push(music_path);

            path
        };
        #[cfg(not(debug_assertions))]
        let background_music_path = music_path;
        app.insert_resource(BackgroundPlayer::new(&background_music_path));

        let sound_and_music_systems = (play_background_music.run_if(in_state(GameState::Active)),);
        app.add_systems(Update, sound_and_music_systems);
        app.add_systems(
            OnEnter(GameState::GameOver),
            play_game_over_song.run_if(in_state(GameState::GameOver)),
        );

        // This section deals with how interactions are handled in the game
        // from outside sources, mainly from Twitch.

        app.insert_resource(TwitchClickListener::connect("103834034"));
        app.add_systems(Update, map_twitch_clicks_to_uv);
    }
}
