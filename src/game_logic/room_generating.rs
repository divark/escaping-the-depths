use crate::{CaveRoom, RoomObject};

use bevy::prelude::*;

#[derive(Event)]
pub struct ChangeRoom(CaveRoom);

impl ChangeRoom {
    pub fn new(room_to_spawn: CaveRoom) -> Self {
        Self(room_to_spawn)
    }
}

#[derive(Event)]
pub struct PlaceRoomObject {
    object_type: RoomObject,
    x: usize,
    y: usize,
}

impl PlaceRoomObject {
    pub fn new(object_type: RoomObject, x: usize, y: usize) -> Self {
        Self { object_type, x, y }
    }
}
