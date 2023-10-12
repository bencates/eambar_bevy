use crate::level::Viewshed;

use {
    crate::{
        assets::TextSprite,
        character::{Character, CharacterBundle, Name},
        movement::BlocksMovement,
    },
    bevy::prelude::*,
};

pub fn infected_crewmember(text_sprite: &TextSprite) -> CharacterBundle {
    CharacterBundle {
        marker: Character::Monster,
        name: Name("Infected Crewmember".to_string()),
        blocks_movement: BlocksMovement,
        viewshed: Viewshed::new(8),
        sprite: text_sprite.build('z', Color::RED),
    }
}

pub fn alien_hatchling(text_sprite: &TextSprite) -> CharacterBundle {
    CharacterBundle {
        marker: Character::Monster,
        name: Name("Alien Hatchling".to_string()),
        blocks_movement: BlocksMovement,
        viewshed: Viewshed::new(8),
        sprite: text_sprite.build('h', Color::RED),
    }
}
