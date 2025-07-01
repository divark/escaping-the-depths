use crate::{CaveRoom, ExitDoorState, LogicalCoordinates, RoomObject};

use bevy::prelude::*;

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

#[derive(Component)]
pub enum ExplorerState {
    Alive,
    Dead,
}

#[derive(Component)]
pub struct ExitDoor;

#[derive(Component)]
pub struct HiddenFloorSwitch;

pub fn spawn_new_room(
    mut spawn_room_requests: EventReader<ChangeRoom>,
    mut place_tile_broadcaster: EventWriter<PlaceRoomObject>,
) {
    if spawn_room_requests.is_empty() {
        return;
    }

    let cave_room = spawn_room_requests.read().next().unwrap().get_room();
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

fn get_tile_sprite(tile_to_place: &PlaceRoomObject, asset_server: &AssetServer) -> Sprite {
    let tile_type = tile_to_place.object_type;
    let sprite_file_path = match tile_type {
        RoomObject::Empty => "environment/floor_plain.png",
        RoomObject::Explorer => "characters/npc_merchant_2.png",
        RoomObject::ExitDoor => "environment/door_closed.png",
        RoomObject::HiddenFloorSwitch => "environment/floor_mud_n_1.png",
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

pub fn place_tile(
    mut place_tile_requests: EventReader<PlaceRoomObject>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for tile_to_place in place_tile_requests.read() {
        let rendered_tile = convert_to_rendered_tile(tile_to_place, &asset_server);
        match tile_to_place.object_type {
            RoomObject::Explorer => {
                commands.spawn((rendered_tile, ExplorerState::Alive));
            }
            RoomObject::ExitDoor => {
                commands.spawn((rendered_tile, ExitDoorState::Closed));
            }
            RoomObject::HiddenFloorSwitch => {
                commands.spawn((rendered_tile, HiddenFloorSwitch));
            }
            _ => {
                commands.spawn(rendered_tile);
            }
        }
    }
}

pub fn unlock_exit_door(
    explorer: Query<&LogicalCoordinates, (With<ExplorerState>, Changed<LogicalCoordinates>)>,
    hidden_floor_switch: Query<&LogicalCoordinates, With<HiddenFloorSwitch>>,
    mut exit_door: Query<&mut ExitDoorState>,
) {
    if explorer.is_empty() || exit_door.is_empty() || hidden_floor_switch.is_empty() {
        return;
    }

    let explorer_logical_coordinates = explorer
        .single()
        .expect("unlock_exit_door: Could not find the explorer.");
    let hidden_floor_switch_coordinates = hidden_floor_switch
        .single()
        .expect("unlock_hidden_door: Could not find the coordinates of the hidden floor switch.");
    let mut exit_door_state = exit_door
        .single_mut()
        .expect("unlock_exit_door: Could not find the exit door.");

    if *explorer_logical_coordinates == *hidden_floor_switch_coordinates {
        *exit_door_state = ExitDoorState::Open;
    }
}
