mod map;

use bevy::prelude::*;
use hex2d::Direction;
use map::{Map, Position, Tile};

// const TERM_WIDTH: i32 = 80;
// const TERM_HEIGHT: i32 = 50;

const MAP_Z_INDEX: i32 = 0;
const PLAYER_Z_INDEX: i32 = 1;

fn main() {
    let mut rng = rand::thread_rng();

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Map::new(&mut rng))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Eambar".to_string(),
                        resolution: (50. * 16., 50. * 16.).into(),
                        ..default()
                    }),
                    ..default()
                })
                // don't alias pixel art
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, player_input)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
    map: Res<Map>,
) {
    let atlas = assets.add(TextureAtlas::from_grid(
        asset_server.load("terminal8x8.png"),
        (8., 8.).into(),
        16,
        16,
        None,
        None,
    ));

    commands.spawn(Camera2dBundle::default());

    for (pos, tile) in map.visible_tiles() {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: atlas.clone(),
            sprite: TextureAtlasSprite {
                index: match tile {
                    Tile::Floor => 46,
                    Tile::Wall => 35,
                },
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: pos.into(),
                ..default()
            },
            ..default()
        });
    }

    // Player
    commands.spawn((
        Player,
        SpriteSheetBundle {
            texture_atlas: atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 64,
                color: Color::YELLOW,
                ..default()
            },
            transform: Transform {
                translation: Position::new(0, 0, PLAYER_Z_INDEX).into(),
                ..default()
            },
            ..default()
        },
    ));
}

#[derive(Component)]
struct Player;

fn player_input(mut query: Query<&mut Transform, With<Player>>, keys: Res<Input<KeyCode>>) {
    for mut transform in &mut query {
        if keys.just_pressed(KeyCode::Q) {
            try_move(Direction::ZX, &mut transform);
        }
        if keys.just_pressed(KeyCode::W) {
            try_move(Direction::ZY, &mut transform);
        }
        if keys.just_pressed(KeyCode::E) {
            try_move(Direction::XY, &mut transform);
        }
        if keys.just_pressed(KeyCode::A) {
            try_move(Direction::YX, &mut transform);
        }
        if keys.just_pressed(KeyCode::S) {
            try_move(Direction::YZ, &mut transform);
        }
        if keys.just_pressed(KeyCode::D) {
            try_move(Direction::XZ, &mut transform);
        }
    }
}

fn try_move(dir: hex2d::Direction, transform: &mut Transform) {
    let pos = Position::from(transform.translation).step(dir);

    // transform.translation.x = (transform.translation.x + delta_x).clamp(-40. * 16., 39. * 16.);
    // transform.translation.y = (transform.translation.y + delta_y).clamp(-24. * 16., 25. * 16.);

    transform.translation = pos.into()
}
