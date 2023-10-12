use {
    super::super::{map_tile::MapTileBundle, Position},
    super::Tiles,
    crate::movement::BlocksMovement,
    bevy::{prelude::*, utils::HashMap},
};

pub(super) fn spawn_map_tiles(tiles: &Tiles, world: &mut World) -> HashMap<Position, Entity> {
    tiles
        .iter()
        .map(|(&coord, &tile)| {
            let position: Position = coord.into();
            let assets = world.resource();

            let mut tile_entity = world.spawn(MapTileBundle::new(tile, position, assets));

            if tile.is_blocked() {
                tile_entity.insert(BlocksMovement);
            }

            (position, tile_entity.id())
        })
        .collect()
}
