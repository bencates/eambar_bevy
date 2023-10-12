mod assets;
mod bestiary;
mod character;
mod level;
mod movement;
mod player;
mod spawn_table;
mod ui;

use bevy::{log::LogPlugin, prelude::*};

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
                        ..default()
                    }),
                    ..default()
                })
                // don't alias pixel art
                .set(ImagePlugin::default_nearest()),
            assets::AssetsPlugin,
            level::LevelPlugin,
            character::CharacterPlugin,
            movement::MovementPlugin,
            player::PlayerPlugin,
            ui::UIPlugin,
        ))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
