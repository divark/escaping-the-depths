use bevy::prelude::*;

use crate::core_logic::scoring::{CurrentRecords, ExplorerHealth};

#[derive(Component)]
pub struct CurrentScoresUI;

#[derive(Component)]
pub struct HighScoresUI;

#[derive(Bundle)]
pub struct TextSection<C: Component> {
    text: Text,
    font: TextFont,
    text_layout: TextLayout,
    container: Node,
    label: C,
}

pub const CONTAINER_WIDTH_PERCENTAGE: f32 = ((1280.0 / 3.0) / 1280.0) * 100.0;

impl<C: Component> TextSection<C> {
    pub fn new(font_size: usize, label: C) -> Self {
        let container = Node {
            height: Val::Percent(100.0),
            flex_grow: 1.0,
            ..default()
        };

        Self {
            text: Text::new(""),
            font: TextFont::from_font_size(font_size as f32),
            text_layout: TextLayout::new_with_justify(JustifyText::Center),
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
    pub fn render(&self, whole_screen_entity: Entity, commands: &mut Commands) {
        let top_half_screen = Node {
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            ..default()
        };

        let score_bar = Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            ..default()
        };

        let current_scores_section = TextSection::new(self.font_size, CurrentScoresUI);
        let top_scores_section = TextSection::new(self.font_size, HighScoresUI);

        commands
            .entity(whole_screen_entity)
            .with_children(|screen| {
                screen
                    .spawn(top_half_screen)
                    .with_children(|top_half_screen| {
                        top_half_screen.spawn(score_bar).with_children(|score_bar| {
                            score_bar.spawn(current_scores_section);
                            score_bar.spawn(top_scores_section);
                        });
                    });
            });
    }
}

#[derive(Component)]
pub struct WholeScreen;

pub fn prepare_screen_ui(mut commands: Commands) {
    let whole_screen = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    };

    commands.spawn((whole_screen, WholeScreen));
}

pub fn spawn_statistics_ui(mut commands: Commands, whole_screen: Query<Entity, With<WholeScreen>>) {
    if whole_screen.is_empty() {
        return;
    }

    let whole_screen = whole_screen
        .single()
        .expect("spawn_statistics_ui: Could not find Node for whole screen.");
    let font_size = 20;

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

#[derive(Component)]
pub struct HealthUI;

pub fn spawn_health_ui(
    mut commands: Commands,
    whole_screen: Query<Entity, With<WholeScreen>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    if whole_screen.is_empty() {
        return;
    }

    let whole_screen_entity = whole_screen
        .single()
        .expect("spawn_health_ui: Could not find whole screen UI Node.");

    let health_bar_image = asset_server.load("ui/health-bar-atlas.png");
    let health_bar_atlas = TextureAtlasLayout::from_grid(UVec2::new(54, 17), 1, 4, None, None);
    let health_bar_atlas_handle = texture_atlases.add(health_bar_atlas);

    let bottom_half_screen = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(50.0),
        flex_direction: FlexDirection::ColumnReverse,
        ..default()
    };

    let health_bar_row = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(CONTAINER_WIDTH_PERCENTAGE),
        flex_direction: FlexDirection::Row,
        ..default()
    };

    let health_bar_ui = ImageNode::from_atlas_image(
        health_bar_image,
        TextureAtlas::from(health_bar_atlas_handle),
    );
    commands
        .entity(whole_screen_entity)
        .with_children(|whole_screen_node| {
            whole_screen_node
                .spawn(bottom_half_screen)
                .with_children(|bottom_half_screen| {
                    bottom_half_screen
                        .spawn(health_bar_row)
                        .with_children(|health_bar_row| {
                            health_bar_row.spawn((health_bar_ui, HealthUI));
                        });
                });
        });
}

pub fn update_health_ui(
    mut health_ui: Query<&mut ImageNode, With<HealthUI>>,
    current_health: Res<ExplorerHealth>,
) {
    let not_ready = health_ui.is_empty() || !current_health.is_changed();
    if not_ready {
        return;
    }

    let mut health_ui_pack = health_ui
        .single_mut()
        .expect("update_health_ui: Could not find Health UI.");

    let health_atlas_idx = current_health.get_current_health();
    if let Some(health_texture_atlas) = &mut health_ui_pack.texture_atlas {
        health_texture_atlas.index = health_atlas_idx;
    }
}
