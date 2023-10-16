mod initiative;
mod movement;
mod visibility;

pub use initiative::{HasInitiative, Initiative, SpendTurnEvent};
pub use movement::{BlocksMovement, MoveEvent};
pub use visibility::Viewshed;

use crate::prelude::*;

#[derive(Clone, Debug, Hash, SystemSet, PartialEq, Eq)]
pub struct PlanTurn;

pub struct RulebookPlugin;

impl Plugin for RulebookPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveEvent>()
            .add_event::<SpendTurnEvent>()
            .add_systems(
                PreUpdate,
                (
                    initiative::assign_initiative,
                    visibility::calculate_field_of_view,
                ),
            )
            .add_systems(Update, movement::handle_move_event.after(PlanTurn))
            .add_systems(
                PostUpdate,
                (
                    initiative::spend_turn,
                    visibility::show_in_player_field_of_view::<Character>,
                ),
            );
    }
}
