use crate::prelude::*;

#[derive(Component)]
pub struct Health(i32, i32);

#[derive(Event)]
pub(super) struct DamageEvent(pub(super) Entity, pub(super) i32);

impl Health {
    pub fn new(health: i32) -> Self {
        Self(health, health)
    }

    pub fn current(&self) -> i32 {
        self.0
    }

    pub fn max(&self) -> i32 {
        self.1
    }

    fn take_damage(&mut self, damage: i32) {
        self.0 -= damage;
    }

    fn is_alive(&self) -> bool {
        self.0 > 0
    }
}

pub(super) fn apply_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut health_pools: Query<&mut Health>,
    names: Query<&Name>,
) {
    for &DamageEvent(entity, damage) in damage_events.iter() {
        if let Ok(mut health) = health_pools.get_mut(entity) {
            health.take_damage(damage);

            if let Ok(name) = names.get(entity) {
                debug!("{name} took {damage} damage, at {}/{}", health.0, health.1);
            }
        }
    }
}

pub(super) fn cull_the_dead(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Name>), Changed<Health>>,
) {
    for (entity, health, name) in &query {
        if !health.is_alive() {
            if let Some(name) = name {
                debug!("{name} died");
            }

            commands.entity(entity).despawn();
        }
    }
}
