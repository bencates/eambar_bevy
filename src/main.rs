mod assets;
mod map;
mod movement;
mod player;

use {
    crate::{
        assets::TextSprite,
        map::{Map, Tile},
    },
    bevy::{log::LogPlugin, prelude::*},
};

// const TERM_WIDTH: i32 = 80;
// const TERM_HEIGHT: i32 = 50;

fn main() {
    let mut rng = rand::thread_rng();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    filter: "eambar=trace,wgpu=warn".to_string(),
                    ..default()
                })
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
            movement::MovementPlugin,
            player::PlayerPlugin,
        ))
        .init_resource::<TextSprite>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Map::new(&mut rng))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, text_sprite: Res<TextSprite>, map: Res<Map>) {
    commands.spawn(Camera2dBundle::default());

    for (pos, tile) in map.visible_tiles() {
        commands
            .spawn(text_sprite.bundle(
                match tile {
                    Tile::Floor => '.',
                    Tile::Wall => '#',
                },
                Color::WHITE,
            ))
            .insert(Transform {
                translation: pos.into(),
                ..default()
            });
    }
}
