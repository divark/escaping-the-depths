use bevy::{audio::PlaybackMode, prelude::*};

use crate::{
    core_logic::{
        scoring::{TrapState, TreasureState},
        traveling::ExitDoorState,
    },
    stream_logic::ui::BonusScoreState,
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
    // During playtesting, it was found that a player could either claim treasure
    // or trigger a trap after it was already despawned, but the game did not know
    // about it, causing it to crash.
    //
    // This is a workaround in the event that the entity does not exist anymore.
    commands.entity(door_entity).try_insert(door_opening_sfx);
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
        commands.entity(trap_entity).try_insert(trap_triggering_sfx);
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
        commands
            .entity(treasure_entity)
            .insert(treasure_claimed_sfx);
    }
}

pub fn trigger_bonus_score_noise(
    bonus_score_ui: Query<
        (Entity, &BonusScoreState),
        (Added<BonusScoreState>, Without<AudioPlayer>),
    >,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if bonus_score_ui.is_empty() {
        return;
    }

    let (bonus_score_ui_entity, bonus_score_state) = bonus_score_ui.single().unwrap();

    let no_armed_traps_triggered = bonus_score_state == &BonusScoreState::NoTrapsTriggered;
    let bonus_score_sound_file = if no_armed_traps_triggered {
        asset_server.load("ui/VictoryBig_fixed.wav")
    } else {
        asset_server.load("ui/VictorySmall_fixed.wav")
    };

    let bonus_score_playback_settings = PlaybackSettings {
        mode: PlaybackMode::Remove,
        ..default()
    };

    let bonus_score_sfx = AudioPlayer::new(bonus_score_sound_file);
    commands
        .entity(bonus_score_ui_entity)
        .try_insert((bonus_score_sfx, bonus_score_playback_settings));
}
