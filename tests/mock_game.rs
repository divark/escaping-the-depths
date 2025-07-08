use bevy::input::InputPlugin;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::sprite::SpritePlugin;
use bevy::state::app::StatesPlugin;
use bevy::window::WindowResolution;
use cucumber::World;

use bevy::prelude::*;

use escaping_the_depths::game_logic::pathfinding::Pathfinding;
use escaping_the_depths::game_logic::room_generating::*;
use escaping_the_depths::game_logic::viewer_interaction::ViewerClick;
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
        let room_generated = CaveRoom::new(self.width, self.height);

        room_generated
    }
}

pub fn parse_object_type(object_name: String) -> RoomObject {
    match object_name.as_str() {
        "hidden floor switch" => RoomObject::HiddenFloorSwitch,
        "exit door" => RoomObject::ExitDoor,
        "armed trap" => RoomObject::Trap,
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
        app.add_plugins(MinimalPlugins);
        app.add_plugins(InputPlugin::default());
        app.add_plugins(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        });
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(RenderPlugin {
            render_creation: WgpuSettings {
                backends: None,
                ..default()
            }
            .into(),
            ..default()
        });
        app.add_plugins(ImagePlugin::default());
        app.add_plugins(SpritePlugin::default());
        app.add_plugins(StatesPlugin);
        app.add_plugins(DefaultPickingPlugins);

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

    fn get_one<T>(&mut self) -> &T
    where
        T: Component,
    {
        self.app
            .world_mut()
            .query::<&T>()
            .iter(self.app.world_mut())
            .next()
            .expect("get_one: Could not find one instance of the specified component.")
    }

    fn get_at<T>(&mut self, logical_coordinates: &LogicalCoordinates) -> &T
    where
        T: Component,
    {
        self.app
            .world_mut()
            .query::<(&T, &LogicalCoordinates)>()
            .iter(self.app.world_mut())
            .find(|trap| trap.1 == logical_coordinates)
            .expect("get_at: Could not find component at logical coordinates.")
            .0
    }

    fn get_with<T, U>(&mut self) -> &T
    where
        T: Component,
        U: Component,
    {
        self.app
            .world_mut()
            .query_filtered::<&T, With<U>>()
            .iter(self.app.world_mut())
            .next()
            .expect("get_with: Could not find component with dependency.")
    }

    pub fn spawn_room(&mut self, room: CaveRoom) {
        self.broadcast(ChangeRoom::new(room));
        self.tick();
    }

    pub fn place(&mut self, object_type: RoomObject, object_x: usize, object_y: usize) {
        let depth_to_place = if object_type == RoomObject::Explorer {
            2
        } else {
            1
        };

        self.broadcast(PlaceRoomObject::new(
            object_type,
            object_x,
            object_y,
            depth_to_place,
        ));

        self.tick();
    }

    pub fn click(&mut self, uv_x: f32, uv_y: f32) {
        self.broadcast(ViewerClick::new(uv_x, uv_y));

        self.tick();
    }

    pub fn get_door_state(&mut self) -> ExitDoorState {
        let found_door_state = self.get_one::<ExitDoorState>();
        *found_door_state
    }

    pub fn get_current_score(&mut self) -> usize {
        let current_record = self.get_one::<CurrentRecords>();
        current_record.get_current_score()
    }

    pub fn get_trap_at(&mut self, trap_tile_x: usize, trap_tile_y: usize) -> TrapState {
        let trap_logical_position = LogicalCoordinates::new(trap_tile_x, trap_tile_y);
        let found_trap_state = self.get_at::<TrapState>(&trap_logical_position);
        *found_trap_state
    }

    pub fn get_explorer_destination_overall(&mut self) -> LogicalCoordinates {
        self.tick();

        let explorer_path = self.get_with::<Pathfinding, ExplorerState>();
        let explorer_destination_overall = *explorer_path.get_destination();
        explorer_destination_overall
    }
}
