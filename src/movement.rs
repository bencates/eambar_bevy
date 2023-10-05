use {
    crate::map::{Map, Position},
    bevy::prelude::*,
    hex2d::Direction,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveEvent>().add_systems(
            Update,
            (handle_move_event, move_to_position.after(handle_move_event)),
        );
    }
}

#[derive(Debug, Event)]
pub struct MoveEvent(pub Entity, pub Direction);

fn handle_move_event(
    mut events: EventReader<MoveEvent>,
    mut query: Query<&mut Position>,
    map: Res<Map>,
) {
    for MoveEvent(entity, dir) in events.iter() {
        if let Ok(mut pos) = query.get_mut(*entity) {
            let new_pos = *pos + *dir;

            debug!("new pos: {pos:?}");

            if !map[&new_pos].is_blocked() {
                *pos = new_pos;
            }
        }
    }
}

fn move_to_position(mut query: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (&pos, mut transform) in &mut query {
        transform.translation = pos.into();
    }
}
