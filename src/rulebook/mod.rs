mod movement;
mod visibility;

pub use movement::{BlocksMovement, MoveEvent};
pub use visibility::Viewshed;

use crate::prelude::*;

pub struct RulebookPlugin;

impl Plugin for RulebookPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveEvent>()
            .add_systems(
                Update,
                (
                    visibility::calculate_field_of_view,
                    movement::handle_move_event,
                ),
            )
            .add_systems(
                PostUpdate,
                visibility::show_in_player_field_of_view::<Character>,
            );
    }
}
