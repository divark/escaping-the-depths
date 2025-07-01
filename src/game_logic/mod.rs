pub mod room_generating;

use bevy::prelude::*;
use room_generating::{ChangeRoom, PlaceRoomObject, place_tile, spawn_new_room};

pub struct CoreLogic;

impl Plugin for CoreLogic {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeRoom>();
        app.add_event::<PlaceRoomObject>();

        app.add_systems(Update, spawn_new_room);
        app.add_systems(Update, place_tile);
    }
}
