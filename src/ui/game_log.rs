use {
    crate::{
        character::Name,
        level::{CompassDirection, Position},
    },
    bevy::prelude::*,
};

#[derive(Default, Resource)]
pub struct GameLog(Vec<String>);

#[derive(Event)]
pub enum LogEvent {
    Move(Entity, CompassDirection, Position),
}

pub(super) fn collect_log_events(
    mut log: ResMut<GameLog>,
    mut log_events: EventReader<LogEvent>,
    names: Query<&Name>,
) {
    for event in log_events.iter() {
        match event {
            &LogEvent::Move(entity, dir, pos) => {
                if let Ok(Name(name)) = names.get(entity) {
                    let log_entry = format!("{name} moved {dir}");
                    log.0.push(log_entry);

                    debug!("{name} moved {dir} to {pos}");
                }
            }
        }
    }
}
