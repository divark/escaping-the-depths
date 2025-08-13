use bevy::prelude::*;

use crate::core_logic::{scoring::TrapState, traveling::ExitDoorState};

pub fn animate_door_opening(
    mut exit_door_query: Query<(&mut Sprite, &ExitDoorState), Changed<ExitDoorState>>,
) {
    if exit_door_query.is_empty() {
        return;
    }

    let (mut exit_door_sprite, exit_door_state) = exit_door_query.single_mut().unwrap();
    if exit_door_state != &ExitDoorState::Open {
        return;
    }

    if let Some(exit_door_atlas) = &mut exit_door_sprite.texture_atlas {
        exit_door_atlas.index = 1;
    }
}

pub fn animate_disarming_trap(mut all_traps: Query<(&mut Sprite, &TrapState), Changed<TrapState>>) {
    for (mut trap_sprite, trap_state) in &mut all_traps {
        if trap_state != &TrapState::Unarmed {
            continue;
        }

        if let Some(trap_atlas) = &mut trap_sprite.texture_atlas {
            trap_atlas.index = 1;
        }
    }
}
