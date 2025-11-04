use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

use bevy::prelude::*;
use serde_json::Value;

use crate::core_logic::{CampersState, setting::LoadMap};

/// Represents the hunger of all campers in the game.
///
/// --------------------------
/// |xxxxxxxxxxxx<~><~><~><~>|
/// --------------------------
/// x = Current percentage left out of total
/// <~> = The chunks of percentage removed so far (percent decrease)
#[derive(Resource)]
pub struct HungerBar {
    total_percentage: usize,
    current_percentage: usize,

    percent_decrease: usize,
}

impl Default for HungerBar {
    fn default() -> Self {
        let total_percentage = 100;
        let current_percentage = 0;
        let percent_decrease = 1;
        Self {
            total_percentage,
            current_percentage,
            percent_decrease,
        }
    }
}

impl HungerBar {
    /// Sets the current percentage of the Hunger Bar.
    pub fn set_percentage(&mut self, current_percentage: usize) {
        self.current_percentage = current_percentage;
    }

    /// Sets the amount of percentage to remove every time the hunger bar decreases.
    pub fn set_percentage_decrease(&mut self, percent_decrease: usize) {
        self.percent_decrease = percent_decrease;
    }

    /// Returns the current percentage left in the hunger bar.
    pub fn get_current_percentage(&self) -> usize {
        self.current_percentage
    }

    /// Decreases the hunger bar by one chunk determined by the percentage decrease.
    pub fn decrease(&mut self) {
        if self.percent_decrease > self.current_percentage {
            self.current_percentage = 0;
            return;
        }

        self.current_percentage -= self.percent_decrease;
    }
}

/// The amount of time to count when decreasing the hunger bar.
#[derive(Resource, Clone)]
pub struct HungerBarTime(Duration);

impl HungerBarTime {
    pub fn new(time_to_count: Duration) -> Self {
        Self(time_to_count)
    }

    pub fn get_duration(&self) -> Duration {
        self.0
    }
}

/// A timer that informs the HungerBar when to decrease
/// indefinitely.
#[derive(Component)]
pub struct HungerBarTimer(Timer);

impl HungerBarTimer {
    pub fn new(hunger_bar_time: &HungerBarTime) -> Self {
        let duration_to_count = hunger_bar_time.get_duration();
        let countdown_timer = Timer::new(duration_to_count, TimerMode::Repeating);
        Self(countdown_timer)
    }

    /// Advances the timer by the amount of time passed.
    pub fn tick(&mut self, time_passed: Duration) {
        self.0.tick(time_passed);
    }

    /// Returns whether the timer has finished counting down to its
    /// desired duration.
    pub fn finished(&self) -> bool {
        self.0.is_finished()
    }
}

/// Spawns the hunger bar when the game first starts.
pub fn spawn_hunger_bar(hunger_bar_time: Res<HungerBarTime>, mut commands: Commands) {
    let hunger_bar_timer = HungerBarTimer::new(&hunger_bar_time);
    commands.spawn(hunger_bar_timer);
}

/// Gradually decreases the hunger bar over time.
pub fn decrease_hunger_bar_over_time(
    mut hunger_bar: ResMut<HungerBar>,
    mut hunger_bar_timer: Single<&mut HungerBarTimer>,
    time: Res<Time>,
) {
    hunger_bar_timer.tick(time.delta());
    if !hunger_bar_timer.finished() {
        return;
    }

    hunger_bar.decrease();
}

/// Determines whether the campers have starved and died or not.
pub fn determine_campers_state(
    hunger_bar: Res<HungerBar>,
    mut campers_state: ResMut<NextState<CampersState>>,
) {
    if !hunger_bar.is_changed() || hunger_bar.get_current_percentage() != 0 {
        return;
    }

    campers_state.set(CampersState::Dead);
}

/// Represents some Objective shown on the screen for campers/viewers
/// to complete, such as 'Seek Food' or 'Find firewood.'
#[derive(Component)]
pub struct CamperObjective {
    label: String,
}

impl CamperObjective {
    pub fn new(label: String) -> Self {
        Self { label }
    }

    pub fn get_name(&self) -> String {
        self.label.clone()
    }
}

/// A resource holding the location of where to load objective files.
#[derive(Resource)]
pub struct ObjectivesDirectory(PathBuf);

impl ObjectivesDirectory {
    pub fn new(objectives_file_path: PathBuf) -> Self {
        Self(objectives_file_path)
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.0
    }
}

/// Returns a BufferedReader from the loaded Objectives file, or panics if
/// there's a problem with the file.
fn load_objective_file(
    objectives_directory: &ObjectivesDirectory,
    loaded_map_name: String,
) -> BufReader<File> {
    let objective_file_name = loaded_map_name + "_objectives.json";
    let mut objective_file_path = PathBuf::from(objectives_directory.get_path());
    objective_file_path.push(objective_file_name);

    let loaded_objective_file =
        File::open(objective_file_path).expect("load_objective_file: Could not load file.");
    BufReader::new(loaded_objective_file)
}

/// Spawns a series of Objectives for the camper based on the currently loaded map.
pub fn load_map_objectives(
    mut loaded_map_reader: MessageReader<LoadMap>,
    objectives_directory: Res<ObjectivesDirectory>,
    mut commands: Commands,
) {
    if loaded_map_reader.is_empty() {
        return;
    }

    let loaded_map = loaded_map_reader.read().next().unwrap();
    let loaded_map_name = loaded_map.get_name();

    let objectives_file = load_objective_file(&objectives_directory, loaded_map_name);
    let objectives_json: Value = serde_json::from_reader(objectives_file)
        .expect("load_map_objectives: Could not read from objectives file");
    let loaded_objectives = objectives_json["objectives"]
        .as_array()
        .expect("load_map_objectives: Could not get array from objectives file.");

    for objective_json_object in loaded_objectives {
        let objective_name = objective_json_object["name"]
            .as_str()
            .expect("load_map_objectives: Could not get name field from objective item.")
            .to_string();
        let camper_objective = CamperObjective::new(objective_name);
        commands.spawn(camper_objective);
    }
}
