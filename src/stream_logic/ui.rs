use bevy::prelude::*;

use crate::core_logic::scoring::CurrentRecords;

#[derive(Component)]
struct CurrentScoresUI;

#[derive(Component)]
struct HighScoresUI;

pub fn spawn_statistics_ui(mut commands: Commands) {
    let mut top_screen_scores = TopScreenUI::new();
    top_screen_scores.set_left(CurrentScoresUI);
    top_screen_scores.set_middle(HighScoresUI);

    let top_screen_scores_ui = top_screen_scores.render();
    commands.spawn(top_screen_scores_ui);
}

pub fn update_statistics_ui(
    mut current_records_ui: Query<&mut Text, With<CurrentScoresUI>>,
    mut highest_records_ui: Query<&mut Text, With<HighScoresUI>>,
    current_records: Query<&CurrentRecords, Changed<CurrentRecords>>,
) {
    let not_ready = current_records_ui.is_empty()
        || highest_records_ui.is_empty()
        || current_records.is_empty();

    if not_ready {
        return;
    }

    let mut current_record_ui = current_records_ui
        .single_mut()
        .expect("update_statistics_ui: Could not find UI for current records.");
    let mut highest_record_ui = highest_records_ui
        .single_mut()
        .expect("update_statistics_ui: Could not find UI for high records.");
    let current_record_info = current_records
        .single()
        .expect("update_statistics_ui: Could not find all scores recorded in the game so far.");

    current_record_ui.0 = std::format!(
        "Current Score: {}\nRooms Explored: {}",
        current_record_info.get_current_score(),
        current_record_info.get_current_room_number()
    );

    highest_record_ui.0 = std::format!(
        "High Score: {}\nMost Rooms Explored: {}",
        current_record_info.get_record_score(),
        current_record_info.get_record_score()
    );
}
