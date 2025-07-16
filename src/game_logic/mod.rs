pub mod pathfinding;
pub mod room_generating;
pub mod scores;
pub mod traps;
pub mod viewer_interaction;

use std::time::Duration;

use bevy::prelude::*;
use pathfinding::{
    make_explorer_go_to_exit_door, make_explorer_wander, move_explorer_to_next_tile,
    set_explorer_target,
};
use room_generating::{
    ChangeRoom, LoadRoom, PlaceRoomObject, despawn_current_room, place_tile, spawn_new_room,
    spawn_next_room, unlock_exit_door_with_explorer,
};
use scores::{claim_treasure_with_explorer, claim_treasure_with_viewer_click, initialize_records};
use traps::{disarm_trap_with_viewer_click, hurt_explorer_with_armed_trap};
use viewer_interaction::{
    ViewerClick, convert_viewer_click_to_tile_click, unlock_exit_door_with_viewer_click,
};

use crate::{ExplorerHealth, LogicalCoordinates, RoomGenerating};

#[derive(Resource, Clone)]
pub struct MovementTime(Duration);

impl MovementTime {
    pub fn new(time: Duration) -> Self {
        Self(time)
    }

    pub fn get_timer(&self) -> Timer {
        Timer::new(self.0, TimerMode::Once)
    }
}

pub struct CoreLogic<T: RoomGenerating + Resource + Clone> {
    movement_time: MovementTime,
    room_generator: T,
}

impl<T: RoomGenerating + Resource + Clone> CoreLogic<T> {
    pub fn new(movement_time: MovementTime, room_generator: T) -> Self {
        Self {
            movement_time,
            room_generator,
        }
    }
}

impl<T: RoomGenerating + Resource + Clone> Plugin for CoreLogic<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadRoom>();
        app.add_event::<ChangeRoom>();
        app.add_event::<PlaceRoomObject>();
        app.add_event::<LogicalCoordinates>();
        app.add_event::<ViewerClick>();

        app.insert_resource(ExplorerHealth::new(3, 3));
        app.insert_resource(self.movement_time.clone());
        app.insert_resource(self.room_generator.clone());

        app.add_systems(Startup, initialize_records);
        app.add_systems(Update, despawn_current_room);
        app.add_systems(Update, spawn_new_room.after(despawn_current_room));
        app.add_systems(Update, place_tile.after(spawn_new_room));

        app.add_systems(Update, convert_viewer_click_to_tile_click);
        app.add_systems(Update, unlock_exit_door_with_viewer_click);
        app.add_systems(Update, claim_treasure_with_viewer_click);
        app.add_systems(Update, disarm_trap_with_viewer_click);

        app.add_systems(Update, make_explorer_wander);
        app.add_systems(
            Update,
            unlock_exit_door_with_explorer.after(make_explorer_wander),
        );
        app.add_systems(
            Update,
            make_explorer_go_to_exit_door.after(unlock_exit_door_with_explorer),
        );
        app.add_systems(Update, set_explorer_target);
        app.add_systems(Update, move_explorer_to_next_tile);

        app.add_systems(Update, spawn_next_room::<T>);

        app.add_systems(Update, claim_treasure_with_explorer);
        app.add_systems(Update, hurt_explorer_with_armed_trap);
    }
}
