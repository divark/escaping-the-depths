use std::time::Duration;

use bevy::prelude::*;
use escaping_the_depths::{
    RoomGenerating, RoomObject,
    game_logic::{
        CoreLogic, GameOverTime, MovementTime,
        room_generating::{ChangeRoom, RandomizedRoomGenerator},
    },
};

fn main() {
    let mut streaming_application = App::new();
    streaming_application.add_plugins(DefaultPlugins);

    let movement_time = MovementTime::new(Duration::from_secs(2));
    let game_over_time = GameOverTime::new(Duration::from_secs(10));

    let min_width = 3;
    let min_height = 3;
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
