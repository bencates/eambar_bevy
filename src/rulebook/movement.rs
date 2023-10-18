use crate::prelude::*;

#[derive(Component)]
pub struct BlocksMovement;

#[derive(Debug, Event)]
pub struct MoveEvent(pub Entity, pub Position);

#[allow(clippy::type_complexity)]
pub(super) fn handle_move_event(
    mut events: EventReader<MoveEvent>,
    mut set: ParamSet<(Query<&mut Position>, Query<&Position, With<BlocksMovement>>)>,
    mut turns: EventWriter<SpendTurnEvent>,
) {
    for &MoveEvent(entity, new_pos) in events.iter() {
        if set.p1().iter().all(|blocker_pos| *blocker_pos != new_pos) {
            *set.p0().get_mut(entity).unwrap() = new_pos;

            turns.send(SpendTurnEvent(entity))
        }
    }
}
