mod bisection_generator;
mod spawner;

use crate::prelude::*;
use hex2d::{Coordinate, Direction::*, Spin};

type Tiles = HashMap<Coordinate, MapTile>;

const ORIGIN: Coordinate = Coordinate { x: 0, y: 0 };

pub struct MapBuilder<R: Rng> {
    rng: R,
    tiles: Tiles,
    player_origin: Option<Coordinate>,
    spawn_points: Vec<Coordinate>,
    spawn_table: SpawnTable,
}

impl<R: Rng> MapBuilder<R> {
    pub fn new(rng: R, spawn_table: &[(i32, &CharacterTemplate)]) -> Self {
        Self {
            rng,
            tiles: HashMap::new(),
            player_origin: Some((0, 0).into()),
            spawn_points: Vec::new(),
            spawn_table: SpawnTable::new(spawn_table),
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

    pub fn random_spawns(mut self, count: i32) -> Self {
        self.spawn_points = self
            .tiles
            .iter()
            .filter_map(|(&coord, &tile)| (tile == MapTile::Floor).then_some(coord))
            .choose_multiple(&mut self.rng, count as usize);

        self
    }

    pub fn spawn(mut self, world: &mut World) -> HashMap<Position, Entity> {
        let tile_ids: HashMap<Position, Entity> = spawner::spawn_map_tiles(&self.tiles, world);

        for (&pos, &tile_id) in tile_ids.iter() {
            spawner::spawn_fog(pos, tile_id, world)
        }

        if let Some(coord) = self.player_origin {
            let _player_id = spawner::spawn_player(coord.into(), world);
        }

        for coord in self.spawn_points {
            let template = self.spawn_table.sample(&mut self.rng);
            let _monster_id = spawner::spawn_monster(coord.into(), template, world);
        }

        tile_ids
    }
}

fn is_blocked(tiles: &Tiles, coord: &Coordinate) -> bool {
    tiles.get(coord).is_some_and(|tile| tile.is_blocked())
}
