use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

use bevy::prelude::*;
use serde_json::Value;

use crate::core_logic::{
    CampersState,
    interacting::{ScenarioAttempt, ScenarioResult},
    setting::LoadMap,
};

/// Represents the hunger of all campers in the game.
///
/// --------------------------
/// |xxxxxxxxxxxx<~><~><~><~>|
/// --------------------------
/// x = Current percentage left out of total
/// <~> = The chunks of percentage removed so far (percent decrease)
#[derive(Resource)]
pub struct HungerBar {
    current_percentage: usize,

    percent_decrease: usize,
}

impl Default for HungerBar {
    fn default() -> Self {
        let current_percentage = 0;
        let percent_decrease = 1;
        Self {
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

/// Represents a list of successful actions done by campers.
#[derive(Component, Debug, Default)]
pub struct ContributionsList {
    contributions: Vec<String>,
}

/// Converts an objective name into an achieved message.
/// Example: 'Find food.' -> 'found food!'
fn get_achieved_objective_msg(objective_name: &str) -> String {
    let mut objective_message = objective_name.to_lowercase();
    // 1. Remove the '.' at the end.
    objective_message
        .pop()
        .expect("get_achieved_objective_msg: An empty string was given.");

    // 2. Past tense the verb in the sentence.
    // Example: 'find' -> 'found'
    let mut objective_split_on_whitespace = objective_message.split_whitespace();
    let objective_action = objective_split_on_whitespace
        .next()
        .expect("get_achieved_objective_msg: Could not find action.");
    let past_tense_action = match objective_action {
        "find" => "found",
        "seek" => "found",
        _ => panic!(
            "get_achieved_objective_msg: Unrecognized action given: {}",
            objective_action
        ),
    };

    let objective_completed = objective_split_on_whitespace
        .next()
        .expect("get_achieved_objective_msg: Could not get what was achieved.");

    format!("{} {}!", past_tense_action, objective_completed)
}

impl ContributionsList {
    pub fn contains(&self, contribution: &String) -> bool {
        self.contributions.contains(contribution)
    }

    /// Converts an objective attempt into a recorded contribution.
    pub fn record(&mut self, objective_attempt: &ScenarioAttempt) {
        let camper_name = objective_attempt.get_camper_name();
        let camper_objective_achieved =
            get_achieved_objective_msg(objective_attempt.get_objective());
        let contribution = format!("{} {}", camper_name, camper_objective_achieved);
        self.contributions.push(contribution);
    }
}

/// Records all successful attempts of an objective from the campers.
pub fn record_camper_contribution(
    mut objective_attempts: MessageReader<ScenarioAttempt>,
    mut contributions_list: Single<&mut ContributionsList>,
) {
    for objective_attempt in objective_attempts.read() {
        if objective_attempt.get_status() == ScenarioResult::Fail {
            continue;
        }

        contributions_list.record(objective_attempt);
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

/// Spawns into the game the list of objectives from the provided objectives json file
/// already loaded.
fn spawn_map_objectives(objectives_json: &Value, commands: &mut Commands) {
    let objective_names: Vec<String> = objectives_json["objectives"]
        .as_array()
        .expect("spawn_map_objectives: Could not find objectives array in json.")
        .iter()
        .map(|objective_json_value| objective_json_value.as_str().unwrap().to_string())
        .collect();

    for objective_name in objective_names {
        let parsed_objective = CamperObjective::new(objective_name);
        commands.spawn(parsed_objective);
    }

    commands.spawn(ContributionsList::default());
}

/// Represents some setting with a series of scenarios that can take place there.
#[derive(Component)]
pub struct Landmark {
    name: String,
    description: String,
    scenarios: Vec<LandmarkScenario>,
}

impl Landmark {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            scenarios: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn add_scenario(&mut self, scenario: LandmarkScenario) {
        self.scenarios.push(scenario);
    }

    pub fn get_scenario(&self, scenario_num: usize) -> &LandmarkScenario {
        &self.scenarios[scenario_num]
    }
}

/// Represents some situation with a series of choices to do.
pub struct LandmarkScenario {
    objective_type: String,
    description: String,

    choices: Vec<ScenarioChoice>,
}

impl LandmarkScenario {
    pub fn new(objective_type: String, description: String) -> Self {
        Self {
            objective_type,
            description,
            choices: Vec::new(),
        }
    }

    pub fn get_type(&self) -> String {
        self.objective_type.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn add_choice(&mut self, choice: ScenarioChoice) {
        self.choices.push(choice);
    }

    pub fn get_choice(&self, choice_num: usize) -> &ScenarioChoice {
        &self.choices[choice_num]
    }
}

/// Represents an action that a camper/viewer can do that yields some result.
pub struct ScenarioChoice {
    description: String,

    success_description: String,
    failure_description: String,
}

impl ScenarioChoice {
    pub fn new(description: String) -> Self {
        Self {
            description,
            success_description: String::new(),
            failure_description: String::new(),
        }
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_success(&mut self, success_description: String) {
        self.success_description = success_description;
    }

    pub fn get_success_result(&self) -> String {
        self.success_description.clone()
    }

    pub fn set_failure(&mut self, failure_description: String) {
        self.failure_description = failure_description;
    }

    pub fn get_failure_result(&self) -> String {
        self.failure_description.clone()
    }
}

/// Returns a Scenario Choice parsed from a JSON object.
fn parse_choice(scenario_choice_entry: &Value) -> ScenarioChoice {
    let choice_description = scenario_choice_entry["choice_description"]
        .as_str()
        .expect(
            "parse_choice: Could not find choice_description field for the scenario in the json.",
        )
        .to_string();
    let mut parsed_objective_choice = ScenarioChoice::new(choice_description);

    let success_description = scenario_choice_entry["results"]["success"]
        .as_str()
        .expect("parse_choice: Could not find success field in json.")
        .to_string();
    parsed_objective_choice.set_success(success_description);
    let failure_description = scenario_choice_entry["results"]["failure"]
        .as_str()
        .expect("parse_choice: Could not find failure field in json.")
        .to_string();
    parsed_objective_choice.set_failure(failure_description);

    parsed_objective_choice
}

/// Returns a Landmark Scenario parsed from a JSON object.
fn parse_scenario(scenario_entry: &Value) -> LandmarkScenario {
    let objective_type = scenario_entry["objective"]
        .as_str()
        .expect("parse_scenario: Could not find objective field for scenario in json.")
        .to_string();
    let scenario_description = scenario_entry["scenario_description"]
        .as_str()
        .expect("parse_scenario: Could not find description field for scenario in json.")
        .to_string();
    let mut parsed_landmark_scenario = LandmarkScenario::new(objective_type, scenario_description);

    let choice_entries = scenario_entry["choices"]
        .as_array()
        .expect("parse_scenario: Could not find choices in objectives json.");
    for choice_entry in choice_entries {
        let parsed_choice = parse_choice(choice_entry);
        parsed_landmark_scenario.add_choice(parsed_choice);
    }

    parsed_landmark_scenario
}

/// Returns a Landmark parsed from a JSON object.
fn parse_landmark(landmark_json_entry: &Value) -> Landmark {
    let landmark_name = landmark_json_entry["name"]
        .as_str()
        .expect("parse_landmark: Could not find name for landmark in json.")
        .to_string();
    let landmark_description = landmark_json_entry["landmark_description"]
        .as_str()
        .expect("parse_landmark: Could not find description for landmark in json.")
        .to_string();
    let mut parsed_landmark = Landmark::new(landmark_name, landmark_description);

    let scenario_entries = landmark_json_entry["scenarios"]
        .as_array()
        .expect("parse_landmark: Could not find scenarios in objectives json.");
    for scenario_entry in scenario_entries {
        let parsed_scenario = parse_scenario(scenario_entry);
        parsed_landmark.add_scenario(parsed_scenario);
    }

    parsed_landmark
}

/// Spawns into the game the list of landmarks from the provided objectives json file
/// already loaded.
fn spawn_landmarks(objectives_json: &Value, commands: &mut Commands) {
    let landmark_entries = objectives_json["landmarks"]
        .as_array()
        .expect("spawn_landmarks: Could not find list of landmarks in objectives json.");
    for landmark_entry in landmark_entries {
        let parsed_landmark = parse_landmark(landmark_entry);
        commands.spawn(parsed_landmark);
    }
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

    spawn_map_objectives(&objectives_json, &mut commands);
    spawn_landmarks(&objectives_json, &mut commands);
}
