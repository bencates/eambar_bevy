use {
    super::Tile,
    bevy::{
        prelude::Resource,
        utils::{HashMap, HashSet},
    },
    hex2d::Coordinate,
    std::ops::Index,
};

#[derive(Resource)]
pub struct Map {
    tiles: HashMap<Coordinate, Tile>,
    revealed: HashSet<Coordinate>,
}

impl Map {
    pub(super) fn new(tiles: HashMap<Coordinate, Tile>) -> Self {
        Self {
            tiles,
            revealed: HashSet::new(),
        }
    }

    #[allow(dead_code)]
    pub(super) fn reveal_all(&mut self) {
        self.revealed.extend(self.tiles.keys());
    }

    pub(super) fn reveal(&mut self, coords: impl Iterator<Item = Coordinate>) {
        self.revealed.extend(coords);
    }

    pub fn revealed(&self) -> &HashSet<Coordinate> {
        &self.revealed
    }

    pub fn tiles(&self) -> &HashMap<Coordinate, Tile> {
        &self.tiles
    }
}

impl Index<&Coordinate> for Map {
    type Output = Tile;

    fn index(&self, coord: &Coordinate) -> &Tile {
        self.tiles.get(coord).unwrap_or(&Tile::Floor)
    }
}
