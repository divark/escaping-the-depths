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

#[derive(Message)]
pub struct LoadMap(Map);

impl LoadMap {
    pub fn new(change_map_event: ChangeMap) -> Self {
        let map_to_load = change_map_event.get_map().clone();
        Self(map_to_load)
    }

    pub fn get_map(&self) -> &Map {
        &self.0
    }
}

struct BevySpriteLoader<'a> {
    asset_server: &'a AssetServer,
    texture_atlas_layouts: &'a mut Assets<TextureAtlasLayout>,
}

impl<'a> BevySpriteLoader<'a> {
    pub fn new(
        asset_server: &'a AssetServer,
        texture_atlas_layouts: &'a mut Assets<TextureAtlasLayout>,
    ) -> Self {
        Self {
            asset_server,
            texture_atlas_layouts,
        }
    }
}

/// Returns the Tile's Sprite based on its location as a SpriteBundle.
fn get_tile_sprite_from_tiled(
    tile_logical_coordinates: LogicalCoordinates,
    tiled_map: &Map,
    sprite_loader: &mut BevySpriteLoader,
) -> SpriteBundle {
    SpriteBundle::default()
}

/// Converts a Tiled map into a series of Tile locations and their sprites.
pub fn load_tiled_map(
    mut load_tiled_map_reader: MessageReader<LoadMap>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let mut bevy_sprite_loader = BevySpriteLoader::new(&asset_server, &mut texture_atlas_layouts);
    for loaded_tile_map in load_tiled_map_reader
        .read()
        .map(|load_map_event| load_map_event.get_map())
    {
        let mut tile_bundles = Vec::new();

        let map_width = loaded_tile_map.width as usize;
        let map_height = loaded_tile_map.height as usize;

        for y in 0..map_height {
            for x in 0..map_width {
                let tile_logical_coordinates = LogicalCoordinates::new(x, y);
                let tile_sprite = get_tile_sprite_from_tiled(
                    tile_logical_coordinates,
                    loaded_tile_map,
                    &mut bevy_sprite_loader,
                );
                tile_bundles.push(TileBundle::new(tile_sprite, tile_logical_coordinates));
            }
        }

        for rendered_tile in tile_bundles {
            commands.spawn(rendered_tile);
        }

        commands.spawn(WorldTileDimensions::new(map_width, map_height));
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

impl TileBundle {
    pub fn new(sprite_bundle: SpriteBundle, logical_coordinates: LogicalCoordinates) -> Self {
        Self {
            sprite_bundle,
            logical_coordinates,
        }
    }
}
