use {
    super::{Level, LocationBundle, Map, Position, Viewshed},
    crate::{assets::HexagonMesh, movement::BlocksMovement, player::Player},
    bevy::prelude::*,
};

#[derive(Clone, Component, Copy, Debug, PartialEq)]
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

#[derive(Bundle)]
struct MapTileBundle {
    tile: Tile,
    location: LocationBundle,
    sprite: ColorMesh2dBundle,
}

pub fn draw_map_tiles(
    mut commands: Commands,
    level_query: Query<Entity, With<Level>>,
    hexagon: Res<HexagonMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
) {
    let level = level_query.single();

    let floor_color = materials.add(ColorMaterial::from(Color::DARK_GRAY));
    let wall_color = materials.add(ColorMaterial::from(Color::GRAY));

    commands.entity(level).with_children(|parent| {
        for (position, &tile) in map.tiles() {
            let mut tile_commands = parent.spawn(MapTileBundle {
                tile,
                location: LocationBundle {
                    position: *position,
                    z_index: 0.into(),
                },
                sprite: ColorMesh2dBundle {
                    mesh: hexagon.clone().into(),
                    material: match tile {
                        Tile::Floor => floor_color.clone(),
                        Tile::Wall => wall_color.clone(),
                    },
                    transform: Transform {
                        rotation: HexagonMesh::ROTATION,
                        ..default()
                    },
                    visibility: match map.revealed().get(position) {
                        Some(_) => Visibility::Visible,
                        None => Visibility::Hidden,
                    },
                    ..default()
                },
            });

            if tile.is_blocked() {
                tile_commands.insert(BlocksMovement);
            }
        }
    });
}

pub fn reveal_visible_map_tiles(
    player_query: Query<&Viewshed, (With<Player>, Changed<Viewshed>)>,
    mut tiles_query: Query<(&Position, &mut Visibility), With<Tile>>,
) {
    if let Ok(viewshed) = player_query.get_single() {
        for (pos, mut visibility) in &mut tiles_query {
            if viewshed.includes(pos) {
                *visibility = Visibility::Visible;
            }
        }
    }
}
