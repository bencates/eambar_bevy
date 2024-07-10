mod fog;
mod location;
mod map_builder;
mod map_tile;
mod target_reticle;

pub use {
    location::{CompassDirection, Position},
    map_tile::{MapTile, MapTileBundle},
};

use crate::prelude::*;
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
                target_reticle::draw,
                location::move_to_location,
                center_under_player,
            ),
        );
    }
}

#[derive(Component, Deref)]
pub struct Level(HashMap<Position, Entity>);

fn spawn(world: &mut World) {
    let templates: &CharacterTemplates = world.resource();

    let tile_ids = MapBuilder::new(
        rand::thread_rng(),
        &[
            (75, &templates["infected_crewmember"]),
            (25, &templates["alien_hatchling"]),
        ],
    )
    .empty_hexagon(24)
    .run_bisection_generator(24)
    .random_spawns(15)
    .spawn(world);

    let tiles: Vec<_> = tile_ids.values().cloned().collect();

    world
        .spawn((Level(tile_ids), SpatialBundle::default()))
        .push_children(&tiles);
}

fn center_under_player(
    player_query: Query<&Position, (With<Player>, Changed<Position>)>,
    mut level_query: Query<&mut Transform, With<Level>>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        let mut level_transform = level_query.single_mut();

        let px = player_pos.to_world_pos();

        level_transform.translation = (-px, 0.).into();
    }
}
