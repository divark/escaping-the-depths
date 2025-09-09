use std::collections::HashSet;
use std::time::Duration;

use bevy::ecs::component::Mutable;
use bevy::input::InputPlugin;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::sprite::SpritePlugin;
use bevy::state::app::StatesPlugin;
use bevy::window::WindowResolution;
use cucumber::World;

use bevy::prelude::*;

use escaping_the_depths::core_logic::interacting::*;
use escaping_the_depths::core_logic::scoring::*;
use escaping_the_depths::core_logic::setting::*;
use escaping_the_depths::core_logic::traveling::*;
use escaping_the_depths::core_logic::*;

const TICKING_LIMIT: usize = 100;

#[derive(Clone, Resource)]
pub struct TestRoomGenerator {
    room_generator: RandomizedRoomGenerator,
    cave_room_objects: Vec<PlaceRoomObject>,
}

impl RoomGenerating for TestRoomGenerator {
    fn generate_empty_room(&mut self, room_number: usize) -> CaveRoom {
        self.room_generator.generate_empty_room(room_number)
    }

    fn generate(
        &mut self,
        room_number: usize,
        desired_explorer_location: &LogicalCoordinates,
    ) -> CaveRoom {
        let mut room_generated = self.room_generator.generate_empty_room(room_number);

        let max_room_size = self.room_generator.get_max_room_size();
        place_explorer(
            &mut room_generated,
            max_room_size,
            desired_explorer_location,
        );

        while let Some(cave_room_object) = self.cave_room_objects.pop() {
            room_generated.set(
                cave_room_object.get_x(),
                cave_room_object.get_y(),
                *cave_room_object.get_type(),
            );
        }

        room_generated
    }
}

impl TestRoomGenerator {
    pub fn new(room_size: WorldTileDimensions) -> Self {
        let default_max_offset = 5;
        let default_max_room_size = WorldTileDimensions::new(
            room_size.get_width() + default_max_offset,
            room_size.get_height() + default_max_offset,
        );

        Self {
            room_generator: RandomizedRoomGenerator::new(room_size, default_max_room_size),
            cave_room_objects: Vec::new(),
        }
    }

    pub fn set_min_size(&mut self, min_size: WorldTileDimensions) {
        self.room_generator.set_min_size(min_size);
    }

    pub fn set_max_size(&mut self, max_size: WorldTileDimensions) {
        self.room_generator.set_max_size(max_size);
    }

    pub fn add_room_object(&mut self, object_type: RoomObject, object_x: usize, object_y: usize) {
        let room_object = PlaceRoomObject::new(object_type, object_x, object_y, 1);
        self.cave_room_objects.push(room_object);
    }
}

pub fn parse_object_type(object_name: String) -> RoomObject {
    match object_name.as_str() {
        "hidden floor switch" => RoomObject::HiddenFloorSwitch,
        "exit door" => RoomObject::ExitDoor,
        "armed trap" => RoomObject::Trap,
        "explorer" => RoomObject::Explorer,
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
    desired_explorer_location: LogicalCoordinates,
}

impl MockGame {
    pub fn new() -> Self {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(InputPlugin);
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
        app.add_plugins(SpritePlugin);
        app.add_plugins(StatesPlugin);
        app.add_plugins(DefaultPickingPlugins);

        Self {
            app,
            desired_explorer_location: LogicalCoordinates::default(),
        }
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

    pub fn get_one<T>(&mut self) -> &T
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

    fn get_one_mut<T>(&mut self) -> Mut<'_, T>
    where
        T: Component<Mutability = Mutable>,
    {
        self.app
            .world_mut()
            .query::<&mut T>()
            .iter_mut(self.app.world_mut())
            .next()
            .expect("get_one_mut: Could not find one mutable instance of the specified component")
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

    fn get_all_without<T, F>(&mut self) -> Vec<&T>
    where
        T: Component,
        F: Component + PartialEq,
    {
        self.app
            .world_mut()
            .query_filtered::<&T, Without<F>>()
            .iter(self.app.world_mut())
            .collect()
    }

    fn get_resource<T>(&mut self) -> &T
    where
        T: Resource,
    {
        self.app
            .world_mut()
            .get_resource::<T>()
            .expect("get_resource: Could not find the desired resource.")
    }

    pub fn get_resource_mut<T>(&mut self) -> Mut<'_, T>
    where
        T: Resource,
    {
        self.app
            .world_mut()
            .get_resource_mut::<T>()
            .expect("get_resource_mut: Could not find the desired resource.")
    }

    fn get_game_state<T>(&mut self) -> &State<T>
    where
        T: States,
    {
        self.get_resource::<State<T>>()
    }

    pub fn spawn_room(&mut self, width: usize, height: usize) {
        let tile_sizing = TileSize::new(16, 1);
        let test_room_generator = TestRoomGenerator::new(WorldTileDimensions::new(width, height));
        let movement_time = MovementTime::new(Duration::from_secs(0));
        let game_over_time = GameOverTime::new(Duration::from_secs(0));
        self.app.add_plugins(CoreLogic::new(
            movement_time,
            game_over_time,
            test_room_generator,
            tile_sizing,
        ));

        self.tick();
    }

    pub fn render_room(&mut self) {
        let desired_explorer_location = self.desired_explorer_location;
        let room_number = self.get_one::<CurrentRecords>().get_current_room_number();
        let mut room_generator = self.get_resource_mut::<TestRoomGenerator>();

        let cave_room = room_generator.generate(room_number, &desired_explorer_location);
        self.broadcast(ChangeRoom::new(cave_room));
        self.tick();
    }

    pub fn place(&mut self, object_type: RoomObject, object_x: usize, object_y: usize) {
        let mut test_room_generator = self.get_resource_mut::<TestRoomGenerator>();
        if object_type == RoomObject::Explorer {
            self.desired_explorer_location = LogicalCoordinates::new(object_x, object_y);
            return;
        }

        test_room_generator.add_room_object(object_type, object_x, object_y);
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
        let explorer_path = self.get_with::<Pathfinding, ExplorerState>();
        let explorer_destination_overall = *explorer_path.get_destination();
        explorer_destination_overall
    }

    pub fn get_explorer_health(&mut self) -> &ExplorerHealth {
        let explorer_health = self.get_resource::<ExplorerHealth>();
        explorer_health
    }

    pub fn set_explorer_health(&mut self, current_health: usize, total_health: usize) {
        let mut explorer_health = self.get_resource_mut::<ExplorerHealth>();
        explorer_health.set_current_health(current_health);
        explorer_health.set_total_health(total_health);
    }

    pub fn get_explorer_state(&mut self) -> ExplorerState {
        *self.get_one()
    }

    pub fn get_explorer_position(&mut self) -> LogicalCoordinates {
        let explorer_position = *self.get_with::<LogicalCoordinates, ExplorerState>();
        explorer_position
    }

    pub fn wait_for_explorer_to_wander_again(&mut self) {
        loop {
            let explorer_state = *self.get_one::<ExplorerState>();
            if explorer_state != ExplorerState::Wandering {
                self.tick();
                continue;
            }

            break;
        }
    }

    pub fn wait_for_explorer_to_finish_exiting(&mut self) {
        loop {
            let explorer_state = *self.get_one::<ExplorerState>();
            if explorer_state == ExplorerState::Wandering {
                self.tick();
                continue;
            }

            break;
        }

        loop {
            let explorer_path = self.get_one::<Pathfinding>();
            let num_places_to_visit = explorer_path.get_locations().len();

            if num_places_to_visit == 0 {
                self.tick();
                break;
            }

            self.tick();
        }
    }

    pub fn get_traversible_tiles(&mut self) -> HashSet<LogicalCoordinates> {
        let all_tiles = self.get_all_without::<LogicalCoordinates, Wall>();

        let mut all_unique_tiles = HashSet::new();
        for tile in all_tiles {
            all_unique_tiles.insert(*tile);
        }

        all_unique_tiles
    }

    pub fn get_explorer_tile_locations_to_be_visited(&mut self) -> HashSet<LogicalCoordinates> {
        let explorer_path = self.get_one::<Pathfinding>();
        let explorer_tiles_to_be_visited = explorer_path.get_locations();

        explorer_tiles_to_be_visited
            .iter()
            .cloned()
            .collect::<HashSet<LogicalCoordinates>>()
    }

    pub fn get_explorer_tile_types_to_be_visited(&mut self) -> HashSet<RoomObject> {
        let explorer_path = self.get_one::<Pathfinding>();
        let explorer_tile_types_to_be_visited = explorer_path.get_types();

        explorer_tile_types_to_be_visited
            .iter()
            .cloned()
            .collect::<HashSet<RoomObject>>()
    }

    pub fn wait_for_explorer_to_reach(&mut self, position: LogicalCoordinates) {
        for _i in 0..TICKING_LIMIT {
            let explorer_position = self.get_with::<LogicalCoordinates, ExplorerState>();
            if *explorer_position == position {
                self.tick();
                break;
            }

            self.tick();
        }
    }

    pub fn get_current_room_number(&mut self) -> usize {
        let current_records = self.get_one::<CurrentRecords>();
        current_records.get_current_room_number()
    }

    pub fn wait_for_game_over_timer_to_finish(&mut self) {
        loop {
            self.tick();
            let game_state = self.get_game_state::<GameState>();

            if *game_state == GameState::Active {
                break;
            }
        }
    }

    pub fn set_current_room_number(&mut self, current_room_num: usize) {
        let mut current_records = self.get_one_mut::<CurrentRecords>();
        current_records.set_current_room_number(current_room_num);
    }

    pub fn set_current_score(&mut self, current_score: usize) {
        let mut current_records = self.get_one_mut::<CurrentRecords>();
        current_records.set_current_score(current_score);
    }

    pub fn get_record_score(&mut self) -> usize {
        let current_records = self.get_one::<CurrentRecords>();
        current_records.get_record_score()
    }

    pub fn get_record_room_number(&mut self) -> usize {
        let current_records = self.get_one::<CurrentRecords>();
        current_records.get_record_room_number()
    }

    pub fn set_tile_scale(&mut self, desired_tile_scale: usize) {
        let mut current_tile_scale = self.get_resource_mut::<TileSize>();
        current_tile_scale.set_scale(desired_tile_scale);
    }
}

impl Default for MockGame {
    fn default() -> Self {
        Self::new()
    }
}
