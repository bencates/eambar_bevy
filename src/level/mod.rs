mod bisection_generator;
mod field_of_view;
mod map;
mod map_tile;
mod position;

pub use {field_of_view::Viewshed, map::Map, map_tile::Tile, position::Position};

use {
    crate::player::Player,
    bevy::prelude::*,
    field_of_view::{
        calculate_field_of_view, draw_fog_outside_player_viewshed, update_map_visibility,
    },
    map_tile::{draw_map_tiles, reveal_visible_map_tiles},
};

pub const TILE_RADIUS: f32 = 8.;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();

        app.insert_resource(Map::new(&mut rng))
            .add_systems(Startup, spawn)
            .add_systems(PostStartup, draw_map_tiles)
            .add_systems(
                Update,
                (calculate_field_of_view, draw_fog_outside_player_viewshed),
            )
            .add_systems(
                PostUpdate,
                (
                    update_map_visibility,
                    reveal_visible_map_tiles,
                    center_under_player,
                ),
            );
    }
}

#[derive(Component)]
pub struct Level;

fn spawn(mut commands: Commands) {
    commands.spawn((
        Level,
        SpatialBundle {
            transform: Transform {
                scale: Vec3::splat(1.4),
                ..default()
            },
            ..default()
        },
    ));
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
