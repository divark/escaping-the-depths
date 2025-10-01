pub mod animation;
pub mod background_music;
pub mod networking;
pub mod sfx;
pub mod ui;

use std::{path::PathBuf, time::Duration};

use animation::{animate_disarming_trap, animate_door_opening};
use background_music::{BackgroundPlayer, play_background_music, play_game_over_song};
use bevy::{prelude::*, window::WindowResolution};
use sfx::{
    trigger_door_opening_noise, trigger_trap_going_off_noise, trigger_treasure_claimed_noise,
};
use ui::{
    despawn_start_screen, prepare_screen_ui, spawn_game_over_screen, spawn_health_ui,
    spawn_start_screen, start_the_game_after_some_time, update_game_over_screen, update_health_ui,
};

use crate::{
    core_logic::{
        CoreLogic, GameOverTime, GameState, MovementTime,
        interacting::ViewerClick,
        setting::{
            ChangeRoom, LogicalCoordinates, RandomizedRoomGenerator, RoomGenerating, TileSize,
            WorldTileDimensions, calculate_max_size,
        },
    },
    stream_logic::{
        networking::{TwitchClickListener, map_twitch_clicks_to_uv},
        sfx::trigger_bonus_score_noise,
        ui::{
            TemporaryUITime, hide_bonus_scores_after_time, show_bonus_scores_on_exit,
            spawn_bonus_scores_ui, spawn_statistics_ui, update_statistics_ui,
        },
    },
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
        let tile_size = 16;
        let tile_scale = 3;
        let tile_sizing = TileSize::new(tile_size, tile_scale);

        let min_size = WorldTileDimensions::new(4, 4);
        let max_size = calculate_max_size(&tile_sizing);

        let room_generator = RandomizedRoomGenerator::new(min_size, max_size);

        let core_logic = CoreLogic::new(movement_time, game_over_time, room_generator, tile_sizing);
        app.add_plugins(core_logic);

        app.insert_state(GameState::Start);
        app.add_systems(Startup, spawn_first_level);

        // Everything in this section sets up and interacts with the Graphical User Interface
        // for the game.
        let temporary_ui_time = TemporaryUITime::new(Duration::from_secs(3));
        app.insert_resource(temporary_ui_time);

        app.add_systems(OnEnter(GameState::Start), spawn_start_screen);
        app.add_systems(
            Update,
            start_the_game_after_some_time.run_if(in_state(GameState::Start)),
        );
        app.add_systems(OnExit(GameState::Start), despawn_start_screen);

        let ui_setup_systems = (
            prepare_screen_ui,
            spawn_statistics_ui.after(prepare_screen_ui),
            spawn_health_ui.after(spawn_statistics_ui),
            spawn_game_over_screen.after(spawn_health_ui),
            spawn_bonus_scores_ui.after(spawn_statistics_ui),
        );
        app.add_systems(Startup, ui_setup_systems);

        let ui_reaction_systems = (
            update_statistics_ui,
            update_health_ui,
            update_game_over_screen,
            show_bonus_scores_on_exit,
            hide_bonus_scores_after_time,
        );
        app.add_systems(Update, ui_reaction_systems);

        // This section makes aspects of the environment such as props come alive.
        let animation_systems = (animate_door_opening, animate_disarming_trap);
        app.add_systems(Update, animation_systems);

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

        let sound_and_music_systems = (
            trigger_door_opening_noise,
            trigger_trap_going_off_noise,
            trigger_treasure_claimed_noise,
            play_background_music.run_if(in_state(GameState::Active)),
            trigger_bonus_score_noise,
        );
        app.add_systems(Update, sound_and_music_systems);
        app.add_systems(
            OnEnter(GameState::GameOver),
            play_game_over_song.run_if(in_state(GameState::GameOver)),
        );

        // This section deals with how interactions are handled in the game
        // from outside sources, mainly from Twitch.

        // TODO: Remove this after playtesting.
        app.add_systems(Update, map_mouse_click_to_uv);

        app.insert_resource(TwitchClickListener::connect("103834034"));
        app.add_systems(Update, map_twitch_clicks_to_uv);
    }
}

/// Spawns the initial room when the application starts.
fn spawn_first_level(
    mut level_spawner_broadcaster: MessageWriter<ChangeRoom>,
    mut room_generator: ResMut<RandomizedRoomGenerator>,
) {
    let explorer_staring_position = LogicalCoordinates::new(1, 1);
    let initial_room = room_generator.generate(1, &explorer_staring_position);

    level_spawner_broadcaster.write(ChangeRoom::new(initial_room));
}

/// FOR PLAYTESTING PURPOSES: Maps a mouse click to UV coordinates
fn map_mouse_click_to_uv(
    mouse_state: Res<ButtonInput<MouseButton>>,
    window_info: Query<&Window>,
    mut viewer_click_broadcaster: MessageWriter<ViewerClick>,
) {
    if window_info.is_empty() {
        return;
    }

    if !mouse_state.just_pressed(MouseButton::Left) {
        return;
    }

    let window = window_info.single().unwrap();
    let window_width = window.physical_width() as f32;
    let window_height = window.physical_height() as f32;

    if let Some(physical_cursor_coordinates) = window.physical_cursor_position() {
        let uv_x = physical_cursor_coordinates.x / window_width;
        let uv_y = 1.0 - (physical_cursor_coordinates.y / window_height);

        let viewer_click_event = ViewerClick::new(uv_x, uv_y);
        viewer_click_broadcaster.write(viewer_click_event);
    }
}
