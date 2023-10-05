use crate::map::Position;
use bevy::prelude::*;

#[derive(Resource)]
pub struct TextSprite {
    texture_atlas: Handle<TextureAtlas>,
}

impl FromWorld for TextSprite {
    fn from_world(world: &mut World) -> Self {
        let texture = world.resource::<AssetServer>().load("terminal8x8.png");

        let texture_atlas = world
            .resource_mut::<Assets<_>>()
            .add(TextureAtlas::from_grid(
                texture,
                (8., 8.).into(),
                16,
                16,
                None,
                None,
            ));

        Self { texture_atlas }
    }
}

impl TextSprite {
    pub fn bundle(&self, c: char, color: Color, pos: Position) -> SpriteSheetBundle {
        SpriteSheetBundle {
            texture_atlas: self.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: match c {
                    '#' => 35,
                    '.' => 46,
                    '@' => 64,
                    _ => unimplemented!(),
                },
                color,
                ..default()
            },
            transform: Transform {
                translation: pos.into(),
                ..default()
            },
            ..default()
        }
    }
}
