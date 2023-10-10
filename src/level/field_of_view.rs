use {
    super::{Level, LocationBundle, Map, Position},
    crate::{assets::HexagonMesh, player::Player},
    bevy::{prelude::*, utils::HashSet},
    std::ops::Index,
};

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
    map: Res<Map>,
) {
    for (pos, mut viewshed) in &mut query {
        let center = pos;

        viewshed.fov.clear();

        for edge in center.ring_iter(viewshed.range) {
            for (p1, p2) in center.line_to_with_edge_detection_iter(&edge) {
                viewshed.fov.insert(p1);
                viewshed.fov.insert(p2);

                if map[&p1].is_opaque() && map[&p2].is_opaque() {
                    break;
                }
            }
        }
    }
}

pub fn update_map_visibility(
    player_query: Query<&Viewshed, (With<Player>, Changed<Viewshed>)>,
    mut map: ResMut<Map>,
) {
    if let Ok(viewshed) = player_query.get_single() {
        map.reveal(viewshed.fov.iter().copied());
    }
}

pub fn draw_fog_outside_player_viewshed(
    mut commands: Commands,
    player_query: Query<&Viewshed, (With<Player>, Changed<Viewshed>)>,
    level_query: Query<Entity, With<Level>>,
    highlight_query: Query<Entity, With<Fog>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    hexagon: Res<HexagonMesh>,
    map: Res<Map>,
) {
    if let Ok(viewshed) = player_query.get_single() {
        let level = level_query.single();
        let fog_color = materials.add(ColorMaterial::from(Color::BLACK.with_a(0.4)));

        for highlight in &highlight_query {
            commands.entity(level).remove_children(&[highlight]);
            commands.entity(highlight).despawn_recursive();
        }

        commands
            .spawn((Fog, SpatialBundle::default()))
            .set_parent(level)
            .with_children(|parent| {
                for &position in map.revealed().difference(&viewshed.fov) {
                    parent.spawn((
                        LocationBundle {
                            position,
                            z_index: 1.into(),
                        },
                        ColorMesh2dBundle {
                            mesh: hexagon.clone().into(),
                            material: fog_color.clone(),
                            transform: Transform {
                                rotation: HexagonMesh::ROTATION,
                                ..default()
                            },
                            ..default()
                        },
                    ));
                }
            });
    }
}
