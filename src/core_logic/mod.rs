pub mod interacting;
pub mod progressing;
pub mod setting;
pub mod traveling;

use std::time::Duration;

use bevy::prelude::*;
use interacting::ViewerClick;

use crate::core_logic::{
    interacting::ObjectiveAttempt,
    progressing::{
        HungerBar, HungerBarTime, decrease_hunger_bar_over_time, determine_campers_state,
        load_map_objectives, record_camper_contribution, spawn_hunger_bar,
    },
    setting::{ChangeMap, LoadMap, load_tiled_map, unload_current_map},
};

#[derive(States, Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub enum CampersState {
    Start,
    #[default]
    Alive,
    Dead,
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

pub struct CoreLogic {
    movement_time: MovementTime,
    game_over_time: GameOverTime,
    hunger_bar_decrease_time: HungerBarTime,
}

impl CoreLogic {
    pub fn new(
        movement_time: MovementTime,
        game_over_time: GameOverTime,
        hunger_bar_decrease_time: HungerBarTime,
    ) -> Self {
        Self {
            movement_time,
            game_over_time,
            hunger_bar_decrease_time,
        }
    }
}

impl Plugin for CoreLogic {
    fn build(&self, app: &mut App) {
        app.add_message::<ObjectiveAttempt>();
        app.add_message::<ViewerClick>();
        app.add_message::<LoadMap>();
        app.add_message::<ChangeMap>();

        app.init_state::<CampersState>();
        app.insert_resource(self.movement_time.clone());
        app.insert_resource(self.game_over_time.clone());

        app.insert_resource(self.hunger_bar_decrease_time.clone());
        app.insert_resource(HungerBar::default());
        app.add_systems(Startup, spawn_hunger_bar);
        app.add_systems(Update, decrease_hunger_bar_over_time);
        app.add_systems(
            Update,
            determine_campers_state.after(decrease_hunger_bar_over_time),
        );

        app.add_systems(Update, (unload_current_map, load_tiled_map));
        app.add_systems(Update, load_map_objectives.after(load_tiled_map));

        app.add_systems(Update, record_camper_contribution);
    }
}
