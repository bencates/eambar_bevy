use {
    crate::{
        assets::TextSprite,
        character::{CharacterBundle, Name},
        level::{attach_to_level, LocationBundle, Viewshed},
        movement::BlocksMovement,
    },
    bevy::prelude::*,
};

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(PostStartup, attach_to_level::<Monster>);
    }
}

#[derive(Component)]
pub struct Monster;

#[derive(Bundle)]
struct MonsterBundle {
    marker: Monster,
    character: CharacterBundle,
}

fn spawn(mut commands: Commands, text_sprite: Res<TextSprite>) {
    for position in [(0, -2), (2, 0), (-2, 2)] {
        commands.spawn(MonsterBundle {
            marker: Monster,
            character: CharacterBundle {
                name: Name("Infected Crewmember".to_string()),
                blocks_movement: BlocksMovement,
                location: LocationBundle {
                    position: position.into(),
                    z_index: 9.into(),
                },
                viewshed: Viewshed::new(8),
                sprite: SpriteSheetBundle {
                    texture_atlas: text_sprite.clone().into(),
                    sprite: TextureAtlasSprite {
                        index: TextSprite::char_index('z'),
                        color: Color::RED,
                        ..default()
                    },
                    ..default()
                },
            },
        });
    }
}
