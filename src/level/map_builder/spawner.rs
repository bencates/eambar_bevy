use super::Tiles;
use crate::prelude::*;
use hex2d::Coordinate;

pub(super) fn spawn_map_tiles(tiles: &Tiles, world: &mut World) -> HashMap<Position, Entity> {
    tiles
        .iter()
        .map(|(&coord, &tile)| {
            let position = coord.into();
            let assets = world.resource();

            let mut tile_entity = world.spawn(MapTileBundle::new(tile, position, assets));

            if tile.is_blocked() {
                tile_entity.insert(BlocksMovement);
            }

            (position, tile_entity.id())
        })
        .collect()
}

pub(super) fn spawn_player(coord: Coordinate, world: &mut World) -> Entity {
    let text_sprite = world.resource();

    world
        .spawn((
            PlayerBundle::new(text_sprite),
            LocationBundle {
                position: coord.into(),
                z_index: 10.into(),
            },
        ))
        .id()
}

pub(super) fn spawn_monster(
    coord: Coordinate,
    builder: impl FnOnce(&TextSprite) -> CharacterBundle,
    world: &mut World,
) -> Entity {
    let text_sprite = world.resource();

    world
        .spawn((
            builder(text_sprite),
            LocationBundle {
                position: coord.into(),
                z_index: 9.into(),
            },
        ))
        .id()
}
