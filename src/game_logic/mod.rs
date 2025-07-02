pub mod room_generating;
pub mod viewer_interaction;

use bevy::prelude::*;
use room_generating::{
    ChangeRoom, PlaceRoomObject, broadcast_location_when_explorer_moves, place_tile,
    spawn_new_room, unlock_exit_door,
};
use viewer_interaction::{ViewerClick, convert_viewer_click_to_tile_click};

use crate::LogicalCoordinates;

pub struct CoreLogic;

impl Plugin for CoreLogic {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeRoom>();
        app.add_event::<PlaceRoomObject>();
        app.add_event::<LogicalCoordinates>();
        app.add_event::<ViewerClick>();

        app.add_systems(Update, spawn_new_room);
        app.add_systems(
            Update,
            (broadcast_location_when_explorer_moves, unlock_exit_door),
        );
        app.add_systems(Update, convert_viewer_click_to_tile_click);
        app.add_systems(Update, place_tile);
    }
}
