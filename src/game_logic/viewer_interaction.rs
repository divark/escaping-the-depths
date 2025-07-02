use bevy::prelude::*;

use crate::LogicalCoordinates;

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

pub fn convert_viewer_click_to_tile_click(
    mut viewer_clicks: EventReader<ViewerClick>,
    mut movement_broadcaster: EventWriter<LogicalCoordinates>,
    window_info: Query<&Window>,
) {
    if window_info.is_empty() {
        return;
    }

    let window = window_info.single().expect(
        "convert_viewer_click_to_tile_click: Could not get information on the game Window.",
    );
    let window_width = window.width();
    let window_height = window.height();

    for viewer_click in viewer_clicks.read() {
        let x_pos = (window_width * viewer_click.get_x() / 16.0).round() as usize;
        let y_pos = (window_height * viewer_click.get_y() / 16.0).round() as usize;

        let converted_tile_pos = LogicalCoordinates::new(x_pos as usize, y_pos as usize);
        movement_broadcaster.write(converted_tile_pos);
    }
}
