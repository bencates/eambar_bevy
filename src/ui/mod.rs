mod player_stats;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UI>()
            .add_systems(PostStartup, player_stats::draw)
            .add_systems(PostUpdate, player_stats::update_health);
    }
}

#[derive(Resource)]
pub struct UI {
    player_stats_frame: Entity,
}

impl FromWorld for UI {
    fn from_world(world: &mut World) -> Self {
        let player_stats_frame = world
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),

                    width: Val::Px(300.),
                    height: Val::Px(150.),
                    border: UiRect {
                        right: Val::Px(3.),
                        bottom: Val::Px(3.),
                        ..default()
                    },
                    padding: UiRect::all(Val::Px(6.)),

                    flex_direction: FlexDirection::Column,

                    ..default()
                },
                background_color: Color::BLACK.into(),
                border_color: Color::WHITE.into(),
                ..default()
            })
            .id();

        UI { player_stats_frame }
    }
}
