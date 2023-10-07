use {
    super::Position,
    bevy::{
        prelude::{Component, Resource},
        utils::{HashMap, HashSet},
    },
    hex2d::Coordinate,
    rand::Rng,
    std::ops::Index,
};

#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    pub fn is_blocked(&self) -> bool {
        match self {
            Tile::Wall => true,
            Tile::Floor => false,
        }
    }

    pub fn is_opaque(&self) -> bool {
        match self {
            Tile::Wall => true,
            Tile::Floor => false,
        }
    }
}

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

    pub fn is_revealed(&self, coord: &Coordinate) -> bool {
        self.revealed.get(coord).is_some()
    }

    pub fn visible_tiles(&self) -> impl Iterator<Item = (Position, &Tile)> {
        self.tiles
            .iter()
            // .filter(|(coord, _)| self.is_revealed(coord))
            .map(|(coord, tile)| (Position::new(coord.x, coord.y, 0), tile))
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
