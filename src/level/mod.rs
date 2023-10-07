mod bisection_generator;
mod field_of_view;
mod map;
mod position;

pub use {
    field_of_view::Viewshed,
    map::{Map, Tile},
    position::Position,
};

use {
    crate::{assets::HexagonMesh, player::Player},
    bevy::prelude::*,
    field_of_view::{
        calculate_field_of_view, draw_fog_outside_player_viewshed, reveal_visible_map_tiles,
    },
};

pub const TILE_RADIUS: f32 = 8.;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();

        app.insert_resource(Map::new(&mut rng))
            .add_systems(Startup, draw_map_tiles)
            .add_systems(
                Update,
                (
                    calculate_field_of_view,
                    draw_fog_outside_player_viewshed,
                    reveal_visible_map_tiles,
                ),
            )
            .add_systems(PostUpdate, (show_revealed_tiles, center_under_player));
    }
}

#[derive(Component)]
pub struct Level;

fn draw_map_tiles(
    mut commands: Commands,
    hexagon: Res<HexagonMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
) {
    let floor_color = materials.add(ColorMaterial::from(Color::DARK_GRAY));
    let wall_color = materials.add(ColorMaterial::from(Color::GRAY));

    commands
        .spawn((
            Level,
            SpatialBundle {
                transform: Transform {
                    scale: Vec3::splat(1.4),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            for (pos, tile) in map.visible_tiles() {
                parent.spawn((
                    pos,
                    *tile,
                    ColorMesh2dBundle {
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
                ));
            }
        });
}

pub fn attach_to_level<Child: Component>(
    mut commands: Commands,
    child_query: Query<Entity, With<Child>>,
    level_query: Query<Entity, With<Level>>,
) {
    let level = level_query.single();
    let children = child_query.iter().collect::<Vec<_>>();

    commands.entity(level).push_children(&children);
}

fn show_revealed_tiles(map: Res<Map>, mut query: Query<(&Position, &mut Visibility), With<Tile>>) {
    for (pos, mut visibility) in &mut query {
        if map.is_revealed(pos.as_ref()) {
            *visibility = Visibility::Visible;
        }
    }
}

fn center_under_player(
    player_query: Query<&Position, (With<Player>, Changed<Position>)>,
    mut level_query: Query<&mut Transform, With<Level>>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        let player_coord = player_pos.as_ref();
        let mut level_transform = level_query.single_mut();

        level_transform.translation =
            Vec3::from(Position::new(-player_coord.x, -player_coord.y, 0)) * level_transform.scale;
    }
}
