use bevy::prelude::*;

pub mod game_logic;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum RoomObject {
    #[default]
    Empty,
    Explorer,
    ExitDoor,
    HiddenFloorSwitch,
    Treasure(usize),
    Trap,
}

#[derive(Event, Component, Clone, Copy, PartialEq, Default)]
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
}

#[derive(Clone, Default)]
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

#[derive(Component, Debug, Default)]
pub struct CurrentRecords {
    current_score: usize,
}

impl CurrentRecords {
    pub fn get_current_score(&self) -> usize {
        self.current_score
    }

    pub fn add_score(&mut self, score_to_add: usize) {
        self.current_score += score_to_add;
    }
}

pub struct CaveRoom {
    width: usize,
    height: usize,

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

        Self {
            width,
            height,

            room_tiles,
            room_objects: Vec::new(),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile_type: RoomObject) {
        let logical_coordinates = LogicalCoordinates::new(x, y);
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
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
