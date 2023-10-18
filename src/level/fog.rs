use crate::prelude::*;

#[derive(Component)]
pub struct Fog;

impl Fog {
    const Z_INDEX: f32 = 1.;

    pub fn bundle(assets: &MapAssets) -> impl Bundle {
        (
            Self,
            ColorMesh2dBundle {
                mesh: assets.hexagon.clone(),
                material: assets.fog_color.clone(),
                transform: Transform {
                    translation: Vec3::Z * Self::Z_INDEX,
                    ..default()
                },
                ..default()
            },
        )
    }
}

pub fn show_outside_player_viewshed(
    mut query: Query<(&Position, &mut Visibility), With<Fog>>,
    player_viewshed: Query<&Viewshed, With<Player>>,
) {
    let viewshed = player_viewshed.single();

    for (pos, mut vis) in &mut query {
        if viewshed.includes(pos) {
            *vis = Visibility::Hidden;
        } else {
            *vis = Visibility::Inherited;
        }
    }
}
