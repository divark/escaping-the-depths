use std::path::PathBuf;

use bevy::prelude::*;
use tiled::{Loader, Map};

pub const WALLS_OFFSET: usize = 2;

#[derive(Resource, Clone)]
pub struct TileSize {
    size: usize,
    scale: usize,
}

impl Default for TileSize {
    fn default() -> Self {
        Self { size: 16, scale: 1 }
    }
}

impl TileSize {
    pub fn new(size: usize, scale: usize) -> Self {
        Self { size, scale }
    }

    pub fn set_scale(&mut self, desired_scale: usize) {
        self.scale = desired_scale;
    }

    pub fn get_scale(&self) -> usize {
        self.scale
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn calculate_size(&self) -> usize {
        self.size * self.scale
    }
}

#[derive(PartialEq, Component, Clone, Copy, Debug, Default)]
pub struct WorldTileDimensions {
    width: usize,
    height: usize,
}

impl WorldTileDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

#[derive(Message, Component, Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct LogicalCoordinates {
    x: usize,
    y: usize,
}

impl LogicalCoordinates {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn to_1d(&self, world_tile_dimensions: &WorldTileDimensions) -> usize {
        (world_tile_dimensions.get_height() * self.get_y()) + self.get_x()
    }
}

#[derive(Clone, Default, Debug)]
pub struct Tile {
    logical_coordinates: LogicalCoordinates,
}

impl Tile {
    pub fn new(logical_coordinates: LogicalCoordinates) -> Self {
        Self {
            logical_coordinates,
        }
    }

    pub fn get_logical_coordinates(&self) -> &LogicalCoordinates {
        &self.logical_coordinates
    }
}

#[derive(Message)]
pub struct ChangeMap(Map);

impl ChangeMap {
    pub fn new(map_to_load: PathBuf) -> Self {
        let mut loader = Loader::new();
        let loaded_map = loader
            .load_tmx_map(map_to_load)
            .expect("Could not load the desired Tiled map.");
        Self(loaded_map)
    }

    pub fn get_map(&self) -> &Map {
        &self.0
    }
}

#[derive(Bundle, Default)]
pub struct SpriteBundle {
    sprite: Sprite,
    position: Transform,
}

impl SpriteBundle {
    pub fn set_sprite(&mut self, new_sprite: Sprite) {
        self.sprite = new_sprite;
    }

    pub fn set_position(&mut self, new_position: Transform) {
        self.position = new_position;
    }
}

#[derive(Bundle, Default)]
pub struct TileBundle {
    sprite_bundle: SpriteBundle,
    logical_coordinates: LogicalCoordinates,
}
