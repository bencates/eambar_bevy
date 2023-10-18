mod combat;
mod durability;
mod initiative;
mod movement;
mod visibility;

pub use combat::{MeleeDamage, MeleeEvent};
pub use durability::Health;
pub use initiative::{HasInitiative, Initiative, SpendTurnEvent};
pub use movement::{BlocksMovement, MoveEvent};
pub use visibility::Viewshed;

use crate::prelude::*;
use durability::DamageEvent;

#[derive(Clone, Debug, Hash, SystemSet, PartialEq, Eq)]
pub struct PlanTurn;

#[derive(Clone, Debug, Hash, SystemSet, PartialEq, Eq)]
pub struct ResolveTurn;

pub struct RulebookPlugin;

impl Plugin for RulebookPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MeleeEvent>()
            .add_event::<MoveEvent>()
            .add_event::<DamageEvent>()
            .add_event::<SpendTurnEvent>()
            .add_systems(
                PreUpdate,
                (
                    initiative::assign_initiative,
                    visibility::calculate_field_of_view,
                ),
            )
            .add_systems(
                Update,
                (
                    (combat::resolve_melee_attacks, movement::handle_move_event),
                    durability::apply_damage,
                    durability::cull_the_dead,
                )
                    .chain()
                    .in_set(ResolveTurn)
                    .after(PlanTurn),
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
