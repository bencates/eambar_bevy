mod game_log;
mod player_stats;

pub use game_log::LogEvent;

use bevy::prelude::*;
use game_log::{collect_log_events, draw_log_window, scroll_log_window};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogEvent>()
            .init_resource::<UI>()
            .add_systems(PostStartup, (draw_log_window, player_stats::draw))
            .add_systems(
                PostUpdate,
                (
                    player_stats::update_health,
                    (collect_log_events, scroll_log_window).chain(),
                ),
            );
    }
}

#[derive(Resource)]
pub struct UI {
    player_stats_frame: Entity,
    log_frame: Entity,
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

        let log_frame = world
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.),
                    bottom: Val::Px(0.),

                    width: Val::Vw(30.),
                    height: Val::Px(200.),
                    border: UiRect {
                        top: Val::Px(3.),
                        left: Val::Px(3.),
                        ..default()
                    },
                    padding: UiRect::all(Val::Px(6.)),

                    ..default()
                },
                background_color: Color::BLACK.into(),
                border_color: Color::WHITE.into(),
                ..default()
            })
            .id();

        UI {
            player_stats_frame,
            log_frame,
        }
    }
}
