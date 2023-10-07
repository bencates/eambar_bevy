use {
    super::{Level, Map, Position},
    crate::{assets::HexagonMesh, player::Player},
    bevy::{prelude::*, utils::HashSet},
    hex2d::{Coordinate, Spin, XY},
    std::ops::Index,
};

#[derive(Component)]
pub struct Viewshed {
    fov: HashSet<Coordinate>,
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

    pub fn includes(&self, coord: &Coordinate) -> bool {
        self.fov.get(coord).is_some()
    }
}

impl Index<&Position> for Viewshed {
    type Output = bool;

    fn index(&self, pos: &Position) -> &bool {
        match self.fov.contains(pos.as_ref()) {
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
        let center: &Coordinate = pos.as_ref();

        viewshed.fov.clear();

        for edge in center.ring_iter(viewshed.range, Spin::CW(XY)) {
            for (c1, c2) in center.line_to_with_edge_detection_iter(edge) {
                viewshed.fov.insert(c1);
                viewshed.fov.insert(c2);

                if map[&c1].is_opaque() && map[&c2].is_opaque() {
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
                for coord in map.revealed().difference(&viewshed.fov) {
                    parent.spawn(ColorMesh2dBundle {
                        mesh: hexagon.clone().into(),
                        material: fog_color.clone(),
                        transform: Transform {
                            translation: Position::new(coord.x, coord.y, 1).into(),
                            rotation: HexagonMesh::ROTATION,
                            ..default()
                        },
                        ..default()
                    });
                }
            });
    }
}
