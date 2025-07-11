use bevy::prelude::*;

use crate::{ExplorerHealth, LogicalCoordinates, TrapState};

use super::room_generating::ExplorerState;

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
