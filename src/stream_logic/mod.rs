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
        let game_over_time = GameOverTime::new(Duration::from_secs(30));

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
