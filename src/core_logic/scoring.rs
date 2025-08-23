use bevy::prelude::*;

use super::{
    GameOverTime, GameOverTimer, GameState,
    setting::{ExplorerState, LogicalCoordinates},
};

#[derive(Component, Debug)]
pub struct CurrentRecords {
    current_score: usize,
    current_room_number: usize,

    record_score: usize,
    record_room_number: usize,
}

impl CurrentRecords {
    pub fn get_current_score(&self) -> usize {
        self.current_score
    }

    pub fn get_current_room_number(&self) -> usize {
        self.current_room_number
    }

    pub fn set_current_room_number(&mut self, new_room_number: usize) {
        self.current_room_number = new_room_number;
    }

    pub fn add_score(&mut self, score_to_add: usize) {
        self.current_score += score_to_add;
    }

    pub fn set_current_score(&mut self, new_score: usize) {
        self.current_score = new_score;
    }

    pub fn get_record_score(&self) -> usize {
        self.record_score
    }

    pub fn increment_room_count(&mut self) {
        self.current_room_number += 1;
    }

    pub fn get_record_room_number(&self) -> usize {
        self.record_room_number
    }

    /// Records the current records and sets all current stats back to their
    /// defaults.
    pub fn reset(&mut self) {
        self.record_score = self.record_score.max(self.current_score);
        self.record_room_number = self.record_room_number.max(self.current_room_number);

        self.current_score = 0;
        self.current_room_number = 1;
    }
}

impl Default for CurrentRecords {
    fn default() -> Self {
        Self {
            current_score: 0,
            current_room_number: 1,

            record_score: 0,
            record_room_number: 1,
        }
    }
}

#[derive(Resource, Debug, PartialEq)]
pub struct ExplorerHealth {
    current: usize,
    total: usize,
}

impl ExplorerHealth {
    pub fn new(current: usize, total: usize) -> Self {
        Self { current, total }
    }

    pub fn get_current_health(&self) -> usize {
        self.current
    }

    pub fn set_current_health(&mut self, current_health: usize) {
        self.current = current_health;
    }

    pub fn set_total_health(&mut self, total_health: usize) {
        self.total = total_health;
    }

    pub fn decrease_current_health(&mut self) {
        if self.current != 0 {
            self.current -= 1;
        }
    }

    pub fn get_total_health(&self) -> usize {
        self.total
    }
}

#[derive(Component)]
pub struct TreasureScore {
    value: usize,
}

impl TreasureScore {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

#[derive(Clone, Copy, Debug, Component, PartialEq)]
pub enum TrapState {
    Armed,
    Unarmed,
}

pub fn initialize_records(mut commands: Commands) {
    let current_records = CurrentRecords::default();
    commands.spawn(current_records);
}

#[derive(Clone, Copy, Debug, Component, PartialEq)]
pub enum TreasureState {
    Unclaimed,
    Claimed,
}

pub fn claim_treasure_with_explorer(
    explorer_movement: Query<
        &LogicalCoordinates,
        (With<ExplorerState>, Changed<LogicalCoordinates>),
    >,
    mut treasures: Query<(
        Entity,
        &LogicalCoordinates,
        &TreasureScore,
        &mut TreasureState,
    )>,
    mut records: Query<&mut CurrentRecords>,
    mut commands: Commands,
) {
    if treasures.is_empty() || records.is_empty() {
        return;
    }

    let mut current_records = records
        .single_mut()
        .expect("claim_treasure_with_explorer: Could not find current records.");

    for explorer_location in explorer_movement.iter() {
        for (treasure_entity, treasure_location, treasure_score, mut treasure_state) in
            treasures.iter_mut()
        {
            if treasure_location != explorer_location {
                continue;
            }

            if *treasure_state != TreasureState::Unclaimed {
                continue;
            }

            current_records.add_score(treasure_score.value());

            *treasure_state = TreasureState::Claimed;
            let invisibility = Visibility::Hidden;
            commands.entity(treasure_entity).insert(invisibility);
        }
    }
}

pub fn claim_treasure_with_viewer_click(
    mut explorer_movement: EventReader<LogicalCoordinates>,
    mut treasures: Query<(
        Entity,
        &LogicalCoordinates,
        &TreasureScore,
        &mut TreasureState,
    )>,
    mut records: Query<&mut CurrentRecords>,
    mut commands: Commands,
) {
    if treasures.is_empty() || records.is_empty() {
        return;
    }

    let mut current_records = records
        .single_mut()
        .expect("claim_treasure_with_viewer_click: Could not find current records.");

    for explorer_location in explorer_movement.read() {
        for (treasure_entity, treasure_location, treasure_score, mut treasure_state) in
            treasures.iter_mut()
        {
            if treasure_location != explorer_location {
                continue;
            }

            if *treasure_state != TreasureState::Unclaimed {
                continue;
            }

            current_records.add_score(treasure_score.value());

            *treasure_state = TreasureState::Claimed;
            let invisibility = Visibility::Hidden;
            // During playtesting, it was found that a player could either claim treasure
            // or trigger a trap after it was already despawned, but the game did not know
            // about it, causing it to crash.
            //
            // This is a workaround in the event that the entity does not exist anymore.
            if let Ok(mut found_treasure_entity) = commands.get_entity(treasure_entity) {
                found_treasure_entity.insert(invisibility);
            }
        }
    }
}

pub fn hurt_explorer_with_armed_trap(
    mut explorer_health: ResMut<ExplorerHealth>,
    mut explorer: Query<
        (&LogicalCoordinates, &mut ExplorerState),
        (Changed<LogicalCoordinates>, With<ExplorerState>),
    >,
    mut traps: Query<(&LogicalCoordinates, &mut TrapState)>,
) {
    if explorer.is_empty() || traps.is_empty() {
        return;
    }

    let (explorer_location, mut explorer_state) = explorer
        .single_mut()
        .expect("hurt_explorer_with_armed_trap: Could not find Explorer.");

    for (trap_location, mut trap_state) in traps.iter_mut() {
        if trap_location != explorer_location {
            continue;
        }

        if *trap_state != TrapState::Armed {
            continue;
        }

        explorer_health.decrease_current_health();
        *trap_state = TrapState::Unarmed;

        if explorer_health.get_current_health() == 0 {
            *explorer_state = ExplorerState::Dead;
        }
    }
}

pub fn start_game_over_countdown_on_death(
    explorer_health: Res<ExplorerHealth>,
    game_over_time: Res<GameOverTime>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    if explorer_health.get_current_health() != 0 {
        return;
    }

    let game_over_timer = GameOverTimer::new(&game_over_time);
    commands.spawn(game_over_timer);

    next_game_state.set(GameState::GameOver);
}

pub fn disarm_trap_with_viewer_click(
    mut viewer_tiles_clicked: EventReader<LogicalCoordinates>,
    mut traps: Query<(&LogicalCoordinates, &mut TrapState)>,
) {
    if traps.is_empty() {
        return;
    }

    for viewer_tile_clicked in viewer_tiles_clicked.read() {
        for (trap_tile, mut trap_state) in traps.iter_mut() {
            if trap_tile != viewer_tile_clicked {
                continue;
            }

            *trap_state = TrapState::Unarmed;
        }
    }
}
