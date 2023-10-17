use super::DamageEvent;
use crate::prelude::*;

#[derive(Component)]
pub struct MeleeDamage(pub i32);

#[derive(Event)]
pub struct MeleeEvent(pub Entity, pub Entity);

pub(super) fn resolve_melee_attacks(
    mut events: EventReader<MeleeEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    mut turns: EventWriter<SpendTurnEvent>,
    damage_pools: Query<&MeleeDamage>,
    names: Query<&Name>,
) {
    for &MeleeEvent(attacker_id, target_id) in events.iter() {
        if let Ok(damage) = damage_pools.get(attacker_id) {
            if let Ok([attacker, target]) = names.get_many([attacker_id, target_id]) {
                debug!("{attacker} meleed {target}");
            }

            damage_events.send(DamageEvent(target_id, damage.0));
            turns.send(SpendTurnEvent(attacker_id));
        }
    }
}
