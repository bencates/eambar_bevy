use crate::prelude::*;

#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub enum MapTile {
    Floor,
    Wall,
}

impl MapTile {
    pub fn is_blocked(&self) -> bool {
        match self {
            MapTile::Wall => true,
            MapTile::Floor => false,
        }
    }

    pub fn is_opaque(&self) -> bool {
        match self {
            MapTile::Wall => true,
            MapTile::Floor => false,
        }
    }
}

#[derive(Bundle)]
pub struct MapTileBundle {
    tile: MapTile,
    position: Position,
    sprite: ColorMesh2dBundle,
}

impl MapTileBundle {
    pub fn new(tile: MapTile, position: Position, assets: &MapAssets) -> Self {
        let px = position.to_world_pos();
        MapTileBundle {
            tile,
            position,
            sprite: ColorMesh2dBundle {
                mesh: assets.hexagon.clone(),
                material: match tile {
                    MapTile::Floor => assets.floor_color.clone(),
                    MapTile::Wall => assets.wall_color.clone(),
                },
                transform: Transform {
                    translation: (px, 0.).into(),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
        }
    }
}

pub fn reveal_visible_map_tiles(
    player_query: Query<&Viewshed, (With<Player>, Changed<Viewshed>)>,
    mut tiles_query: Query<(&Position, &mut Visibility), With<MapTile>>,
) {
    if let Ok(viewshed) = player_query.get_single() {
        for (pos, mut visibility) in &mut tiles_query {
            if viewshed.includes(pos) {
                *visibility = Visibility::default();
            }
        }
    }
}
