use std::time::Duration;

use bevy::prelude::*;

use crate::core_logic::{
    GameOverTimer, GameState,
    scoring::{CurrentRecords, ExplorerHealth, TrapState},
    setting::{ExplorerState, LogicalCoordinates},
    traveling::ExitDoorState,
};

#[derive(Component)]
pub struct CurrentScoresUI;

#[derive(Component)]
pub struct HighScoresUI;

#[derive(Bundle)]
pub struct TextSection<C: Component> {
    text: Text,
    font: TextFont,
    text_layout: TextLayout,
    label: C,
}

pub const CONTAINER_WIDTH_PERCENTAGE: f32 = ((1280.0 / 3.0) / 1280.0) * 100.0;

impl<C: Component> TextSection<C> {
    pub fn new(font_size: usize, label: C) -> Self {
        Self {
            text: Text::new(""),
            font: TextFont::from_font_size(font_size as f32),
            text_layout: TextLayout::new_with_justify(JustifyText::Center),
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
    pub fn render(&self, top_half_screen: Entity, commands: &mut Commands) {
        let score_bar = Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            ..default()
        };

        let score_container = Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_grow: 1.0,
            ..default()
        };

        let current_scores_section = TextSection::new(self.font_size, CurrentScoresUI);
        let top_scores_section = TextSection::new(self.font_size, HighScoresUI);

        commands
            .entity(top_half_screen)
            .with_children(|top_half_screen| {
                top_half_screen.spawn(score_bar).with_children(|score_bar| {
                    score_bar
                        .spawn(score_container.clone())
                        .with_children(|score_container| {
                            score_container.spawn(current_scores_section);
                        });
                    score_bar
                        .spawn(score_container.clone())
                        .with_children(|score_container| {
                            score_container.spawn(top_scores_section);
                        });
                });
            });
    }
}

#[derive(Component)]
pub struct WholeScreen;

#[derive(Component)]
pub struct TopHalfScreen;

#[derive(Component)]
pub struct BottomHalfScreen;

pub fn prepare_screen_ui(mut commands: Commands) {
    let whole_screen = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    };

    let top_half_screen = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(50.0),
        flex_direction: FlexDirection::Column,
        ..default()
    };

    let bottom_half_screen = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(50.0),
        flex_direction: FlexDirection::ColumnReverse,
        ..default()
    };

    commands
        .spawn((whole_screen, WholeScreen))
        .with_children(|whole_screen| {
            whole_screen.spawn((top_half_screen, TopHalfScreen));
            whole_screen.spawn((bottom_half_screen, BottomHalfScreen));
        });
}

pub fn spawn_statistics_ui(
    mut commands: Commands,
    top_half_screen: Query<Entity, With<TopHalfScreen>>,
) {
    if top_half_screen.is_empty() {
        return;
    }

    let top_half_screen_entity = top_half_screen
        .single()
        .expect("spawn_statistics_ui: Could not find Node for top half of the screen.");
    let font_size = 36;

    let records_ui_bar = ScoresUI::new(font_size);
    records_ui_bar.render(top_half_screen_entity, &mut commands);
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
    bottom_half_screen: Query<Entity, With<BottomHalfScreen>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    if bottom_half_screen.is_empty() {
        return;
    }

    let bottom_half_screen_entity = bottom_half_screen
        .single()
        .expect("spawn_health_ui: Could not find bottom half screen UI Node.");

    let health_bar_image = asset_server.load("ui/health-bar-atlas.png");
    let health_bar_atlas = TextureAtlasLayout::from_grid(UVec2::new(54, 17), 1, 4, None, None);
    let health_bar_atlas_handle = texture_atlases.add(health_bar_atlas);

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
        .entity(bottom_half_screen_entity)
        .with_children(|bottom_half_screen| {
            bottom_half_screen
                .spawn(health_bar_row)
                .with_children(|health_bar_row| {
                    health_bar_row.spawn((health_bar_ui, HealthUI));
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

#[derive(Component)]
pub struct GameOverScreen;

fn spawn_game_over_ui(font_size: usize, commands: &mut Commands) {
    let text = TextSection::new(font_size, GameOverScreen);
    let visibility = Visibility::Hidden;

    let game_over_screen = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };

    commands
        .spawn((game_over_screen, visibility))
        .with_children(|game_over_screen| {
            game_over_screen.spawn(text);
        });
}

pub fn spawn_game_over_screen(mut commands: Commands) {
    let font_size = 48;
    spawn_game_over_ui(font_size, &mut commands);
}

pub fn update_game_over_screen(
    mut game_over_screen: Query<(&mut Visibility, &mut Text), (With<GameOverScreen>,)>,
    game_state: Res<State<GameState>>,
    game_over_timer: Query<&GameOverTimer, Changed<GameOverTimer>>,
) {
    if game_over_screen.is_empty() {
        return;
    }

    let (mut game_over_screen_visibility, mut game_over_screen_text) =
        game_over_screen.single_mut().unwrap();

    if game_state.get() == &GameState::Active {
        *game_over_screen_visibility = Visibility::Hidden;
        return;
    }

    *game_over_screen_visibility = Visibility::Visible;

    let game_over_timer = game_over_timer
        .single()
        .expect("update_game_over_screen: Could not find game over timer");

    game_over_screen_text.0 = format!(
        "Game Over\nContinue? {}",
        game_over_timer.get_timer().remaining().as_secs()
    );
}

#[derive(Component)]
pub struct BonusScoreUI;

#[derive(Resource, Clone)]
pub struct TemporaryUITime(Duration);

impl TemporaryUITime {
    pub fn new(time: Duration) -> Self {
        Self(time)
    }

    pub fn get_timer(&self) -> Timer {
        Timer::new(self.0, TimerMode::Once)
    }
}

#[derive(Component)]
pub struct TemporaryUITimer(Timer);

impl TemporaryUITimer {
    pub fn new(temporary_time: &TemporaryUITime) -> Self {
        Self(temporary_time.get_timer())
    }

    pub fn get_timer_mut(&mut self) -> &mut Timer {
        &mut self.0
    }

    pub fn get_timer(&self) -> &Timer {
        &self.0
    }
}

pub fn spawn_bonus_scores_ui(
    top_half_screen: Query<Entity, With<TopHalfScreen>>,
    mut commands: Commands,
) {
    let top_half_screen_entity = top_half_screen
        .single()
        .expect("spawn_bonus_ui: Could not find top half screen");
    let bonus_ui_node = Node {
        width: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };

    let bonus_ui_text = TextSection::new(36, BonusScoreUI);
    commands
        .entity(top_half_screen_entity)
        .with_children(|top_half_screen| {
            top_half_screen
                .spawn((bonus_ui_node, Visibility::Hidden))
                .with_children(|bonus_ui_node| {
                    bonus_ui_node.spawn(bonus_ui_text);
                });
        });
}

pub fn show_bonus_scores_on_exit(
    armed_traps: Query<&TrapState>,
    explorer_location: Query<
        &LogicalCoordinates,
        (With<ExplorerState>, Changed<LogicalCoordinates>),
    >,
    exit_door: Query<&LogicalCoordinates, With<ExitDoorState>>,
    mut bonus_score_ui: Query<
        (Entity, &mut Text, &mut Visibility),
        (With<BonusScoreUI>, Without<TemporaryUITimer>),
    >,
    temp_ui_visibility_time: Res<TemporaryUITime>,
    mut commands: Commands,
) {
    if armed_traps.is_empty()
        || explorer_location.is_empty()
        || exit_door.is_empty()
        || bonus_score_ui.is_empty()
    {
        return;
    }

    let exit_door_position = exit_door.single().unwrap();
    let explorer_position = explorer_location.single().unwrap();

    if explorer_position != exit_door_position {
        return;
    }

    let (bonus_score_ui_entity, mut bonus_score_ui_text, mut bonus_score_ui_visibility) =
        bonus_score_ui.single_mut().unwrap();

    let num_armed_traps = armed_traps
        .iter()
        .filter(|&armed_trap_state| armed_trap_state == &TrapState::Armed)
        .count();

    // No bonus points are awarded if there are no armed traps found.
    if num_armed_traps == 0 {
        return;
    }

    let bonus_points = num_armed_traps * 200;
    bonus_score_ui_text.0 = format!(
        "Bonus!\n{} Armed Traps Avoided x 200 = {}",
        num_armed_traps, bonus_points
    );

    *bonus_score_ui_visibility = Visibility::Visible;

    let temporary_ui_timer = TemporaryUITimer::new(&temp_ui_visibility_time);
    commands
        .entity(bonus_score_ui_entity)
        .insert(temporary_ui_timer);
}

pub fn hide_bonus_scores_after_time(
    mut bonus_scores_ui: Query<
        (Entity, &mut Visibility, &mut TemporaryUITimer),
        With<BonusScoreUI>,
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    if bonus_scores_ui.is_empty() {
        return;
    }

    let (bonus_score_ui_entity, mut bonus_score_ui_visibility, mut bonus_score_timer) =
        bonus_scores_ui.single_mut().unwrap();

    let bonus_score_timer = bonus_score_timer.get_timer_mut();
    let time_passed = time.delta();
    bonus_score_timer.tick(time_passed);
    if !bonus_score_timer.just_finished() {
        return;
    }

    *bonus_score_ui_visibility = Visibility::Hidden;
    commands
        .entity(bonus_score_ui_entity)
        .remove::<TemporaryUITimer>();
}
