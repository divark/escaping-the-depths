use bevy::prelude::*;

use crate::core_logic::traveling::Pathfinding;

use super::setting::{LogicalCoordinates, TileSize};

#[derive(Component)]
pub struct CamperInformation {
    camper_name: String,
}

impl CamperInformation {
    pub fn new(camper_name: String) -> Self {
        Self { camper_name }
    }

    pub fn get_camper_name(&self) -> &String {
        &self.camper_name
    }
}

#[derive(Bundle)]
pub struct CamperBundle {
    camper_info: CamperInformation,
    camper_physical_location: Transform,
    camper_logical_location: LogicalCoordinates,
    travel_path: Pathfinding,
}

impl CamperBundle {
    pub fn new(
        camper_name: &String,
        camper_physical_location: Transform,
        camper_logical_location: LogicalCoordinates,
        travel_path: Pathfinding,
    ) -> Self {
        let camper_info = CamperInformation::new(camper_name.clone());

        Self {
            camper_info,
            camper_physical_location,
            camper_logical_location,
            travel_path,
        }
    }
}

#[derive(Message)]
pub struct ObjectiveAttempt {
    player_name: String,
    objective_attempted: String,
}

impl ObjectiveAttempt {
    pub fn new(player_name: String, objective_attempted: String) -> Self {
        Self {
            player_name,
            objective_attempted,
        }
    }

    pub fn get_camper_name(&self) -> &String {
        &self.player_name
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ScenarioResult {
    Fail,
    Success,
}

#[derive(Message)]
pub struct ScenarioAttempt {
    player_name: String,
    objective_completed: String,
    attempt_status: ScenarioResult,
}

impl ScenarioAttempt {
    pub fn new(
        player_name: String,
        objective_completed: String,
        attempt_status: ScenarioResult,
    ) -> Self {
        Self {
            player_name,
            objective_completed,
            attempt_status,
        }
    }

    pub fn get_camper_name(&self) -> &String {
        &self.player_name
    }

    pub fn get_objective(&self) -> &String {
        &self.objective_completed
    }

    pub fn get_status(&self) -> ScenarioResult {
        self.attempt_status
    }
}

#[derive(Message)]
pub struct ViewerClick {
    uv_x: f32,
    uv_y: f32,
}

impl ViewerClick {
    pub fn new(uv_x: f32, uv_y: f32) -> Self {
        Self { uv_x, uv_y }
    }

    pub fn get_x(&self) -> f32 {
        self.uv_x
    }

    pub fn get_y(&self) -> f32 {
        self.uv_y
    }
}

fn convert_to_coords(
    uv_coordinates: &ViewerClick,
    window_width: f32,
    window_height: f32,
) -> Transform {
    let coordinate_offset_x = window_width / 2.0;
    let coordinate_offset_y = window_height / 2.0;

    let screen_x = (uv_coordinates.get_x() * window_width) - coordinate_offset_x;
    let screen_y = (uv_coordinates.get_y() * window_height) - coordinate_offset_y;

    Transform::from_xyz(screen_x, screen_y, 2.0)
}

fn adjust_from_camera(screen_coordinates: Transform, camera: &Transform) -> Transform {
    *camera * screen_coordinates
}

fn convert_to_tilepos(game_coordinates: Transform, tile_size: f32) -> LogicalCoordinates {
    let x_pos = (game_coordinates.translation.x / tile_size).round();
    let y_pos = (game_coordinates.translation.y / tile_size).round();
    LogicalCoordinates::new(x_pos as usize, y_pos as usize, 0)
}

pub fn convert_viewer_click_to_tile_click(
    mut viewer_clicks: MessageReader<ViewerClick>,
    mut movement_broadcaster: MessageWriter<LogicalCoordinates>,
    window_info: Query<&Window>,
    tile_scaling: Res<TileSize>,
    camera: Query<&Transform, With<Camera2d>>,
) {
    if window_info.is_empty() || camera.is_empty() {
        return;
    }

    let window = window_info.single().expect(
        "convert_viewer_click_to_tile_click: Could not get information on the game Window.",
    );
    let window_width = window.physical_width() as f32;
    let window_height = window.physical_height() as f32;
    let tile_size = tile_scaling.calculate_size() as f32;

    let camera_position = camera
        .single()
        .expect("convert_viewer_click_to_tile_click: Could not get the camera.");

    for viewer_click in viewer_clicks.read() {
        let game_coords = convert_to_coords(viewer_click, window_width, window_height);
        let adjusted_game_coords = adjust_from_camera(game_coords, camera_position);
        let converted_tile_pos = convert_to_tilepos(adjusted_game_coords, tile_size);

        movement_broadcaster.write(converted_tile_pos);
    }
}
