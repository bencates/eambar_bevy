mod initiative;
mod movement;
mod visibility;

pub use initiative::{HasInitiative, Initiative, SpendTurnEvent};
pub use movement::{BlocksMovement, MoveEvent};
pub use visibility::Viewshed;

use crate::prelude::*;

pub struct RulebookPlugin;

impl Plugin for RulebookPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveEvent>()
            .add_event::<SpendTurnEvent>()
            .add_systems(PreUpdate, initiative::assign_initiative)
            .add_systems(
                Update,
                (
                    visibility::calculate_field_of_view,
                    movement::handle_move_event,
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    initiative::spend_turn,
                    visibility::show_in_player_field_of_view::<Character>,
                ),
            );
    }
}
