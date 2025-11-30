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
    depth: usize,
}

impl WorldTileDimensions {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }
}

#[derive(Message, Component, Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct LogicalCoordinates {
    x: usize,
    y: usize,
    z: usize,
}

impl LogicalCoordinates {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_z(&self) -> usize {
        self.z
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
    pub fn new(change_map_event: &ChangeMap) -> Self {
        let map_to_load = change_map_event.get_map().clone();
        Self(map_to_load)
    }

    /// Returns the filename without the extension for the loaded
    /// map.
    pub fn get_name(&self) -> String {
        self.0
            .source
            .file_stem()
            .expect("get_name: File does not have a name.")
            .to_str()
            .unwrap()
            .to_string()
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

    /// Returns a loaded Sprite as an Image Handle for the given Tile in the Tiled Map.
    fn load_sprite_from_tiled(
        &mut self,
        tiled_map: &Map,
        tile_logical_coordinates: &LogicalCoordinates,
    ) -> Option<Handle<Image>> {
        let x = tile_logical_coordinates.get_x();
        let y = tile_logical_coordinates.get_y();
        let z = tile_logical_coordinates.get_z();
        let tile = tiled_map
            .get_layer(z)?
            .as_tile_layer()?
            .get_tile(x as i32, y as i32)?;

        let tile_tileset_path = tile.get_tileset().source.clone();
        Some(self.asset_server.load(tile_tileset_path))
    }

    /// Returns a loaded Texture Atlas as a Texture Atlas Layout Handle for the given Tile in the Tiled Map.
    fn load_tilesheet_atlas(
        &mut self,
        tiled_map: &Map,
        tile_logical_coordinates: &LogicalCoordinates,
    ) -> Option<TextureAtlas> {
        let x = tile_logical_coordinates.get_x();
        let y = tile_logical_coordinates.get_y();
        let z = tile_logical_coordinates.get_z();
        let tile = tiled_map
            .get_layer(z)?
            .as_tile_layer()?
            .get_tile(x as i32, y as i32)?;
        let tile_sprite_idx = tile.tileset_index();
        let tile_tilesheet = tile.get_tileset();
        let tile_tilesheet_image = tile_tilesheet.image.as_ref()?;
        let tile_width = tile_tilesheet.tile_width;
        let tile_height = tile_tilesheet.tile_height;

        let num_rows = tile_tilesheet_image.height as u32 / tile_height;
        let num_columns = tile_tilesheet_image.width as u32 / tile_width;
        let tilesheet_atlas_layout = TextureAtlasLayout::from_grid(
            UVec2::new(tile_width, tile_height),
            num_columns,
            num_rows,
            None,
            None,
        );
        let loaded_tile_tilesheet_atlas = self.texture_atlas_layouts.add(tilesheet_atlas_layout);

        Some(TextureAtlas {
            layout: loaded_tile_tilesheet_atlas,
            index: tile_sprite_idx,
        })
    }

    /// Returns a Sprite for the given Tile in the Tiled map if it exists, or None otherwise.
    pub fn load_from_tiled(
        &mut self,
        tiled_map: &Map,
        tile_logical_coordinates: &LogicalCoordinates,
    ) -> Option<Sprite> {
        let tile_tileset_image =
            self.load_sprite_from_tiled(tiled_map, tile_logical_coordinates)?;
        let tile_tilesheet_atlas =
            self.load_tilesheet_atlas(tiled_map, tile_logical_coordinates)?;

        Some(Sprite::from_atlas_image(
            tile_tileset_image,
            tile_tilesheet_atlas,
        ))
    }
}

/// Returns a physical y coordinate in pixels whose y-axis has been reversed.
///
/// This is needed because Tiled has its y coordinates going down, while Bevy
/// has it going up.
fn flip_physical_y_coordinate(
    tile_logical_coordinates: &LogicalCoordinates,
    tiled_map: &Map,
) -> f32 {
    let tile_height = tiled_map
        .get_layer(tile_logical_coordinates.get_z())
        .expect("flip_physical_y_coordinate: Layer does not exist.")
        .as_tile_layer()
        .expect("flip_physical_y_coordinate: This is not a Tile layer.")
        .get_tile(
            tile_logical_coordinates.get_x() as i32,
            tile_logical_coordinates.get_y() as i32,
        )
        .expect("flip_physical_y_coordinate: Tile does not exist on the Tile layer.")
        .get_tileset()
        .tile_height;
    let tile_map_height = tiled_map.height * tiled_map.tile_height;

    let tile_logical_y = tile_logical_coordinates.get_y();
    let tile_y = tile_logical_y * tile_height as usize;
    tile_map_height as f32 - tile_y as f32
}

/// Returns the Tile's Sprite based on its location as a SpriteBundle.
fn get_tile_sprite_from_tiled(
    tile_logical_coordinates: &LogicalCoordinates,
    tiled_map: &Map,
    sprite_loader: &mut BevySpriteLoader,
) -> Option<SpriteBundle> {
    let tile_sprite = sprite_loader.load_from_tiled(tiled_map, tile_logical_coordinates)?;

    let tile_x = tile_logical_coordinates.get_x();
    let tile_z = tile_logical_coordinates.get_z();
    let tile_width = tiled_map.tile_width as f32;

    // Tiled has the y coordinates going from top-to-bottom, while Bevy has it going from bottom-to-top.
    //
    // Because of this, the y coordinate has to be "flipped."
    let flipped_y = flip_physical_y_coordinate(tile_logical_coordinates, tiled_map);
    let tile_physical_position =
        Transform::from_xyz(tile_x as f32 * tile_width, flipped_y, tile_z as f32);
    Some(SpriteBundle {
        sprite: tile_sprite,
        position: tile_physical_position,
    })
}

/// Unloads the current map rendered before loading the new one.
pub fn unload_current_map(
    mut change_map_reader: MessageReader<ChangeMap>,
    mut load_map_broadcaster: MessageWriter<LoadMap>,
    rendered_tiles: Query<Entity, With<LogicalCoordinates>>,
    map_size: Query<Entity, With<WorldTileDimensions>>,
    mut commands: Commands,
) {
    for change_map_event in change_map_reader.read() {
        for rendered_tile_entity in &rendered_tiles {
            commands.entity(rendered_tile_entity).despawn();
        }

        if let Ok(map_size_entity) = map_size.single() {
            commands.entity(map_size_entity).despawn();
        }

        load_map_broadcaster.write(LoadMap::new(change_map_event));
    }
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
        // let mut locations_of_interest: Vec<MapLocationBundle> = Vec::new();

        let map_depth = loaded_tile_map.layers().len();
        let map_width = loaded_tile_map.width as usize;
        let map_height = loaded_tile_map.height as usize;

        let tiled_map_dimensions = WorldTileDimensions::new(map_width, map_height, map_depth);

        for z in 0..map_depth {
            for y in 0..map_height {
                for x in 0..map_width {
                    let tile_logical_coordinates = LogicalCoordinates::new(x, y, z);
                    if let Some(tile_sprite) = get_tile_sprite_from_tiled(
                        &tile_logical_coordinates,
                        loaded_tile_map,
                        &mut bevy_sprite_loader,
                    ) {
                        tile_bundles.push(TileBundle::new(tile_sprite, tile_logical_coordinates));
                    }
                }
            }
        }

        for rendered_tile in tile_bundles {
            commands.spawn(rendered_tile);
        }

        commands.spawn(tiled_map_dimensions);
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
