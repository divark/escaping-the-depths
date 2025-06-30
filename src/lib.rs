pub mod game_logic;

#[derive(Clone, Copy, Default)]
pub enum RoomObject {
    #[default]
    Empty,
    Explorer,
    ExitDoor,
    HiddenFloorSwitch,
}

#[derive(Clone, Default)]
pub struct Tile {
    tile_type: RoomObject,
}

impl Tile {
    pub fn set_type(&mut self, new_type: RoomObject) {
        self.tile_type = new_type;
    }
}

#[derive(Debug, PartialEq)]
pub enum ExitDoorState {
    Closed,
    Open,
}

pub struct CaveRoom {
    tiles: Vec<Vec<Tile>>,
}

impl CaveRoom {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![vec![Tile::default(); height]; width],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile_type: RoomObject) {
        self.tiles[x][y].set_type(tile_type);
    }
}
