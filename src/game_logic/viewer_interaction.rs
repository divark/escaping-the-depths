use bevy::prelude::*;

use super::room_generating::HiddenFloorSwitch;
use crate::{ExitDoorState, LogicalCoordinates};

#[derive(Event)]
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
    let screen_x = (uv_coordinates.get_x() * window_width) - (window_width / 2.0);
    let screen_y = (uv_coordinates.get_y() * window_height) - (window_height / 2.0);

    Transform::from_xyz(screen_x, screen_y, 2.0)
}

fn adjust_from_camera(screen_coordinates: Transform, camera: &Transform) -> Transform {
    *camera * screen_coordinates
}

fn convert_to_tilepos(game_coordinates: Transform, tile_size: usize) -> LogicalCoordinates {
    let x_pos = game_coordinates.translation.x / tile_size as f32;
    let y_pos = game_coordinates.translation.y / tile_size as f32;
    LogicalCoordinates::new(x_pos as usize, y_pos as usize)
}

pub fn convert_viewer_click_to_tile_click(
    mut viewer_clicks: EventReader<ViewerClick>,
    mut movement_broadcaster: EventWriter<LogicalCoordinates>,
    window_info: Query<&Window>,
    camera: Query<&Transform, With<Camera2d>>,
) {
    if window_info.is_empty() || camera.is_empty() {
        return;
    }

    let window = window_info.single().expect(
        "convert_viewer_click_to_tile_click: Could not get information on the game Window.",
    );
    let window_width = window.width();
    let window_height = window.height();

    let camera_position = camera
        .single()
        .expect("convert_viewer_click_to_tile_click: Could not get the camera.");

    for viewer_click in viewer_clicks.read() {
        let game_coords = convert_to_coords(viewer_click, window_width, window_height);
        let adjusted_game_coords = adjust_from_camera(game_coords, camera_position);
        let converted_tile_pos = convert_to_tilepos(adjusted_game_coords, 16);

        movement_broadcaster.write(converted_tile_pos);
    }
}

pub fn unlock_exit_door_with_viewer_click(
    mut movement_changes: EventReader<LogicalCoordinates>,
    hidden_floor_switch: Query<&LogicalCoordinates, With<HiddenFloorSwitch>>,
    mut exit_door: Query<&mut ExitDoorState>,
) {
    if movement_changes.is_empty() || exit_door.is_empty() || hidden_floor_switch.is_empty() {
        return;
    }

    for movement_coordinates in movement_changes.read() {
        let hidden_floor_switch_coordinates = hidden_floor_switch.single().expect(
            "unlock_hidden_door_with_viewer_click: Could not find the coordinates of the hidden floor switch.",
        );
        let mut exit_door_state = exit_door
            .single_mut()
            .expect("unlock_exit_door_with_viewer_click: Could not find the exit door.");

        if *movement_coordinates == *hidden_floor_switch_coordinates {
            *exit_door_state = ExitDoorState::Open;
        }
    }
}
