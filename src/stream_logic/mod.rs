pub mod animation;
pub mod ui;

use std::time::Duration;

use animation::{animate_disarming_trap, animate_door_opening};
use bevy::{prelude::*, window::WindowResolution};
use ui::{
    prepare_screen_ui, spawn_game_over_screen, spawn_health_ui, update_game_over_screen,
    update_health_ui,
};

use crate::{
    core_logic::{
        CoreLogic, GameOverTime, MovementTime,
        interacting::ViewerClick,
        setting::{ChangeRoom, LogicalCoordinates, RandomizedRoomGenerator, RoomGenerating},
    },
    stream_logic::ui::{spawn_statistics_ui, update_statistics_ui},
};

pub struct StreamLogic;

impl Plugin for StreamLogic {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280.0, 720.0).with_scale_factor_override(2.0),
                ..default()
            }),
            ..default()
        }));

        let movement_time = MovementTime::new(Duration::from_secs(1));
        let game_over_time = GameOverTime::new(Duration::from_secs(20));

        let min_width = 4;
        let min_height = 4;
        let room_generator = RandomizedRoomGenerator::new(min_width, min_height);

        let core_logic = CoreLogic::new(movement_time, game_over_time, room_generator);
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

        // TODO: Remove this after playtesting.
        app.add_systems(Update, map_mouse_click_to_uv);
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
