use {
    super::UI,
    crate::{
        character::Name,
        level::{CompassDirection, Position},
    },
    bevy::prelude::*,
};

#[derive(Component)]
pub struct LogWindowNode;

#[derive(Event)]
pub enum LogEvent {
    Move(Entity, CompassDirection, Position),
}

pub(super) fn draw_log_window(ui: Res<UI>, mut commands: Commands) {
    commands.entity(ui.log_frame).with_children(|frame| {
        frame
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                ..default()
            })
            .with_children(|scroll_container| {
                scroll_container.spawn((
                    LogWindowNode,
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                ));
            });
    });
}

pub(super) fn collect_log_events(
    mut commands: Commands,
    mut log_events: EventReader<LogEvent>,
    log_window_query: Query<Entity, With<LogWindowNode>>,
    names: Query<&Name>,
) {
    let mut log_window = commands.entity(log_window_query.single());

    for event in log_events.iter() {
        let log_entry = match event {
            LogEvent::Move(entity, dir, _pos) => names
                .get(*entity)
                .ok()
                .map(|Name(name)| format!("{name} moved {dir}")),
        };

        if let Some(log_entry) = log_entry {
            log_window.with_children(|parent| {
                parent.spawn(TextBundle::from_section(log_entry, TextStyle::default()));
            });
        }
    }
}

pub(super) fn scroll_log_window(
    mut query: Query<(&Parent, &Node, &mut Style), With<LogWindowNode>>,
    nodes: Query<&Node>,
) {
    let (container, log, mut style) = query.single_mut();

    let log_height = log.size().y;
    let container_height = nodes.get(container.get()).unwrap().size().y;

    let offset = (log_height - container_height).max(0.);

    style.top = Val::Px(-offset);
}
