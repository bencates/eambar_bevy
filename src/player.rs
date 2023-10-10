use {
    crate::{
        assets::TextSprite,
        character::{CharacterBundle, Name},
        level::{attach_to_level, CompassDirection::*, LocationBundle, Viewshed},
        movement::BlocksMovement,
        movement::MoveEvent,
    },
    bevy::prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(PostStartup, attach_to_level::<Player>)
            .add_systems(Update, keyboard_input);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    character: CharacterBundle,
}

fn spawn(mut commands: Commands, text_sprite: Res<TextSprite>) {
    commands.spawn(PlayerBundle {
        marker: Player,
        character: CharacterBundle {
            name: Name("Player".to_string()),
            blocks_movement: BlocksMovement,
            location: LocationBundle {
                position: (0, 0).into(),
                z_index: 10.into(),
            },
            viewshed: Viewshed::new(8),
            sprite: SpriteSheetBundle {
                texture_atlas: text_sprite.clone().into(),
                sprite: TextureAtlasSprite {
                    index: TextSprite::char_index('@'),
                    color: Color::YELLOW,
                    ..default()
                },
                ..default()
            },
        },
    });
}

fn keyboard_input(
    query: Query<Entity, With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut move_action: EventWriter<MoveEvent>,
) {
    let player_id = query.single();

    if keys.just_pressed(KeyCode::Q) {
        move_action.send(MoveEvent(player_id, NorthWest));
    }
    if keys.just_pressed(KeyCode::W) {
        move_action.send(MoveEvent(player_id, North));
    }
    if keys.just_pressed(KeyCode::E) {
        move_action.send(MoveEvent(player_id, NorthEast));
    }
    if keys.just_pressed(KeyCode::A) {
        move_action.send(MoveEvent(player_id, SouthWest));
    }
    if keys.just_pressed(KeyCode::S) {
        move_action.send(MoveEvent(player_id, South));
    }
    if keys.just_pressed(KeyCode::D) {
        move_action.send(MoveEvent(player_id, SouthEast));
    }
}
