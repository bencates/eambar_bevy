mod fog;
mod location;
mod map_builder;
mod map_tile;

pub use {
    location::{CompassDirection, Position},
    map_tile::{MapTile, MapTileBundle},
};

use crate::{bestiary, prelude::*};
use map_builder::MapBuilder;

pub const TILE_RADIUS: f32 = 8.;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn).add_systems(
            PostUpdate,
            (
                fog::show_outside_player_viewshed,
                map_tile::reveal_visible_map_tiles,
                location::move_to_location,
                center_under_player,
            ),
        );
    }
}

#[derive(Component, Deref)]
pub struct Level(HashMap<Position, Entity>);

fn spawn(world: &mut World) {
    let tile_ids = MapBuilder::new(
        rand::thread_rng(),
        &[
            (75, bestiary::infected_crewmember),
            (25, bestiary::alien_hatchling),
        ],
    )
    .empty_hexagon(24)
    .run_bisection_generator(24)
    .spawn(world);

    let tiles: Vec<_> = tile_ids.values().cloned().collect();

    world
        .spawn((
            Level(tile_ids),
            SpatialBundle {
                transform: Transform {
                    scale: Vec3::splat(1.4),
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&tiles);
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
