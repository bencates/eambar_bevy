use crate::prelude::*;

#[derive(Component)]
pub(super) struct HealthBarNode;

pub(super) fn draw(
    ui: Res<UI>,
    mut commands: Commands,
    player_query: Query<(&Name, &Health), With<Player>>,
) {
    let (name, health) = player_query.single();

    commands
        .entity(ui.player_stats_frame)
        .with_children(|frame| {
            frame.spawn(TextBundle::from_section(name, TextStyle::default()));

            frame.spawn((
                HealthBarNode,
                TextBundle::from_section(
                    format!("HP: {} / {}", health.current(), health.max()),
                    TextStyle::default(),
                ),
            ));
        });
}

pub(super) fn update_health(
    mut text_query: Query<&mut Text, With<HealthBarNode>>,
    player_query: Query<&Health, With<Player>>,
) {
    let health = player_query.single();
    let mut text = text_query.single_mut();

    *text = Text::from_section(
        format!("HP: {} / {}", health.current(), health.max()),
        TextStyle::default(),
    );
}
