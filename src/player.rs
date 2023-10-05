use {
    crate::{
        assets::TextSprite,
        map::{Map, Position},
    },
    bevy::prelude::*,
    hex2d::Direction,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveEvent>()
            .add_systems(Startup, spawn)
            .add_systems(Update, (keyboard_input, handle_move_event));
    }
}

/// Marker component for the player entity
#[derive(Component)]
pub struct Player;

fn spawn(mut commands: Commands, text_sprite: Res<TextSprite>) {
    commands.spawn((
        Player,
        text_sprite.bundle('@', Color::YELLOW, Position::new(0, 0, 1)),
    ));
}

#[derive(Debug, Event)]
pub struct MoveEvent(Direction);

fn keyboard_input(keys: Res<Input<KeyCode>>, mut action: EventWriter<MoveEvent>) {
    if keys.just_pressed(KeyCode::Q) {
        action.send(MoveEvent(Direction::ZX));
    }
    if keys.just_pressed(KeyCode::W) {
        action.send(MoveEvent(Direction::ZY));
    }
    if keys.just_pressed(KeyCode::E) {
        action.send(MoveEvent(Direction::XY));
    }
    if keys.just_pressed(KeyCode::A) {
        action.send(MoveEvent(Direction::YX));
    }
    if keys.just_pressed(KeyCode::S) {
        action.send(MoveEvent(Direction::YZ));
    }
    if keys.just_pressed(KeyCode::D) {
        action.send(MoveEvent(Direction::XZ));
    }
}

fn handle_move_event(
    mut events: EventReader<MoveEvent>,
    mut query: Query<&mut Transform, With<Player>>,
    map: Res<Map>,
) {
    for MoveEvent(dir) in events.iter() {
        for mut transform in &mut query {
            let pos = Position::from(transform.translation).step(*dir);

            debug!("new pos: {pos:?}");

            if !map[&pos].is_blocked() {
                transform.translation = pos.into()
            }
        }
    }
}
