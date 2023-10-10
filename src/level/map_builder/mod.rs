mod bisection_generator;

use {
    super::{Map, Tile},
    bevy::utils::HashMap,
    hex2d::{Coordinate, Direction::*, Spin},
    rand::prelude::*,
};

type Tiles = HashMap<Coordinate, Tile>;

const ORIGIN: Coordinate = Coordinate { x: 0, y: 0 };

pub struct MapBuilder<R: Rng> {
    tiles: Tiles,
    rng: R,
}

impl<R: Rng> MapBuilder<R> {
    pub fn new(rng: R) -> Self {
        Self {
            tiles: HashMap::new(),
            rng,
        }
    }

    pub fn empty_hexagon(mut self, radius: i32) -> Self {
        self.tiles = ORIGIN
            .ring_iter(radius, Spin::CW(ZX))
            .map(|c| (c, Tile::Wall))
            .collect();

        self.tiles
            .extend(ORIGIN.range_iter(radius - 1).map(|c| (c, Tile::Floor)));

        self
    }

    pub fn run_bisection_generator(mut self, radius: i32) -> Self {
        bisection_generator::run(&mut self.tiles, radius, &mut self.rng);

        self
    }
}

impl<R: Rng> From<MapBuilder<R>> for Map {
    fn from(builder: MapBuilder<R>) -> Self {
        let tiles = builder
            .tiles
            .iter()
            .map(|(&coord, &tile)| (coord.into(), tile))
            .collect();

        Map::new(tiles)
    }
}

fn is_blocked(tiles: &Tiles, coord: &Coordinate) -> bool {
    tiles.get(coord).is_some_and(|tile| tile.is_blocked())
}
