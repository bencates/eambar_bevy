use crate::{player::Target, prelude::*};
use bevy::ecs::query::QuerySingleError::*;

#[derive(Component)]
pub(super) struct TargetReticle;

impl TargetReticle {
    const Z_INDEX: f32 = 1.;
}

pub(super) fn draw(
    mut commands: Commands,
    level_query: Query<&Level>,
    player_query: Query<&Target, With<Player>>,
    reticle_query: Query<Entity, With<TargetReticle>>,
    positions: Query<&Position>,
    map_assets: Res<MapAssets>,
) {
    let level = level_query.single();
    let target = player_query.single();

    let mut reticle_commands = match reticle_query.get_single() {
        Ok(reticle_id) => commands.entity(reticle_id),

        Err(NoEntities(_)) => commands.spawn((
            TargetReticle,
            ColorMesh2dBundle {
                mesh: map_assets.hexagon.clone(),
                material: map_assets.target_reticle_color.clone(),
                transform: Transform {
                    translation: Vec3::Z * TargetReticle::Z_INDEX,
                    ..default()
                },
                ..default()
            },
        )),

        Err(MultipleEntities(_)) => unreachable!(),
    };

    match target
        .id()
        .and_then(|target_id| positions.get(target_id).ok())
        .and_then(|pos| level.get(pos))
    {
        Some(&tile_id) => {
            reticle_commands.set_parent(tile_id);
        }

        None => reticle_commands.despawn(),
    }
}
