use crate::prelude::*;

#[derive(Component)]
pub struct HasInitiative;

#[derive(Component)]
pub struct Initiative {
    current: i32,
    speed: i32,
}

#[derive(Event)]
pub struct SpendTurnEvent(pub Entity);

impl Initiative {
    pub fn new(speed: i32) -> Self {
        Self {
            current: speed,
            speed,
        }
    }
}

pub(super) fn assign_initiative(
    mut commands: Commands,
    has_initiative_query: Query<&HasInitiative>,
    mut initiatives: Query<(Entity, &mut Initiative)>,
) {
    if !has_initiative_query.is_empty() {
        return;
    }

    if let Some((
        entity,
        &Initiative {
            current: min_initiative,
            ..
        },
    )) = initiatives.iter().min_by_key(|(_, init)| init.current)
    {
        commands.entity(entity).insert(HasInitiative);

        for (_, mut initiative) in &mut initiatives {
            initiative.current -= min_initiative;
        }
    }
}

pub(super) fn spend_turn(
    mut events: EventReader<SpendTurnEvent>,
    mut commands: Commands,
    mut initiatives: Query<&mut Initiative>,
) {
    for &SpendTurnEvent(entity) in events.iter() {
        commands.entity(entity).remove::<HasInitiative>();

        if let Ok(mut initiative) = initiatives.get_mut(entity) {
            initiative.current = initiative.speed;
        }
    }
}
