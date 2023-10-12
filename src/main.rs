mod assets;
mod bestiary;
mod character;
mod level;
mod movement;
mod player;
mod spawn_table;
mod ui;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::utils::{HashMap, HashSet};
    pub use rand::prelude::*;

    // bundles
    pub use crate::character::CharacterBundle;
    pub use crate::level::{LocationBundle, MapTileBundle};
    pub use crate::player::PlayerBundle;

    // components
    pub use crate::character::{Character, Name};
    // pub use crate::level::Fog;
    pub use crate::level::{Level, MapTile, Position, Viewshed, ZIndex};
    pub use crate::movement::BlocksMovement;
    pub use crate::player::Player;

    // events
    pub use crate::movement::MoveEvent;
    pub use crate::ui::LogEvent;

    //resources
    pub use crate::assets::{MapAssets, TextSprite};
    pub use crate::ui::UI;

    // misc
    pub use crate::level::CompassDirection::{self, *};
    pub use crate::spawn_table::{SpawnFn, SpawnTable};
}

use bevy::log::LogPlugin;
use prelude::*;

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
