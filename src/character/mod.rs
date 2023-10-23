mod ai;
mod target;
mod template;

pub use target::Target;
pub use template::{CharacterTemplate, CharacterTemplates};

use crate::prelude::*;
use serde::Deserialize;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CharacterTemplates::load())
            .add_systems(Update, ai::plan_turn.in_set(PlanTurn));
    }
}

#[derive(Clone, Copy, Component, Debug, Default, Deserialize)]
pub enum Character {
    Player,
    #[default]
    Monster,
}

impl Character {
    pub const Z_INDEX: f32 = 9.;
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub marker: Character,
    pub name: Name,
    pub health: Health,
    pub initiative: Initiative,
    pub melee_damage: MeleeDamage,
    pub blocks_movement: BlocksMovement,
    pub viewshed: Viewshed,
    pub sprite: SpriteSheetBundle,
}
