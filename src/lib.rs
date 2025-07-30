use bevy::prelude::*;

pub mod game_logic;

pub trait RoomGenerating {
    fn generate(&self) -> CaveRoom;
}

#[derive(States, Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Active,
    GameOver,
}

#[derive(Clone, Copy, PartialEq, Default, Hash, Eq, Debug)]
pub enum RoomObject {
    #[default]
    Empty,
    Wall,
    Explorer,
    ExitDoor,
    HiddenFloorSwitch,
    Treasure(usize),
    Trap,
}

#[derive(Event, Component, Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
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
    tile_type: RoomObject,
}

impl Tile {
    pub fn new(logical_coordinates: LogicalCoordinates) -> Self {
        Self {
            logical_coordinates,
            tile_type: RoomObject::default(),
        }
    }

    pub fn get_type(&self) -> &RoomObject {
        &self.tile_type
    }

    pub fn get_logical_coordinates(&self) -> &LogicalCoordinates {
        &self.logical_coordinates
    }

    pub fn set_type(&mut self, new_type: RoomObject) {
        self.tile_type = new_type;
    }
}

#[derive(Clone, Copy, Debug, Component, PartialEq)]
pub enum ExitDoorState {
    Closed,
    Open,
}

#[derive(Clone, Copy, Debug, Component, PartialEq)]
pub enum TrapState {
    Armed,
    Unarmed,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Wall;

#[derive(Component, Debug)]
pub struct CurrentRecords {
    current_score: usize,
    current_room_number: usize,

    record_score: usize,
    record_room_number: usize,
}

impl CurrentRecords {
    pub fn get_current_score(&self) -> usize {
        self.current_score
    }

    pub fn get_current_room_number(&self) -> usize {
        self.current_room_number
    }

    pub fn set_current_room_number(&mut self, new_room_number: usize) {
        self.current_room_number = new_room_number;
    }

    pub fn add_score(&mut self, score_to_add: usize) {
        self.current_score += score_to_add;
    }

    pub fn set_current_score(&mut self, new_score: usize) {
        self.current_score = new_score;
    }

    pub fn get_record_score(&self) -> usize {
        self.record_score
    }

    pub fn increment_room_count(&mut self) {
        self.current_room_number += 1;
    }

    pub fn get_record_room_number(&self) -> usize {
        self.record_room_number
    }

    /// Records the current records and sets all current stats back to their
    /// defaults.
    pub fn reset(&mut self) {
        self.record_score = self.record_score.max(self.current_score);
        self.record_room_number = self.record_room_number.max(self.current_room_number);

        self.current_score = 0;
        self.current_room_number = 1;
    }
}

impl Default for CurrentRecords {
    fn default() -> Self {
        Self {
            current_score: 0,
            current_room_number: 1,

            record_score: 0,
            record_room_number: 1,
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Resource, Debug, PartialEq)]
pub struct ExplorerHealth {
    current: usize,
    total: usize,
}

impl ExplorerHealth {
    pub fn new(current: usize, total: usize) -> Self {
        Self { current, total }
    }

    pub fn get_current_health(&self) -> usize {
        self.current
    }

    pub fn set_current_health(&mut self, current_health: usize) {
        self.current = current_health;
    }

    pub fn set_total_health(&mut self, total_health: usize) {
        self.total = total_health;
    }

    pub fn decrease_current_health(&mut self) {
        if self.current != 0 {
            self.current -= 1;
        }
    }

    pub fn get_total_health(&self) -> usize {
        self.total
    }
}

#[derive(Clone, Debug)]
pub struct CaveRoom {
    world_tile_dimensions: WorldTileDimensions,

    room_tiles: Vec<Tile>,
    room_objects: Vec<Tile>,
}

impl CaveRoom {
    pub fn new(width: usize, height: usize) -> Self {
        let mut room_tiles = Vec::new();
        for i in 0..width {
            for j in 0..height {
                room_tiles.push(Tile::new(LogicalCoordinates::new(i, j)));
            }
        }

        let world_tile_dimensions = WorldTileDimensions::new(width, height);
        Self {
            world_tile_dimensions,
            room_tiles,
            room_objects: Vec::new(),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile_type: RoomObject) {
        let logical_coordinates = LogicalCoordinates::new(x, y);
        if tile_type == RoomObject::Wall || tile_type == RoomObject::ExitDoor {
            let found_tile = self
                .room_tiles
                .iter_mut()
                .find(|tile| tile.get_logical_coordinates() == &logical_coordinates)
                .expect("set: Could not find designated tile for wall.");
            found_tile.set_type(tile_type);

            return;
        }

        let mut room_object = Tile::new(logical_coordinates);
        room_object.set_type(tile_type);

        self.room_objects.push(room_object);
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.room_tiles
    }

    pub fn get_objects(&self) -> &Vec<Tile> {
        &self.room_objects
    }

    pub fn get_width(&self) -> usize {
        self.world_tile_dimensions.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.world_tile_dimensions.get_height()
    }

    pub fn get_dimensions(&self) -> &WorldTileDimensions {
        &self.world_tile_dimensions
    }
}
