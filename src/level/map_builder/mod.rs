mod bisection_generator;
mod spawner;

use {
    super::MapTile,
    bevy::{prelude::World, utils::HashMap},
    hex2d::{Coordinate, Direction::*, Spin},
    rand::prelude::*,
};

type Tiles = HashMap<Coordinate, MapTile>;

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
            .map(|c| (c, MapTile::Wall))
            .collect();

        self.tiles
            .extend(ORIGIN.range_iter(radius - 1).map(|c| (c, MapTile::Floor)));

        self
    }

    pub fn run_bisection_generator(mut self, radius: i32) -> Self {
        bisection_generator::run(&mut self.tiles, radius, &mut self.rng);

        self
    }

    pub fn spawn(self, world: &mut World) {
        let _tiles = spawner::spawn_map_tiles(&self.tiles, world);
    }
}

fn is_blocked(tiles: &Tiles, coord: &Coordinate) -> bool {
    tiles.get(coord).is_some_and(|tile| tile.is_blocked())
}
