use crate::prelude::*;

#[derive(Component)]
pub struct BlocksMovement;

#[derive(Debug, Event)]
pub struct MoveEvent(pub Entity, pub CompassDirection);

#[allow(clippy::type_complexity)]
pub(super) fn handle_move_event(
    mut events: EventReader<MoveEvent>,
    mut set: ParamSet<(Query<&mut Position>, Query<&Position, With<BlocksMovement>>)>,
    mut log: EventWriter<LogEvent>,
    mut turns: EventWriter<SpendTurnEvent>,
) {
    for &MoveEvent(entity, dir) in events.iter() {
        if let Ok(&old_pos) = set.p0().get(entity) {
            let new_pos = old_pos + dir;

            if set.p1().iter().all(|blocker_pos| *blocker_pos != new_pos) {
                *set.p0().get_mut(entity).unwrap() = new_pos;

                log.send(LogEvent::Move(entity, dir, new_pos));
                turns.send(SpendTurnEvent(entity))
            }
        }
    }
}
