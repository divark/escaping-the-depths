pub mod interacting;
pub mod setting;
pub mod traveling;

use std::time::Duration;

use bevy::prelude::*;
use interacting::ViewerClick;

#[derive(States, Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    Start,
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

pub struct CoreLogic {
    movement_time: MovementTime,
    game_over_time: GameOverTime,
}

impl CoreLogic {
    pub fn new(movement_time: MovementTime, game_over_time: GameOverTime) -> Self {
        Self {
            movement_time,
            game_over_time,
        }
    }
}

impl Plugin for CoreLogic {
    fn build(&self, app: &mut App) {
        app.add_message::<ViewerClick>();

        app.init_state::<GameState>();
        app.insert_resource(self.movement_time.clone());
        app.insert_resource(self.game_over_time.clone());
    }
}
