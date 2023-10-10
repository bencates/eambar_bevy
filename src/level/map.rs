use {
    super::{Position, Tile},
    bevy::{
        prelude::Resource,
        utils::{HashMap, HashSet},
    },
    std::ops::Index,
};

#[derive(Resource)]
pub struct Map {
    tiles: HashMap<Position, Tile>,
    revealed: HashSet<Position>,
}

impl Map {
    pub(super) fn new(tiles: HashMap<Position, Tile>) -> Self {
        Self {
            tiles,
            revealed: HashSet::new(),
        }
    }

    #[allow(dead_code)]
    pub(super) fn reveal_all(&mut self) {
        self.revealed.extend(self.tiles.keys());
    }

    pub(super) fn reveal(&mut self, coords: impl Iterator<Item = Position>) {
        self.revealed.extend(coords);
    }

    pub fn revealed(&self) -> &HashSet<Position> {
        &self.revealed
    }

    pub fn tiles(&self) -> &HashMap<Position, Tile> {
        &self.tiles
    }
}

impl Index<&Position> for Map {
    type Output = Tile;

    fn index(&self, pos: &Position) -> &Tile {
        self.tiles.get(pos).unwrap_or(&Tile::Floor)
    }
}
