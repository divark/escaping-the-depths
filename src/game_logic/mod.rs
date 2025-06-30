pub mod room_generating;

use bevy::prelude::*;
use room_generating::{ChangeRoom, PlaceRoomObject};

pub struct CoreLogic;

impl Plugin for CoreLogic {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeRoom>();
        app.add_event::<PlaceRoomObject>();
    }
}
