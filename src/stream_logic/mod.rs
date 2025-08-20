pub mod animation;
pub mod background_music;
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
    prepare_screen_ui, spawn_game_over_screen, spawn_health_ui, update_game_over_screen,
    update_health_ui,
};

use crate::{
    core_logic::{
        CoreLogic, GameOverTime, GameState, MovementTime,
        interacting::ViewerClick,
        scoring::CurrentRecords,
        setting::{
            ChangeRoom, LogicalCoordinates, RandomizedRoomGenerator, RoomGenerating,
            respawn_level_one, spawn_next_room,
        },
    },
    stream_logic::ui::{spawn_statistics_ui, update_statistics_ui},
};

pub struct StreamLogic;

impl Plugin for StreamLogic {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 720.0)
                            .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        let movement_time = MovementTime::new(Duration::from_secs(1));
        let game_over_time = GameOverTime::new(Duration::from_secs(20));

        let min_width = 4;
        let min_height = 4;
        let room_generator = RandomizedRoomGenerator::new(min_width, min_height);

        let tile_scale = 2;
        let core_logic = CoreLogic::new(movement_time, game_over_time, room_generator, tile_scale);
        app.add_plugins(core_logic);

        app.add_systems(Startup, spawn_first_level);

        app.add_systems(Startup, prepare_screen_ui);

        app.add_systems(Startup, spawn_statistics_ui.after(prepare_screen_ui));
        app.add_systems(Update, update_statistics_ui);

        app.add_systems(Startup, spawn_health_ui.after(spawn_statistics_ui));
        app.add_systems(Update, update_health_ui);

        app.add_systems(Startup, spawn_game_over_screen.after(spawn_health_ui));
        app.add_systems(Update, update_game_over_screen);

        app.add_systems(Update, animate_door_opening);
        app.add_systems(Update, animate_disarming_trap);

        app.add_systems(Update, trigger_door_opening_noise);
        app.add_systems(Update, trigger_trap_going_off_noise);
        app.add_systems(Update, trigger_treasure_claimed_noise);

        // TODO: Remove this after playtesting.
        app.add_systems(Update, map_mouse_click_to_uv);

        app.add_systems(
            Update,
            update_room_count_for_room_generator
                .before(respawn_level_one::<RandomizedRoomGenerator>)
                .before(spawn_next_room::<RandomizedRoomGenerator>),
        );

        let background_music_path = PathBuf::from("assets/background_music/");
        app.insert_resource(BackgroundPlayer::new(&background_music_path));
        app.add_systems(
            Update,
            play_background_music.run_if(in_state(GameState::Active)),
        );

        app.add_systems(
            OnEnter(GameState::GameOver),
            play_game_over_song.run_if(in_state(GameState::GameOver)),
        );
    }
}

/// Spawns the initial room when the application starts.
fn spawn_first_level(
    mut level_spawner_broadcaster: EventWriter<ChangeRoom>,
    room_generator: Res<RandomizedRoomGenerator>,
) {
    let explorer_staring_position = LogicalCoordinates::new(1, 1);
    let initial_room = room_generator.generate_with_explorer(&explorer_staring_position);

    level_spawner_broadcaster.write(ChangeRoom::new(initial_room));
}

/// FOR PLAYTESTING PURPOSES: Maps a mouse click to UV coordinates
fn map_mouse_click_to_uv(
    mouse_state: Res<ButtonInput<MouseButton>>,
    window_info: Query<&Window>,
    mut viewer_click_broadcaster: EventWriter<ViewerClick>,
) {
    if window_info.is_empty() {
        return;
    }

    if !mouse_state.pressed(MouseButton::Left) {
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

fn update_room_count_for_room_generator(
    current_records: Query<&CurrentRecords, Changed<CurrentRecords>>,
    mut room_generator: ResMut<RandomizedRoomGenerator>,
) {
    if current_records.is_empty() {
        return;
    }

    let current_room_number = current_records.single().unwrap().get_current_room_number();
    room_generator.set_room_number(current_room_number);
}
