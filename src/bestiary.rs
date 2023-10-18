use crate::prelude::*;

pub fn infected_crewmember(text_sprite: &TextSprite) -> CharacterBundle {
    CharacterBundle {
        marker: Character::Monster,
        name: Name::new("Infected Crewmember"),
        health: Health::new(16),
        initiative: Initiative::new(8),
        melee_damage: MeleeDamage(4),
        blocks_movement: BlocksMovement,
        viewshed: Viewshed::new(8),
        sprite: text_sprite.build('z', Color::RED, Character::Z_INDEX),
    }
}

pub fn alien_hatchling(text_sprite: &TextSprite) -> CharacterBundle {
    CharacterBundle {
        marker: Character::Monster,
        name: Name::new("Alien Hatchling"),
        health: Health::new(16),
        initiative: Initiative::new(4),
        melee_damage: MeleeDamage(4),
        blocks_movement: BlocksMovement,
        viewshed: Viewshed::new(8),
        sprite: text_sprite.build('h', Color::RED, Character::Z_INDEX),
    }
}
