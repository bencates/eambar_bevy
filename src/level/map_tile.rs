use {
    super::{Level, Map, Position, Viewshed},
    crate::{assets::HexagonMesh, player::Player},
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
    position: Position,
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
        for (coord, tile) in map.tiles() {
            let pos = Position::new(coord.x, coord.y, 0);

            parent.spawn(MapTileBundle {
                position: pos,
                tile: *tile,
                sprite: ColorMesh2dBundle {
                    mesh: hexagon.clone().into(),
                    material: match tile {
                        Tile::Floor => floor_color.clone(),
                        Tile::Wall => wall_color.clone(),
                    },
                    transform: Transform {
                        translation: pos.into(),
                        rotation: HexagonMesh::ROTATION,
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
            });
        }
    });
}

pub fn reveal_visible_map_tiles(
    player_query: Query<&Viewshed, (With<Player>, Changed<Viewshed>)>,
    mut tiles_query: Query<(&Position, &mut Visibility), With<Tile>>,
) {
    if let Ok(viewshed) = player_query.get_single() {
        for (pos, mut visibility) in &mut tiles_query {
            if viewshed.includes(pos.as_ref()) {
                *visibility = Visibility::Visible;
            }
        }
    }
}
