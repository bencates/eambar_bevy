use {
    super::{Map, Position},
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
pub struct ViewshedHighlight;

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            fov: HashSet::new(),
            range,
        }
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

pub(crate) fn calculate_field_of_view(
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

pub(crate) fn highlight_player_viewshed(
    mut commands: Commands,
    player_query: Query<&Viewshed, (With<Player>, Changed<Viewshed>)>,
    highlight_query: Query<Entity, With<ViewshedHighlight>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    hexagon: Res<HexagonMesh>,
) {
    if let Ok(viewshed) = player_query.get_single() {
        let highlight_color = materials.add(ColorMaterial::from(Color::YELLOW_GREEN.with_a(0.1)));

        for highlight in &highlight_query {
            commands.entity(highlight).despawn_recursive()
        }

        commands
            .spawn((ViewshedHighlight, SpatialBundle::default()))
            .with_children(|parent| {
                for &coord in viewshed.fov.iter() {
                    parent.spawn(ColorMesh2dBundle {
                        mesh: hexagon.clone().into(),
                        material: highlight_color.clone(),
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
