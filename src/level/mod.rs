mod bisection_generator;
mod map;
mod position;

pub use {
    map::{Map, Tile},
    position::Position,
};

use {crate::assets::HexagonMesh, bevy::prelude::*, field_of_view::calculate_field_of_view};

pub const TILE_RADIUS: f32 = 8.;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();

        app.insert_resource(Map::new(&mut rng))
            .add_systems(Startup, draw_map_tiles);
    }
}

fn draw_map_tiles(
    mut commands: Commands,
    hexagon: Res<HexagonMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
) {
    let floor_color = materials.add(ColorMaterial::from(Color::DARK_GRAY));
    let wall_color = materials.add(ColorMaterial::from(Color::GRAY));

    commands.spawn_batch(
        map.visible_tiles()
            .map(|(pos, tile)| ColorMesh2dBundle {
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
                ..default()
            })
            .collect::<Vec<_>>(),
    );
}
