use bevy::prelude::*;

use crate::core_logic::scoring::CurrentRecords;

#[derive(Component)]
pub struct CurrentScoresUI;

#[derive(Component)]
pub struct HighScoresUI;

#[derive(Bundle)]
pub struct TextSection<C: Component> {
    text: Text,
    font: TextFont,
    container: Node,
    label: C,
}

impl<C: Component> TextSection<C> {
    pub fn new(font_size: usize, label: C) -> Self {
        let container_width_percentage = ((1280.0 / 3.0) / 1280.0) * 100.0;

        let container = Node {
            width: Val::Percent(container_width_percentage),
            height: Val::Percent(100.0),
            ..default()
        };

        Self {
            text: Text::new(""),
            font: TextFont::from_font_size(font_size as f32),
            container,
            label,
        }
    }
}

struct ScoresUI {
    font_size: usize,
}

impl ScoresUI {
    pub fn new(font_size: usize) -> Self {
        Self { font_size }
    }

    // Places the Scores UI, represented as a row of current and top scores
    // at the top of the screen.
    pub fn render(&self, whole_screen: Node, commands: &mut Commands) {
        let height_percentage = (120.0 / 720.0) * 100.0;

        let score_bar = Node {
            width: Val::Percent(100.0),
            height: Val::Percent(height_percentage),
            flex_direction: FlexDirection::Row,
            ..default()
        };

        let current_scores_section = TextSection::new(self.font_size, CurrentScoresUI);
        let top_scores_section = TextSection::new(self.font_size, HighScoresUI);

        commands.spawn(whole_screen).with_children(|screen| {
            screen.spawn(score_bar).with_children(|score_bar| {
                score_bar.spawn(current_scores_section);
                score_bar.spawn(top_scores_section);
            });
        });
    }
}

pub fn spawn_statistics_ui(mut commands: Commands) {
    let whole_screen = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    };

    let font_size = 14;

    let records_ui_bar = ScoresUI::new(font_size);
    records_ui_bar.render(whole_screen, &mut commands);
}

pub fn update_statistics_ui(
    mut current_records_ui: Query<&mut Text, (With<CurrentScoresUI>, Without<HighScoresUI>)>,
    mut highest_records_ui: Query<&mut Text, (With<HighScoresUI>, Without<CurrentScoresUI>)>,
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
        current_record_info.get_record_room_number()
    );
}
