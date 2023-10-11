mod game_log;

pub use game_log::LogEvent;

use {
    bevy::prelude::*,
    game_log::{collect_log_events, GameLog},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameLog>()
            .add_event::<LogEvent>()
            .add_systems(PostUpdate, collect_log_events);
    }
}
