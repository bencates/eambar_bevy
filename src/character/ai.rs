use crate::prelude::*;

pub(super) fn take_turn(
    query: Query<Entity, (With<HasInitiative>, Without<Player>)>,
    mut turns: EventWriter<SpendTurnEvent>,
) {
    for entity in &query {
        debug!("{entity:?} taking turn");

        turns.send(SpendTurnEvent(entity));
    }
}
