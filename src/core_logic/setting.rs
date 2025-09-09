use std::collections::HashSet;

use bevy::prelude::*;
use rand::Rng;

use super::{
    GameOverTimer, GameState,
    scoring::{CurrentRecords, ExplorerHealth, TrapState, TreasureScore, TreasureState},
    traveling::{ExitDoorState, Graph},
};

pub const WALLS_OFFSET: usize = 2;

#[derive(Resource, Clone)]
pub struct TileSize {
    size: usize,
    scale: usize,
}

impl Default for TileSize {
    fn default() -> Self {
        Self { size: 16, scale: 1 }
    }
}

impl TileSize {
    pub fn new(size: usize, scale: usize) -> Self {
        Self { size, scale }
    }

    pub fn set_scale(&mut self, desired_scale: usize) {
        self.scale = desired_scale;
    }

    pub fn get_scale(&self) -> usize {
        self.scale
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn calculate_size(&self) -> usize {
        self.size * self.scale
    }
}

#[derive(Clone, Copy, PartialEq, Default, Hash, Eq, Debug)]
pub enum RoomObject {
    #[default]
    Empty,
    Wall,
    Explorer,
    ExitDoor,
    HiddenFloorSwitch,
    Treasure(usize),
    Trap,
}

pub trait RoomGenerating {
    fn generate_empty_room(&mut self, room_number: usize) -> CaveRoom;
    fn generate(
        &mut self,
        room_number: usize,
        desired_explorer_location: &LogicalCoordinates,
    ) -> CaveRoom;
}

#[derive(PartialEq, Component, Clone, Copy, Debug, Default)]
pub struct WorldTileDimensions {
    width: usize,
    height: usize,
}

impl WorldTileDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

#[derive(Event, Component, Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct LogicalCoordinates {
    x: usize,
    y: usize,
}

impl LogicalCoordinates {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn to_1d(&self, world_tile_dimensions: &WorldTileDimensions) -> usize {
        (world_tile_dimensions.get_height() * self.get_y()) + self.get_x()
    }
}

#[derive(Clone, Default, Debug)]
pub struct Tile {
    logical_coordinates: LogicalCoordinates,
    tile_type: RoomObject,
}

impl Tile {
    pub fn new(logical_coordinates: LogicalCoordinates) -> Self {
        Self {
            logical_coordinates,
            tile_type: RoomObject::default(),
        }
    }

    pub fn get_type(&self) -> &RoomObject {
        &self.tile_type
    }

    pub fn get_logical_coordinates(&self) -> &LogicalCoordinates {
        &self.logical_coordinates
    }

    pub fn set_type(&mut self, new_type: RoomObject) {
        self.tile_type = new_type;
    }
}

#[derive(Clone, Debug)]
pub struct CaveRoom {
    world_tile_dimensions: WorldTileDimensions,

    room_tiles: Vec<Tile>,
    room_objects: Vec<Tile>,
}

impl CaveRoom {
    pub fn new(width: usize, height: usize) -> Self {
        let mut room_tiles = Vec::new();
        for col_idx in 0..width {
            for row_idx in 0..height {
                room_tiles.push(Tile::new(LogicalCoordinates::new(row_idx, col_idx)));
            }
        }

        let world_tile_dimensions = WorldTileDimensions::new(width, height);
        Self {
            world_tile_dimensions,
            room_tiles,
            room_objects: Vec::new(),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile_type: RoomObject) {
        let logical_coordinates = LogicalCoordinates::new(x, y);
        if tile_type == RoomObject::Wall || tile_type == RoomObject::ExitDoor {
            let error_msg = &format!(
                "set: Could not find designated tile {} {} for wall.",
                logical_coordinates.get_x(),
                logical_coordinates.get_y()
            );

            let found_tile = self
                .room_tiles
                .iter_mut()
                .find(|tile| tile.get_logical_coordinates() == &logical_coordinates)
                .expect(error_msg);
            found_tile.set_type(tile_type);

            return;
        }

        let mut room_object = Tile::new(logical_coordinates);
        room_object.set_type(tile_type);

        self.room_objects.push(room_object);
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.room_tiles
    }

    pub fn get_objects(&self) -> &Vec<Tile> {
        &self.room_objects
    }

    pub fn get_width(&self) -> usize {
        self.world_tile_dimensions.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.world_tile_dimensions.get_height()
    }

    pub fn get_dimensions(&self) -> &WorldTileDimensions {
        &self.world_tile_dimensions
    }
}

#[derive(Event)]
pub struct ChangeRoom(CaveRoom);

impl ChangeRoom {
    pub fn new(room_to_spawn: CaveRoom) -> Self {
        Self(room_to_spawn)
    }

    pub fn get_room(&self) -> &CaveRoom {
        &self.0
    }
}

#[derive(Event)]
pub struct LoadRoom(CaveRoom);

impl LoadRoom {
    pub fn from_change_room(change_room_event: &ChangeRoom) -> Self {
        Self(change_room_event.get_room().clone())
    }

    pub fn get_room(&self) -> &CaveRoom {
        &self.0
    }
}

#[derive(Event, Clone, Debug)]
pub struct PlaceRoomObject {
    object_type: RoomObject,
    x: usize,
    y: usize,
    z: usize,
}

impl PlaceRoomObject {
    pub fn new(object_type: RoomObject, x: usize, y: usize, z: usize) -> Self {
        Self {
            object_type,
            x,
            y,
            z,
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_type(&self) -> &RoomObject {
        &self.object_type
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Wall;

#[derive(Bundle, Default)]
pub struct SpriteBundle {
    sprite: Sprite,
    position: Transform,
}

impl SpriteBundle {
    pub fn set_sprite(&mut self, new_sprite: Sprite) {
        self.sprite = new_sprite;
    }

    pub fn set_position(&mut self, new_position: Transform) {
        self.position = new_position;
    }
}

#[derive(Bundle, Default)]
pub struct TileBundle {
    sprite_bundle: SpriteBundle,
    logical_coordinates: LogicalCoordinates,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub enum ExplorerState {
    #[default]
    Wandering,
    Exiting,
    Dead,
}

#[derive(Component)]
pub struct ExitDoor;

#[derive(Component)]
pub struct HiddenFloorSwitch;

fn spawn_room(
    cave_room: &CaveRoom,
    place_tile_broadcaster: &mut EventWriter<PlaceRoomObject>,
    commands: &mut Commands,
) {
    let cave_room_tiles = cave_room.get_tiles();
    for tile in cave_room_tiles {
        let tile_type = tile.get_type();
        let tile_coords = tile.get_logical_coordinates();

        place_tile_broadcaster.write(PlaceRoomObject::new(
            *tile_type,
            tile_coords.get_x(),
            tile_coords.get_y(),
            0,
        ));
    }

    commands.spawn(*cave_room.get_dimensions());
}

fn spawn_objects_in_room(
    cave_room: &CaveRoom,
    place_tile_broadcaster: &mut EventWriter<PlaceRoomObject>,
) {
    let cave_room_objects = cave_room.get_objects();
    for room_object in cave_room_objects {
        let tile_type = room_object.get_type();
        let tile_coords = room_object.get_logical_coordinates();

        let tile_depth = if *tile_type == RoomObject::Explorer {
            2
        } else {
            1
        };

        place_tile_broadcaster.write(PlaceRoomObject::new(
            *tile_type,
            tile_coords.get_x(),
            tile_coords.get_y(),
            tile_depth,
        ));
    }
}

fn spawn_centered_camera(cave_room: &CaveRoom, tile_scale: &TileSize, commands: &mut Commands) {
    let centered_on_map_camera = Camera2d;
    let cave_room_px_width = cave_room.get_width() * tile_scale.calculate_size();
    let cave_room_px_height = cave_room.get_height() * tile_scale.calculate_size();
    let camera_position = Transform::from_xyz(
        cave_room_px_width as f32 / 2.0,
        cave_room_px_height as f32 / 2.0,
        3.0,
    );
    commands.spawn((centered_on_map_camera, camera_position));
}

fn spawn_room_traversal_graph(cave_room: &CaveRoom, commands: &mut Commands) {
    let room_tile_locations = cave_room
        .get_tiles()
        .iter()
        .map(|tile| (*tile.get_logical_coordinates(), *tile.get_type()))
        .collect();

    let traversal_graph = Graph::from_tiles(&room_tile_locations, cave_room.get_dimensions());
    commands.spawn(traversal_graph);
}

pub fn spawn_new_room(
    mut spawn_room_requests: EventReader<LoadRoom>,
    mut place_tile_broadcaster: EventWriter<PlaceRoomObject>,
    tile_scale: Res<TileSize>,
    mut commands: Commands,
) {
    if spawn_room_requests.is_empty() {
        return;
    }

    let cave_room = spawn_room_requests.read().next().unwrap().get_room();
    spawn_room(cave_room, &mut place_tile_broadcaster, &mut commands);
    spawn_objects_in_room(cave_room, &mut place_tile_broadcaster);
    spawn_room_traversal_graph(cave_room, &mut commands);
    spawn_centered_camera(cave_room, &tile_scale, &mut commands);
}

pub fn spawn_next_room<T>(
    mut change_room_broadcaster: EventWriter<ChangeRoom>,
    explorer: Query<(&LogicalCoordinates, &ExplorerState), Changed<LogicalCoordinates>>,
    exit_door: Query<(&LogicalCoordinates, &ExitDoorState)>,
    armed_traps: Query<&TrapState>,
    mut current_records: Query<&mut CurrentRecords>,
    mut room_generator: ResMut<T>,
) where
    T: Resource + RoomGenerating,
{
    if explorer.is_empty() || exit_door.is_empty() || current_records.is_empty() {
        return;
    }

    let (exit_door_location, exit_door_state) = exit_door
        .single()
        .expect("spawn_next_room: Could not find the exit door.");
    let (explorer_location, explorer_state) = explorer
        .single()
        .expect("spawn_next_room: Could not find explorer.");
    let explorer_hasnt_reached_exit =
        *explorer_state != ExplorerState::Exiting || explorer_location != exit_door_location;
    let exit_door_not_opened = *exit_door_state != ExitDoorState::Open;
    if explorer_hasnt_reached_exit || exit_door_not_opened {
        return;
    }

    let mut current_records = current_records
        .single_mut()
        .expect("spawn_next_room: Could not get current records.");
    current_records.increment_room_count();

    let num_traps_still_armed = armed_traps
        .iter()
        .filter(|&trap_state| trap_state == &TrapState::Armed)
        .count();
    current_records.add_score(num_traps_still_armed * 200);

    let explorer_starting_location = LogicalCoordinates::new(explorer_location.get_x(), 1);
    let newly_generated_caveroom = room_generator.generate(
        current_records.get_current_room_number(),
        &explorer_starting_location,
    );

    let change_room_request = ChangeRoom::new(newly_generated_caveroom);
    change_room_broadcaster.write(change_room_request);
}

pub fn reset_to_level_one_after_game_over(
    mut game_over_timers: Query<(Entity, &mut GameOverTimer)>,
    time: Res<Time>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut explorer_health: ResMut<ExplorerHealth>,
    mut scores: Query<&mut CurrentRecords>,
    mut commands: Commands,
) {
    if game_over_timers.is_empty() || scores.is_empty() {
        return;
    }

    let (game_over_timer_entity, mut game_over_timer) = game_over_timers
        .single_mut()
        .expect("reset_to_level_one_after_game_over: Could not find game over timer.");

    let time_passed = time.delta();
    let game_over_timer = game_over_timer.get_timer_mut();
    game_over_timer.tick(time_passed);

    if !game_over_timer.just_finished() {
        return;
    }

    commands.entity(game_over_timer_entity).despawn();

    explorer_health.set_current_health(3);
    explorer_health.set_total_health(3);

    let mut score_keeping = scores
        .single_mut()
        .expect("reset_to_level_one_after_game_over: Could not find the current record of scores.");
    score_keeping.reset();

    next_game_state.set(GameState::Active);
}

pub fn respawn_level_one<T>(
    mut room_generator: ResMut<T>,
    mut change_room_broadcaster: EventWriter<ChangeRoom>,
) where
    T: Resource + RoomGenerating,
{
    let explorer_starting_location = LogicalCoordinates::new(1, 1);
    let newly_generated_caveroom = room_generator.generate(1, &explorer_starting_location);

    let change_room_request = ChangeRoom::new(newly_generated_caveroom);
    change_room_broadcaster.write(change_room_request);
}

pub fn despawn_current_room(
    mut change_room_requests: EventReader<ChangeRoom>,
    mut spawn_room_broadcaster: EventWriter<LoadRoom>,
    mut commands: Commands,
    all_room_tiles: Query<Entity, With<LogicalCoordinates>>,
    room_graph: Query<Entity, With<Graph>>,
    room_size: Query<Entity, With<WorldTileDimensions>>,
    camera_in_room: Query<Entity, With<Camera2d>>,
) {
    if change_room_requests.is_empty() {
        return;
    }

    for tile_entity in all_room_tiles.iter() {
        commands.entity(tile_entity).despawn();
    }

    for room_graph_entity in room_graph.iter() {
        commands.entity(room_graph_entity).despawn();
    }

    for room_size_entity in room_size.iter() {
        commands.entity(room_size_entity).despawn();
    }

    for camera_entity in camera_in_room.iter() {
        commands.entity(camera_entity).despawn();
    }

    let change_room_event = change_room_requests
        .read()
        .next()
        .expect("despawn_current_room: Could not get ChangeRoom request.");
    let load_room_event = LoadRoom::from_change_room(change_room_event);
    spawn_room_broadcaster.write(load_room_event);
}

fn get_tile_sprite(
    tile_to_place: &PlaceRoomObject,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    tile_scale: &TileSize,
) -> Sprite {
    let tile_type = tile_to_place.object_type;
    let mut num_sprites = 1;
    let sprite_file_path = match tile_type {
        RoomObject::Empty => "environment/floor_1.png",
        RoomObject::Wall => "environment/wall_mid.png",
        RoomObject::Explorer => "characters/npc_merchant_2.png",
        RoomObject::ExitDoor => {
            num_sprites = 2;
            "environment/wall_hole_to_floor.png"
        }
        RoomObject::HiddenFloorSwitch => "environment/floor_4.png",
        RoomObject::Treasure(_) => "environment/chest_full_open_anim_f1.png",
        RoomObject::Trap => {
            num_sprites = 2;
            "environment/floor_spikes_atlas.png"
        }
    };

    let spritesheet_image = asset_server.load(sprite_file_path);
    let adjusted_tile_size = tile_scale.calculate_size() as f32;

    if num_sprites == 1 {
        return Sprite {
            custom_size: Some(Vec2::new(adjusted_tile_size, adjusted_tile_size)),
            image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
            ..Sprite::from_image(spritesheet_image)
        };
    }

    let tile_sprite_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(tile_scale.get_size() as u32),
        num_sprites,
        1,
        None,
        None,
    );
    let loaded_atlas_layout = texture_atlas_layouts.add(tile_sprite_atlas_layout);
    let tile_spritesheet_atlas = TextureAtlas {
        layout: loaded_atlas_layout,
        index: 0,
    };

    Sprite {
        custom_size: Some(Vec2::new(adjusted_tile_size, adjusted_tile_size)),
        image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
        ..Sprite::from_atlas_image(spritesheet_image, tile_spritesheet_atlas)
    }
}

fn get_tile_position(tile_to_place: &PlaceRoomObject, tile_scale: &TileSize) -> Transform {
    let tile_x = tile_to_place.x * tile_scale.calculate_size();
    let tile_y = tile_to_place.y * tile_scale.calculate_size();

    let tile_z = tile_to_place.z;
    Transform::from_xyz(tile_x as f32, tile_y as f32, tile_z as f32)
}

fn convert_to_rendered_tile(
    tile_to_place: &PlaceRoomObject,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    tile_scale: &TileSize,
) -> TileBundle {
    let mut tile_sprite_bundle = SpriteBundle::default();

    let tile_sprite = get_tile_sprite(
        tile_to_place,
        asset_server,
        texture_atlas_layouts,
        tile_scale,
    );
    tile_sprite_bundle.set_sprite(tile_sprite);

    let tile_position = get_tile_position(tile_to_place, tile_scale);
    tile_sprite_bundle.set_position(tile_position);

    let tile_logical_position = LogicalCoordinates::new(tile_to_place.x, tile_to_place.y);
    TileBundle::new(tile_sprite_bundle, tile_logical_position)
}

impl TileBundle {
    pub fn new(sprite_bundle: SpriteBundle, logical_coordinates: LogicalCoordinates) -> Self {
        Self {
            sprite_bundle,
            logical_coordinates,
        }
    }
}

#[derive(Bundle)]
pub struct ExplorerBundle {
    tile_bundle: TileBundle,
    explorer_state: ExplorerState,
}

impl ExplorerBundle {
    pub fn new(tile_bundle: TileBundle) -> Self {
        Self {
            tile_bundle,
            explorer_state: ExplorerState::default(),
        }
    }
}

pub fn place_tile(
    mut place_tile_requests: EventReader<PlaceRoomObject>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    tile_scale: Res<TileSize>,
    mut commands: Commands,
) {
    for tile_to_place in place_tile_requests.read() {
        let rendered_tile = convert_to_rendered_tile(
            tile_to_place,
            &asset_server,
            &mut texture_atlas_layouts,
            &tile_scale,
        );
        match tile_to_place.object_type {
            RoomObject::Explorer => {
                commands.spawn(ExplorerBundle::new(rendered_tile));
            }
            RoomObject::Wall => {
                commands.spawn((rendered_tile, Wall));
            }
            RoomObject::ExitDoor => {
                commands.spawn((rendered_tile, ExitDoorState::Closed));
            }
            RoomObject::HiddenFloorSwitch => {
                commands.spawn((rendered_tile, HiddenFloorSwitch));
            }
            RoomObject::Treasure(treasure_value) => {
                commands.spawn((
                    rendered_tile,
                    TreasureScore::new(treasure_value),
                    TreasureState::Unclaimed,
                ));
            }
            RoomObject::Trap => {
                commands.spawn((rendered_tile, TrapState::Armed));
            }
            _ => {
                commands.spawn(rendered_tile);
            }
        }
    }
}

#[derive(Resource, Clone)]
pub struct RandomizedRoomGenerator {
    min_size: WorldTileDimensions,
    max_size: WorldTileDimensions,
}

impl RandomizedRoomGenerator {
    pub fn new(min_size: WorldTileDimensions, max_size: WorldTileDimensions) -> Self {
        Self { min_size, max_size }
    }

    pub fn set_min_size(&mut self, new_min_size: WorldTileDimensions) {
        self.min_size = new_min_size;
    }

    pub fn set_max_size(&mut self, new_max_size: WorldTileDimensions) {
        self.max_size = new_max_size;
    }

    pub fn get_max_room_size(&self) -> &WorldTileDimensions {
        &self.max_size
    }
}

fn add_walls_to_row(generated_cave_room: &mut CaveRoom, row_idx: usize, row_length: usize) {
    for col_idx in 0..row_length {
        generated_cave_room.set(row_idx, col_idx, RoomObject::Wall);
    }
}

fn add_walls_to_column(generated_cave_room: &mut CaveRoom, col_idx: usize, column_length: usize) {
    for row_idx in 0..column_length {
        generated_cave_room.set(row_idx, col_idx, RoomObject::Wall);
    }
}

pub fn add_walls(generated_cave_room: &mut CaveRoom) {
    let room_width = generated_cave_room.get_width();
    let room_height = generated_cave_room.get_height();

    add_walls_to_row(generated_cave_room, 0, room_width);
    add_walls_to_row(generated_cave_room, room_height - 1, room_width);

    add_walls_to_column(generated_cave_room, 0, room_height);
    add_walls_to_column(generated_cave_room, room_width - 1, room_height);
}

fn place_exit_door(generated_cave_room: &mut CaveRoom) {
    let mut random_number_generator = rand::rng();
    let row_width = generated_cave_room.get_width();
    let x = random_number_generator.random_range(1..row_width - 1);
    let y = row_width - 1;

    generated_cave_room.set(x, y, RoomObject::ExitDoor);
}

fn place_hidden_door_switch(
    generated_cave_room: &mut CaveRoom,
    claimed_tiles: &mut HashSet<LogicalCoordinates>,
) {
    let hidden_floor_switch_position = get_unique_room_location(generated_cave_room, claimed_tiles);
    generated_cave_room.set(
        hidden_floor_switch_position.get_x(),
        hidden_floor_switch_position.get_y(),
        RoomObject::HiddenFloorSwitch,
    );

    claimed_tiles.insert(hidden_floor_switch_position);
}

fn get_unique_room_location(
    generated_cave_room: &mut CaveRoom,
    claimed_tiles: &mut HashSet<LogicalCoordinates>,
) -> LogicalCoordinates {
    let mut random_number_generator = rand::rng();

    let room_width = generated_cave_room.get_width() - 1;
    let room_height = generated_cave_room.get_height() - 1;
    loop {
        let row_idx = random_number_generator.random_range(1..room_height);
        let col_idx = random_number_generator.random_range(1..room_width);

        let found_room_location = LogicalCoordinates::new(row_idx, col_idx);
        if !claimed_tiles.contains(&found_room_location) {
            return found_room_location;
        }
    }
}

fn place_treasure(
    generated_cave_room: &mut CaveRoom,
    claimed_tiles: &mut HashSet<LogicalCoordinates>,
) {
    let placement_percentage_decimal = 0.15;

    let num_tiles_to_consider = (generated_cave_room.get_width() - WALLS_OFFSET)
        * (generated_cave_room.get_height() - WALLS_OFFSET);
    let num_treasure_to_place =
        (num_tiles_to_consider as f32 * placement_percentage_decimal) as usize;

    for _i in 0..num_treasure_to_place {
        let treasure_location = get_unique_room_location(generated_cave_room, claimed_tiles);
        generated_cave_room.set(
            treasure_location.get_x(),
            treasure_location.get_y(),
            RoomObject::Treasure(100),
        );
        claimed_tiles.insert(treasure_location);
    }
}

fn place_armed_traps(
    generated_cave_room: &mut CaveRoom,
    claimed_tiles: &mut HashSet<LogicalCoordinates>,
) {
    let placement_percentage_decimal = 0.15;

    let num_tiles_to_consider = (generated_cave_room.get_width() - WALLS_OFFSET)
        * (generated_cave_room.get_height() - WALLS_OFFSET);
    let num_traps_to_place = (num_tiles_to_consider as f32 * placement_percentage_decimal) as usize;

    for _i in 0..num_traps_to_place {
        let trap_location = get_unique_room_location(generated_cave_room, claimed_tiles);
        generated_cave_room.set(
            trap_location.get_x(),
            trap_location.get_y(),
            RoomObject::Trap,
        );
        claimed_tiles.insert(trap_location);
    }
}

/// Returns the number - 1, or the lower bound if it cannot be subtracted.
fn calculate_lower_bound(number: usize, lower_bound: usize) -> usize {
    if number > lower_bound {
        number - 1
    } else {
        lower_bound
    }
}

/// Returns the number + 1, or the upper bound - 1 if it cannot be added.
fn calculate_upper_bound(number: usize, upper_bound: usize) -> usize {
    if number < upper_bound - 1 {
        number + 1
    } else {
        upper_bound - 1
    }
}

pub fn exclude_tiles_at_and_around(
    player_location: &LogicalCoordinates,
    room_dimensions: &WorldTileDimensions,
) -> HashSet<LogicalCoordinates> {
    let mut excluded_tiles = HashSet::new();

    let lower_bound_x = calculate_lower_bound(player_location.get_x(), 0);
    let upper_bound_x = calculate_upper_bound(player_location.get_x(), room_dimensions.get_width());

    let lower_bound_y = calculate_lower_bound(player_location.get_y(), 0);
    let upper_bound_y =
        calculate_upper_bound(player_location.get_y(), room_dimensions.get_height());

    for x in lower_bound_x..=upper_bound_x {
        for y in lower_bound_y..=upper_bound_y {
            let excluded_location = LogicalCoordinates::new(x, y);
            excluded_tiles.insert(excluded_location);
        }
    }

    excluded_tiles
}

/*
 * We don't want the size of the room to exceed the fixed window size
 * of 1280x720 because we do not move the camera with the explorer.
 *
 * The camera will not move with the explorer since we want to register
 * clicks from the stream even if there's high latency. Moving the camera
 * around would result in unintended actions otherwise.
 */
pub fn calculate_max_size(tile_sizing: &TileSize) -> WorldTileDimensions {
    let max_width = 1280 / tile_sizing.calculate_size();
    let max_height = 480 / tile_sizing.calculate_size();

    WorldTileDimensions::new(max_width, max_height)
}

/// Places an explorer in some cave room based on their desired location. If the location
/// is not within the room's bounds, it is adjusted based on the max room size.
///
/// Returns the set of tiles around the explorer.
pub fn place_explorer(
    cave_room: &mut CaveRoom,
    max_room_size: &WorldTileDimensions,
    desired_explorer_location: &LogicalCoordinates,
) -> HashSet<LogicalCoordinates> {
    let cave_room_dimensions = cave_room.get_dimensions();
    let max_room_width = max_room_size.get_width();

    let mut capped_x = desired_explorer_location.get_x() % max_room_size.get_width();
    // 1 <= capped_x < max_room_width - 1, where 1 represents a wall on some side.
    if capped_x == 0 {
        capped_x += 1;
    } else if capped_x == max_room_width - 1 {
        capped_x -= 1;
    }

    let capped_explorer_starting_location = LogicalCoordinates::new(capped_x, 1);
    // We should start by not add something at or around the explorer when they
    // first enter the room, such as a trap, or treasure, or even
    // the hidden door switch right away. It's unfair otherwise, as reported from
    // play testing.
    let claimed_tiles =
        exclude_tiles_at_and_around(&capped_explorer_starting_location, cave_room_dimensions);
    cave_room.set(
        capped_explorer_starting_location.get_x(),
        capped_explorer_starting_location.get_y(),
        RoomObject::Explorer,
    );

    claimed_tiles
}

impl RoomGenerating for RandomizedRoomGenerator {
    fn generate_empty_room(&mut self, room_number: usize) -> CaveRoom {
        // We need to account for walls, hence why all widths and heights need to be adjusted
        // by + 2.
        let min_width = self.min_size.get_width();
        let min_height = self.min_size.get_height();
        let desired_width = min_width + (room_number / min_width) + WALLS_OFFSET;
        let desired_height = min_height + (room_number / min_height) + WALLS_OFFSET;

        // TODO: Right now, we're rendering square rooms, so the height and
        // width should be equal. In the future with different shapes,
        // this will have to change.
        let mut height = desired_height % (self.max_size.get_height() + WALLS_OFFSET);
        if height == 0 {
            height = min_height + WALLS_OFFSET;
        }

        let mut width = desired_width % (self.max_size.get_width() + WALLS_OFFSET);
        if width == 0 {
            width = min_width + WALLS_OFFSET;
        }

        let mut generated_cave_room = CaveRoom::new(width, height);
        add_walls(&mut generated_cave_room);

        generated_cave_room
    }

    fn generate(
        &mut self,
        room_number: usize,
        desired_explorer_location: &LogicalCoordinates,
    ) -> CaveRoom {
        let mut generated_cave_room = self.generate_empty_room(room_number);

        let mut claimed_tiles = place_explorer(
            &mut generated_cave_room,
            &self.max_size,
            desired_explorer_location,
        );
        place_exit_door(&mut generated_cave_room);
        place_hidden_door_switch(&mut generated_cave_room, &mut claimed_tiles);
        place_treasure(&mut generated_cave_room, &mut claimed_tiles);
        place_armed_traps(&mut generated_cave_room, &mut claimed_tiles);

        generated_cave_room
    }
}
