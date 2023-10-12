use crate::prelude::*;

#[derive(Component)]
pub struct Fog;

pub fn draw_fog_outside_player_viewshed(
    mut commands: Commands,
    player_query: Query<&Viewshed, (With<Player>, Changed<Viewshed>)>,
    level_query: Query<Entity, With<Level>>,
    tile_query: Query<(&Position, &ComputedVisibility), With<MapTile>>,
    highlight_query: Query<Entity, With<Fog>>,
    assets: Res<MapAssets>,
) {
    if let Ok(viewshed) = player_query.get_single() {
        let level = level_query.single();

        for highlight in &highlight_query {
            commands.entity(level).remove_children(&[highlight]);
            commands.entity(highlight).despawn_recursive();
        }

        commands
            .spawn((Fog, SpatialBundle::default()))
            .set_parent(level)
            .with_children(|parent| {
                let revealed_but_not_visible = tile_query
                    .iter()
                    .filter(|(pos, vis)| vis.is_visible() && !viewshed.includes(pos));

                for (&position, _) in revealed_but_not_visible {
                    parent.spawn((
                        LocationBundle {
                            position,
                            z_index: 1.into(),
                        },
                        ColorMesh2dBundle {
                            mesh: assets.hexagon.clone(),
                            material: assets.fog_color.clone(),
                            transform: Transform {
                                rotation: MapAssets::HEX_ROTATION,
                                ..default()
                            },
                            ..default()
                        },
                    ));
                }
            });
    }
}
