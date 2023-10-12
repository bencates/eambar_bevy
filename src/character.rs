use crate::{level::attach_to_level, prelude::*};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, attach_to_level::<Character>);
    }
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component, Default)]
pub enum Character {
    Player,
    #[default]
    Monster,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub marker: Character,
    pub name: Name,
    pub blocks_movement: BlocksMovement,
    pub viewshed: Viewshed,
    pub sprite: SpriteSheetBundle,
}
