mod bisection_generator;

use bevy::prelude::{debug, Component, Resource, Vec3};
use bevy::utils::{HashMap, HashSet};
use hex2d::{Coordinate, Direction, Spacing};
use rand::Rng;
use std::ops::Index;

// const MAP_WIDTH: i32 = 49;
// const MAP_HEIGHT: i32 = 49;

const TILE_SIZE: Spacing = Spacing::FlatTop(8.);

#[derive(Component, Debug, PartialEq)]
pub struct Position(Coordinate, f32);

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Coordinate::new(x, y), z as f32)
    }

    pub fn step(&self, dir: Direction) -> Self {
        Self(self.0 + dir, self.1)
    }
}

impl From<Vec3> for Position {
    fn from(Vec3 { x, y, z }: Vec3) -> Self {
        Self(Coordinate::from_pixel(x, y, TILE_SIZE), z)
    }
}

impl From<Position> for Vec3 {
    fn from(Position(coord, z): Position) -> Self {
        let (x, y) = coord.to_pixel(TILE_SIZE);

        Vec3::new(x, y, z)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    pub fn is_blocked(&self) -> bool {
        self == &Tile::Wall
    }
}

#[derive(Resource)]
pub struct Map {
    tiles: HashMap<Coordinate, Tile>,
    _revealed: HashSet<Coordinate>,
}

impl Map {
    pub fn new(rng: &mut impl Rng) -> Self {
        debug!("building a new map");

        Self {
            tiles: bisection_generator::build(24, rng),
            _revealed: HashSet::new(),
        }
    }

    pub fn visible_tiles(&self) -> impl Iterator<Item = (Position, &Tile)> {
        self.tiles
            .iter()
            .map(|(coord, tile)| (Position(*coord, 0.), tile))
    }
}

impl Index<&Position> for Map {
    type Output = Tile;

    fn index(&self, Position(coord, _): &Position) -> &Tile {
        self.tiles.get(coord).unwrap_or(&Tile::Floor)
    }
}
