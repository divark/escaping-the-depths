use bevy::prelude::*;

use crate::{CurrentRecords, LogicalCoordinates};

use super::room_generating::ExplorerState;

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

pub fn initialize_records(mut commands: Commands) {
    let current_records = CurrentRecords::default();
    commands.spawn(current_records);
}

pub fn claim_treasure_with_explorer(
    explorer_movement: Query<
        &LogicalCoordinates,
        (With<ExplorerState>, Changed<LogicalCoordinates>),
    >,
    treasures: Query<(Entity, &LogicalCoordinates, &TreasureScore)>,
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
        for (treasure_entity, treasure_location, treasure_score) in treasures {
            if treasure_location != explorer_location {
                continue;
            }

            current_records.add_score(treasure_score.value());
            commands.entity(treasure_entity).despawn();
        }
    }
}
