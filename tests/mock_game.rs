use cucumber::World;

use bevy::prelude::*;

use escaping_the_depths::game_logic::room_generating::*;
use escaping_the_depths::game_logic::*;
use escaping_the_depths::*;

pub struct TestRoomGenerator {
    width: usize,
    height: usize,
}

impl TestRoomGenerator {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn generate(&mut self) -> CaveRoom {
        let mut room_generated = CaveRoom::new(self.width, self.height);

        room_generated.set(1, 1, RoomObject::Explorer);

        room_generated
    }
}

pub fn parse_object_type(object_name: String) -> RoomObject {
    match object_name.as_str() {
        "hidden floor switch" => RoomObject::HiddenFloorSwitch,
        "exit door" => RoomObject::ExitDoor,
        _ => panic!(
            "parse_object_type: {} is not a known room object.",
            object_name
        ),
    }
}

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct MockGame {
    app: App,
}

impl MockGame {
    pub fn new() -> Self {
        let mut app = App::new();
        app.add_plugins(CoreLogic);

        Self { app }
    }

    fn tick(&mut self) {
        self.app.update();
    }

    fn broadcast<T>(&mut self, event: T)
    where
        T: Event,
    {
        self.app
            .world_mut()
            .send_event(event)
            .expect("broadcast: Could not send event.");

        self.tick();
    }

    pub fn spawn_room(&mut self, room: CaveRoom) {
        self.broadcast(ChangeRoom::new(room));
        self.tick();
    }

    pub fn place(&mut self, object_type: RoomObject, object_x: usize, object_y: usize) {
        self.broadcast(PlaceRoomObject::new(object_type, object_x, object_y));
        self.tick();
    }

    pub fn move_explorer_to(&mut self, desired_x: usize, desired_y: usize) {}

    pub fn get_door_state(&mut self, door_x: usize, door_y: usize) -> ExitDoorState {
        ExitDoorState::Closed
    }
}
