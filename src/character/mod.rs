mod ai;

use crate::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ai::plan_turn.in_set(PlanTurn));
    }
}

#[derive(Component, Default)]
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
