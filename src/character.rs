use {
    crate::{
        level::{LocationBundle, Viewshed},
        movement::BlocksMovement,
    },
    bevy::prelude::*,
};

#[derive(Component)]
pub struct Name(pub String);

#[derive(Bundle)]
pub struct CharacterBundle {
    pub name: Name,
    pub blocks_movement: BlocksMovement,
    pub location: LocationBundle,
    pub viewshed: Viewshed,
    pub sprite: SpriteSheetBundle,
}
