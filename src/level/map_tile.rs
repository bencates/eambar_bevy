use {
    super::{LocationBundle, Position, Viewshed},
    crate::{assets::MapAssets, player::Player},
    bevy::prelude::*,
};

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
    location: LocationBundle,
    sprite: ColorMesh2dBundle,
}

impl MapTileBundle {
    pub fn new(tile: MapTile, position: Position, assets: &MapAssets) -> Self {
        MapTileBundle {
            tile,
            location: LocationBundle {
                position,
                z_index: 0.into(),
            },
            sprite: ColorMesh2dBundle {
                mesh: assets.hexagon.clone(),
                material: match tile {
                    MapTile::Floor => assets.floor_color.clone(),
                    MapTile::Wall => assets.wall_color.clone(),
                },
                transform: Transform {
                    rotation: MapAssets::HEX_ROTATION,
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
