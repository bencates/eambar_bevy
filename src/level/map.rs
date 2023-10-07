use {
    super::{Position, Tile},
    bevy::{
        prelude::Resource,
        utils::{HashMap, HashSet},
    },
    hex2d::Coordinate,
    rand::Rng,
    std::ops::Index,
};

#[derive(Resource)]
pub struct Map {
    tiles: HashMap<Coordinate, Tile>,
    revealed: HashSet<Coordinate>,
}

impl Map {
    pub fn new(rng: &mut impl Rng) -> Self {
        Self {
            tiles: super::bisection_generator::build(24, rng),
            revealed: HashSet::new(),
        }
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

impl Index<&Position> for Map {
    type Output = Tile;

    fn index(&self, pos: &Position) -> &Tile {
        &self[pos.as_ref()]
    }
}

impl Index<&Coordinate> for Map {
    type Output = Tile;

    fn index(&self, coord: &Coordinate) -> &Tile {
        self.tiles.get(coord).unwrap_or(&Tile::Floor)
    }
}
