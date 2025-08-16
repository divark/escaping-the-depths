pub mod interacting;
pub mod scoring;
pub mod setting;
pub mod traveling;

use std::time::Duration;

use bevy::prelude::*;
use interacting::{ViewerClick, convert_viewer_click_to_tile_click};
use scoring::{
    ExplorerHealth, claim_treasure_with_explorer, claim_treasure_with_viewer_click,
    disarm_trap_with_viewer_click, hurt_explorer_with_armed_trap, initialize_records,
    start_game_over_countdown_on_death,
};
use setting::{
    ChangeRoom, LoadRoom, LogicalCoordinates, PlaceRoomObject, RoomGenerating,
    despawn_current_room, place_tile, reset_to_level_one_after_game_over, spawn_new_room,
    spawn_next_room,
};
use traveling::{
    make_explorer_go_to_exit_door, make_explorer_wander, move_explorer_to_next_tile,
    set_explorer_target, unlock_exit_door_with_explorer, unlock_exit_door_with_viewer_click,
};

pub const TILE_SIZE: usize = 16;

#[derive(States, Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Active,
    GameOver,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum TimerType {
    GameOver,
}

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

#[derive(Resource, Clone)]
pub struct GameOverTime(Duration);

impl GameOverTime {
    pub fn new(time: Duration) -> Self {
        Self(time)
    }

    pub fn get_timer(&self) -> Timer {
        Timer::new(self.0, TimerMode::Once)
    }
}

#[derive(Component)]
pub struct GameOverTimer(Timer);

impl GameOverTimer {
    pub fn new(game_over_time: &GameOverTime) -> Self {
        Self(game_over_time.get_timer())
    }

    pub fn get_timer_mut(&mut self) -> &mut Timer {
        &mut self.0
    }

    pub fn get_timer(&self) -> &Timer {
        &self.0
    }
}

pub struct CoreLogic<T: RoomGenerating + Resource + Clone> {
    movement_time: MovementTime,
    game_over_time: GameOverTime,
    room_generator: T,
}

impl<T: RoomGenerating + Resource + Clone> CoreLogic<T> {
    pub fn new(
        movement_time: MovementTime,
        game_over_time: GameOverTime,
        room_generator: T,
    ) -> Self {
        Self {
            movement_time,
            game_over_time,
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

        app.init_state::<GameState>();
        app.insert_resource(ExplorerHealth::new(3, 3));
        app.insert_resource(self.movement_time.clone());
        app.insert_resource(self.game_over_time.clone());
        app.insert_resource(self.room_generator.clone());

        app.add_systems(Startup, initialize_records);
        app.add_systems(Update, despawn_current_room);
        app.add_systems(Update, spawn_new_room.after(despawn_current_room));
        app.add_systems(Update, place_tile.after(spawn_new_room));
        app.add_systems(Update, spawn_next_room::<T>);

        let clicking_systems = (
            convert_viewer_click_to_tile_click,
            unlock_exit_door_with_viewer_click,
            claim_treasure_with_viewer_click,
            disarm_trap_with_viewer_click,
        );
        app.add_systems(Update, clicking_systems.run_if(in_state(GameState::Active)));

        let automatic_behavior_systems = (
            make_explorer_wander,
            unlock_exit_door_with_explorer.after(make_explorer_wander),
            make_explorer_go_to_exit_door.after(unlock_exit_door_with_explorer),
            set_explorer_target,
            move_explorer_to_next_tile,
            claim_treasure_with_explorer,
            hurt_explorer_with_armed_trap,
            start_game_over_countdown_on_death,
        );
        app.add_systems(
            Update,
            automatic_behavior_systems.run_if(in_state(GameState::Active)),
        );

        app.add_systems(
            Update,
            reset_to_level_one_after_game_over::<T>.run_if(in_state(GameState::GameOver)),
        );
    }
}
