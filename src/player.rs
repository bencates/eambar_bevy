use {
    crate::{
        assets::TextSprite,
        level::{Position, Viewshed},
        movement::MoveEvent,
    },
    bevy::prelude::*,
    hex2d::Direction::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveEvent>()
            .add_systems(Startup, spawn)
            .add_systems(Update, keyboard_input);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    position: Position,
    viewshed: Viewshed,
    sprite: SpriteSheetBundle,
}

impl PlayerBundle {
    fn new(text_sprite: TextSprite) -> Self {
        PlayerBundle {
            marker: Player,
            position: Position::new(0, 0, 10),
            viewshed: Viewshed::new(8),
            sprite: SpriteSheetBundle {
                texture_atlas: text_sprite.into(),
                sprite: TextureAtlasSprite {
                    index: TextSprite::char_index('@'),
                    color: Color::YELLOW,
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn spawn(mut commands: Commands, text_sprite: Res<TextSprite>) {
    commands.spawn(PlayerBundle::new(text_sprite.clone()));
}

fn keyboard_input(
    query: Query<Entity, With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut move_action: EventWriter<MoveEvent>,
) {
    let player = query.single();

    if keys.just_pressed(KeyCode::Q) {
        move_action.send(MoveEvent(player, ZX));
    }
    if keys.just_pressed(KeyCode::W) {
        move_action.send(MoveEvent(player, ZY));
    }
    if keys.just_pressed(KeyCode::E) {
        move_action.send(MoveEvent(player, XY));
    }
    if keys.just_pressed(KeyCode::A) {
        move_action.send(MoveEvent(player, YX));
    }
    if keys.just_pressed(KeyCode::S) {
        move_action.send(MoveEvent(player, YZ));
    }
    if keys.just_pressed(KeyCode::D) {
        move_action.send(MoveEvent(player, XZ));
    }
}
