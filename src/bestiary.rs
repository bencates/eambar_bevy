use crate::prelude::*;

pub fn infected_crewmember(text_sprite: &TextSprite) -> CharacterBundle {
    CharacterBundle {
        marker: Character::Monster,
        name: Name("Infected Crewmember".to_string()),
        initiative: Initiative::new(8),
        blocks_movement: BlocksMovement,
        viewshed: Viewshed::new(8),
        sprite: text_sprite.build('z', Color::RED),
    }
}

pub fn alien_hatchling(text_sprite: &TextSprite) -> CharacterBundle {
    CharacterBundle {
        marker: Character::Monster,
        name: Name("Alien Hatchling".to_string()),
        initiative: Initiative::new(4),
        blocks_movement: BlocksMovement,
        viewshed: Viewshed::new(8),
        sprite: text_sprite.build('h', Color::RED),
    }
}
