use super::Tiles;
use crate::{level::fog::Fog, prelude::*};

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

pub(super) fn spawn_fog(pos: Position, tile_id: Entity, world: &mut World) {
    let assets = world.resource();

    world.spawn((Fog::bundle(assets), pos)).set_parent(tile_id);
}

pub(super) fn spawn_player(pos: Position, world: &mut World) -> Entity {
    let text_sprite = world.resource();

    world.spawn((PlayerBundle::new(text_sprite), pos)).id()
}

pub(super) fn spawn_monster(
    pos: Position,
    template: CharacterTemplate,
    world: &mut World,
) -> Entity {
    let text_sprite = world.resource();

    world.spawn((template.build(text_sprite), pos)).id()
}
