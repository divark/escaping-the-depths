use std::time::Duration;

use bevy::prelude::*;

use bevy::window::WindowResolution;
use escaping_the_depths::core_logic::setting::*;
use escaping_the_depths::core_logic::*;

fn main() {
    let mut streaming_application = App::new();
    streaming_application.add_plugins(DefaultPlugins.set(WindowPlugin {
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

    let mut initial_room = room_generator.generate();
    initial_room.set(1, 1, RoomObject::Explorer);

    let core_logic = CoreLogic::new(movement_time, game_over_time, room_generator);
    streaming_application.add_plugins(core_logic);

    streaming_application
        .world_mut()
        .send_event(ChangeRoom::new(initial_room));

    streaming_application.run();
}
