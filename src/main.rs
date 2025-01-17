mod assets;
mod character;
mod level;
mod player;
mod rulebook;
mod spawn_table;
mod ui;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::utils::{HashMap, HashSet};
    pub use rand::prelude::*;

    // bundles
    pub use crate::character::CharacterBundle;
    pub use crate::level::MapTileBundle;
    pub use crate::player::PlayerBundle;

    // components
    pub use crate::character::{Character, Target};
    pub use crate::level::{Level, MapTile, Position};
    pub use crate::player::Player;
    pub use crate::rulebook::{
        BlocksMovement, HasInitiative, Health, Initiative, MeleeDamage, Viewshed,
    };

    // events
    pub use crate::rulebook::{MeleeEvent, MoveEvent, SpendTurnEvent};

    // resources
    pub use crate::assets::{MapAssets, TextSprite};
    pub use crate::character::CharacterTemplates;
    pub use crate::ui::UI;

    // system sets
    pub use crate::rulebook::PlanTurn;

    // misc
    pub use crate::character::CharacterTemplate;
    pub use crate::level::CompassDirection::{self, *};
    pub use crate::spawn_table::SpawnTable;
}

use bevy::log::LogPlugin;
use prelude::*;

fn main() {
    App::new()
        .add_plugins(
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
        )
        .add_plugins((
            assets::AssetsPlugin,
            level::LevelPlugin,
            rulebook::RulebookPlugin,
            character::CharacterPlugin,
            player::PlayerPlugin,
            ui::UIPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 1. / 1.4,
            ..default()
        },
        ..default()
    });
}
