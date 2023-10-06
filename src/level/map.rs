use {
    super::Position,
    bevy::{
        prelude::Resource,
        utils::{HashMap, HashSet},
    },
    hex2d::Coordinate,
    rand::Rng,
    std::ops::Index,
};

#[derive(Clone, Copy, Debug, PartialEq)]
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
    _revealed: HashSet<Coordinate>,
}

impl Map {
    pub fn new(rng: &mut impl Rng) -> Self {
        Self {
            tiles: super::bisection_generator::build(24, rng),
            _revealed: HashSet::new(),
        }
    }

    pub fn visible_tiles(&self) -> impl Iterator<Item = (Position, &Tile)> {
        self.tiles
            .iter()
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
