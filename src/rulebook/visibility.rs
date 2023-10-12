use crate::prelude::*;

#[derive(Component)]
pub struct Viewshed {
    fov: HashSet<Position>,
    range: i32,
}

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

pub(super) fn calculate_field_of_view(
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

pub(super) fn show_in_player_field_of_view<T: Component>(
    mut query: Query<(&Position, &mut Visibility), With<T>>,
    player_viewshed: Query<&Viewshed, With<Player>>,
) {
    let viewshed = player_viewshed.single();

    for (pos, mut vis) in &mut query {
        if viewshed.includes(pos) {
            *vis = Visibility::Visible;
        } else {
            *vis = Visibility::Hidden;
        }
    }
}
