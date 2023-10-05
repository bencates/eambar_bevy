use {
    crate::{assets::TextSprite, map::Position, movement::MoveEvent},
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
struct Player;

fn spawn(mut commands: Commands, text_sprite: Res<TextSprite>) {
    commands.spawn((
        Player,
        Position::new(0, 0, 1),
        text_sprite.bundle('@', Color::YELLOW),
    ));
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
