use bevy::prelude::*;

use crate::core_logic::{
    scoring::{TrapState, TreasureState},
    traveling::ExitDoorState,
};

pub fn trigger_door_opening_noise(
    door_state: Query<(Entity, &ExitDoorState), Changed<ExitDoorState>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if door_state.is_empty() {
        return;
    }

    let (door_entity, door_state) = door_state.single().unwrap();
    if *door_state != ExitDoorState::Open {
        return;
    }

    let door_opening_sound_file = asset_server.load("environment/Explosion3__007.wav");
    let door_opening_sfx = AudioPlayer::new(door_opening_sound_file);
    commands.entity(door_entity).insert(door_opening_sfx);
}

pub fn trigger_trap_going_off_noise(
    trap_state: Query<(Entity, &TrapState), Changed<TrapState>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (trap_entity, trap_state) in trap_state.iter() {
        if *trap_state != TrapState::Unarmed {
            continue;
        }

        let trap_triggering_sound_file = asset_server.load("environment/Hi-hat__003.wav");
        let trap_triggering_sfx = AudioPlayer::new(trap_triggering_sound_file);

        // During playtesting, it was found that a player could either claim treasure
        // or trigger a trap after it was already despawned, but the game did not know
        // about it, causing it to crash.
        //
        // This is a workaround in the event that the entity does not exist anymore.
        if let Ok(mut found_trap_entity) = commands.get_entity(trap_entity) {
            found_trap_entity.insert(trap_triggering_sfx);
        }
    }
}

pub fn trigger_treasure_claimed_noise(
    treasures: Query<(Entity, &TreasureState), Changed<TreasureState>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (treasure_entity, treasure_state) in treasures.iter() {
        if *treasure_state != TreasureState::Claimed {
            continue;
        }

        let treasure_claimed_sound_file = asset_server.load("environment/sfx_coin_double7.wav");
        let treasure_claimed_sfx = AudioPlayer::new(treasure_claimed_sound_file);
        // During playtesting, it was found that a player could either claim treasure
        // or trigger a trap after it was already despawned, but the game did not know
        // about it, causing it to crash.
        //
        // This is a workaround in the event that the entity does not exist anymore.
        if let Ok(mut found_treasure_entity) = commands.get_entity(treasure_entity) {
            found_treasure_entity.insert(treasure_claimed_sfx);
        }
    }
}
