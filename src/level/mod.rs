mod field_of_view;
mod location;
mod map;
mod map_builder;
mod map_tile;

pub use {
    field_of_view::Viewshed,
    location::{CompassDirection, LocationBundle, Position, ZIndex},
    map::Map,
    map_tile::Tile,
};

use {
    crate::player::Player,
    bevy::prelude::*,
    field_of_view::{
        calculate_field_of_view, draw_fog_outside_player_viewshed, update_map_visibility,
    },
    location::move_to_location,
    map_builder::MapBuilder,
    map_tile::{draw_map_tiles, reveal_visible_map_tiles},
};

pub const TILE_RADIUS: f32 = 8.;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        let mut map: Map = MapBuilder::new(rand::thread_rng())
            .empty_hexagon(24)
            .run_bisection_generator(24)
            .into();

        map.reveal_all();

        app.insert_resource(map)
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
                    move_to_location,
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
        let mut level_transform = level_query.single_mut();

        let (x, y) = player_pos.to_pixel();

        level_transform.translation = Vec3::new(-x, -y, 0.) * level_transform.scale;
    }
}
