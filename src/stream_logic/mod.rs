pub mod ui;

use std::time::Duration;

use bevy::{prelude::*, window::WindowResolution};

use crate::core_logic::{
    CoreLogic, GameOverTime, MovementTime,
    setting::{ChangeRoom, RandomizedRoomGenerator, RoomGenerating, RoomObject},
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
        let game_over_time = GameOverTime::new(Duration::from_secs(10));

        let min_width = 4;
        let min_height = 4;
        let room_generator = RandomizedRoomGenerator::new(min_width, min_height);

        let core_logic = CoreLogic::new(movement_time, game_over_time, room_generator);
        app.add_plugins(core_logic);

        app.add_systems(Startup, spawn_first_level);

        app.add_systems(Startup, spawn_statistics_ui);
        app.add_systems(Update, update_statistics_ui);
    }
}

/// Spawns the initial room when the application starts.
fn spawn_first_level(
    mut level_spawner_broadcaster: EventWriter<ChangeRoom>,
    room_generator: Res<RandomizedRoomGenerator>,
) {
    let mut initial_room = room_generator.generate();
    initial_room.set(1, 1, RoomObject::Explorer);

    level_spawner_broadcaster.write(ChangeRoom::new(initial_room));
}
