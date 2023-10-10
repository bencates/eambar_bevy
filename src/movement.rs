use {
    crate::level::{CompassDirection, Position},
    bevy::prelude::*,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveEvent>()
            .add_systems(Update, handle_move_event);
    }
}

#[derive(Component)]
pub struct BlocksMovement;

#[derive(Debug, Event)]
pub struct MoveEvent(pub Entity, pub CompassDirection);

#[allow(clippy::type_complexity)]
fn handle_move_event(
    mut events: EventReader<MoveEvent>,
    mut set: ParamSet<(Query<&mut Position>, Query<&Position, With<BlocksMovement>>)>,
) {
    for MoveEvent(entity, dir) in events.iter() {
        if let Ok(old_pos) = set.p0().get(*entity) {
            let new_pos = *old_pos + *dir;

            if set.p1().iter().all(|blocker_pos| *blocker_pos != new_pos) {
                debug!("new pos: {new_pos:?}");

                *set.p0().get_mut(*entity).unwrap() = new_pos;
            }
        }
    }
}
