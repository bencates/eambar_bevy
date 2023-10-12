use crate::prelude::*;
use std::ops::Index;

#[derive(Component)]
pub struct Viewshed {
    fov: HashSet<Position>,
    range: i32,
}

#[derive(Component)]
pub struct Fog;

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            fov: HashSet::new(),
            range,
        }
    }

    pub fn includes(&self, pos: &Position) -> bool {
        self.fov.get(pos).is_some()
    }
}

impl Index<&Position> for Viewshed {
    type Output = bool;

    fn index(&self, pos: &Position) -> &bool {
        match self.fov.contains(pos) {
            true => &true,
            false => &false,
        }
    }
}

pub fn calculate_field_of_view(
    mut query: Query<(&Position, &mut Viewshed), Changed<Position>>,
    tiles_query: Query<(&Position, &MapTile)>,
) {
    let tiles: HashMap<_, _> = tiles_query.iter().collect();

    for (pos, mut viewshed) in &mut query {
        let center = pos;

        viewshed.fov.clear();

        for edge in center.ring_iter(viewshed.range) {
            for (p1, p2) in center.line_to_with_edge_detection_iter(&edge) {
                viewshed.fov.insert(p1);
                viewshed.fov.insert(p2);

                if tiles[&p1].is_opaque() && tiles[&p2].is_opaque() {
                    break;
                }
            }
        }
    }
}

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

        let revealed: HashSet<_> = tile_query
            .iter()
            .filter(|(_, vis)| vis.is_visible())
            .map(|(&pos, _)| pos)
            .collect();

        commands
            .spawn((Fog, SpatialBundle::default()))
            .set_parent(level)
            .with_children(|parent| {
                for &position in revealed.difference(&viewshed.fov) {
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
