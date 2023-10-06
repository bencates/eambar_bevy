mod assets;
mod map;
mod movement;
mod player;

use {
    crate::assets::TextSprite,
    bevy::{log::LogPlugin, prelude::*},
};

// const TERM_WIDTH: i32 = 80;
// const TERM_HEIGHT: i32 = 50;

fn main() {
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
            map::MapPlugin,
            movement::MovementPlugin,
            player::PlayerPlugin,
        ))
        .init_resource::<TextSprite>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
