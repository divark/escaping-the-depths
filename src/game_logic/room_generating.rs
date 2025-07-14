use crate::{
    CaveRoom, ExitDoorState, LogicalCoordinates, RoomObject, TrapState,
    game_logic::scores::TreasureScore,
};

use bevy::prelude::*;

use super::pathfinding::Graph;

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
}

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

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum ExplorerState {
    Alive,
    Dead,
    Traveling,
}

#[derive(Component)]
pub struct ExitDoor;

#[derive(Component)]
pub struct HiddenFloorSwitch;

fn spawn_room(cave_room: &CaveRoom, place_tile_broadcaster: &mut EventWriter<PlaceRoomObject>) {
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
}

fn spawn_objects_in_room(
    cave_room: &CaveRoom,
    place_tile_broadcaster: &mut EventWriter<PlaceRoomObject>,
) {
    let cave_room_objects = cave_room.get_objects();
    for room_object in cave_room_objects {
        let tile_type = room_object.get_type();
        let tile_coords = room_object.get_logical_coordinates();

        place_tile_broadcaster.write(PlaceRoomObject::new(
            *tile_type,
            tile_coords.get_x(),
            tile_coords.get_y(),
            1,
        ));
    }
}

fn spawn_centered_camera(cave_room: &CaveRoom, commands: &mut Commands) {
    let centered_on_map_camera = Camera2d::default();
    let cave_room_px_width = cave_room.get_width() * 16;
    let cave_room_px_height = cave_room.get_height() * 16;
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
        .map(|tile| *tile.get_logical_coordinates())
        .collect();

    let traversal_graph = Graph::from_tiles(&room_tile_locations, cave_room.get_dimensions());
    commands.spawn(traversal_graph);
}

pub fn spawn_new_room(
    mut spawn_room_requests: EventReader<ChangeRoom>,
    mut place_tile_broadcaster: EventWriter<PlaceRoomObject>,
    mut commands: Commands,
) {
    if spawn_room_requests.is_empty() {
        return;
    }

    let cave_room = spawn_room_requests.read().next().unwrap().get_room();
    spawn_room(cave_room, &mut place_tile_broadcaster);
    spawn_objects_in_room(cave_room, &mut place_tile_broadcaster);
    spawn_room_traversal_graph(cave_room, &mut commands);
    spawn_centered_camera(cave_room, &mut commands);
}

fn get_tile_sprite(tile_to_place: &PlaceRoomObject, asset_server: &AssetServer) -> Sprite {
    let tile_type = tile_to_place.object_type;
    let sprite_file_path = match tile_type {
        RoomObject::Empty => "environment/floor_plain.png",
        RoomObject::Explorer => "characters/npc_merchant_2.png",
        RoomObject::ExitDoor => "environment/door_closed.png",
        RoomObject::HiddenFloorSwitch => "environment/floor_mud_n_1.png",
        RoomObject::Treasure(_) => "environment/treasure.png",
        RoomObject::Trap => "environment/trap.png",
    };

    let spritesheet_image = asset_server.load(sprite_file_path);

    Sprite::from_image(spritesheet_image)
}

fn get_tile_position(tile_to_place: &PlaceRoomObject) -> Transform {
    let tile_size = 16;

    let tile_x = tile_to_place.x * tile_size;
    let tile_y = tile_to_place.y * tile_size;

    let tile_z = tile_to_place.z;
    Transform::from_xyz(tile_x as f32, tile_y as f32, tile_z as f32)
}

fn convert_to_rendered_tile(
    tile_to_place: &PlaceRoomObject,
    asset_server: &AssetServer,
) -> TileBundle {
    let mut tile_sprite_bundle = SpriteBundle::default();

    let tile_sprite = get_tile_sprite(tile_to_place, asset_server);
    tile_sprite_bundle.set_sprite(tile_sprite);

    let tile_position = get_tile_position(tile_to_place);
    tile_sprite_bundle.set_position(tile_position);

    let tile_logical_position = LogicalCoordinates::new(tile_to_place.x, tile_to_place.y);
    let tile_bundle = TileBundle::new(tile_sprite_bundle, tile_logical_position);

    tile_bundle
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
            explorer_state: ExplorerState::Alive,
        }
    }
}

pub fn place_tile(
    mut place_tile_requests: EventReader<PlaceRoomObject>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for tile_to_place in place_tile_requests.read() {
        let rendered_tile = convert_to_rendered_tile(tile_to_place, &asset_server);
        match tile_to_place.object_type {
            RoomObject::Explorer => {
                commands.spawn(ExplorerBundle::new(rendered_tile));
            }
            RoomObject::ExitDoor => {
                commands.spawn((rendered_tile, ExitDoorState::Closed));
            }
            RoomObject::HiddenFloorSwitch => {
                commands.spawn((rendered_tile, HiddenFloorSwitch));
            }
            RoomObject::Treasure(treasure_value) => {
                commands.spawn((rendered_tile, TreasureScore::new(treasure_value)));
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

pub fn unlock_exit_door_with_explorer(
    movement_changes: Query<
        &LogicalCoordinates,
        (With<ExplorerState>, Changed<LogicalCoordinates>),
    >,
    hidden_floor_switch: Query<&LogicalCoordinates, With<HiddenFloorSwitch>>,
    mut exit_door: Query<&mut ExitDoorState>,
) {
    if movement_changes.is_empty() || exit_door.is_empty() || hidden_floor_switch.is_empty() {
        return;
    }

    for movement_coordinates in movement_changes.iter() {
        let hidden_floor_switch_coordinates = hidden_floor_switch.single().expect(
            "unlock_hidden_door_with_explorer: Could not find the coordinates of the hidden floor switch.",
        );
        let mut exit_door_state = exit_door
            .single_mut()
            .expect("unlock_exit_door_with_explorer: Could not find the exit door.");

        if *movement_coordinates == *hidden_floor_switch_coordinates {
            *exit_door_state = ExitDoorState::Open;
        }
    }
}
