use crate::prelude::*;
use serde::Deserialize;
use std::ops::Index;

const BESTIARY_RON: &str = include_str!("../../templates/bestiary.ron");

#[derive(Clone, Debug, Deserialize, Resource)]
pub struct CharacterTemplates(HashMap<String, CharacterTemplate>);

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterTemplate {
    marker: Character,
    name: String,
    health: i32,
    initiative: i32,
    melee_damage: i32,
    viewshed: i32,
    sprite: (char, Color),
}

impl CharacterTemplates {
    pub fn load() -> Self {
        Self(ron::from_str(BESTIARY_RON).unwrap())
    }
}

impl Index<&str> for CharacterTemplates {
    type Output = CharacterTemplate;

    fn index(&self, index: &str) -> &Self::Output {
        &self.0[index]
    }
}

impl CharacterTemplate {
    pub fn build(&self, text_sprite: &TextSprite) -> CharacterBundle {
        CharacterBundle {
            marker: self.marker,
            name: Name::new(self.name.clone()),
            health: Health::new(self.health),
            initiative: Initiative::new(self.initiative),
            melee_damage: MeleeDamage(self.melee_damage),
            blocks_movement: BlocksMovement,
            viewshed: Viewshed::new(self.viewshed),
            sprite: text_sprite.build(self.sprite.0, self.sprite.1, Character::Z_INDEX),
        }
    }
}
