use crate::prelude::*;

#[derive(Event)]
pub struct MeleeEvent(pub Entity, pub Entity);

pub(super) fn resolve_melee_attacks(
    mut events: EventReader<MeleeEvent>,
    mut turns: EventWriter<SpendTurnEvent>,
) {
    for &MeleeEvent(attacker_id, target_id) in events.iter() {
        debug!("{attacker_id:?} attacking {target_id:?}");

        turns.send(SpendTurnEvent(attacker_id))
    }
}
