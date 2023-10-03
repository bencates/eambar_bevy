use bevy::{prelude::*, sprite::Anchor};

// const TERM_WIDTH: i32 = 80;
// const TERM_HEIGHT: i32 = 50;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Eambar".to_string(),
                        resolution: (80. * 16., 50. * 16.).into(),
                        ..default()
                    }),
                    ..default()
                })
                // don't alias pixel art
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (move_left, player_input))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    let atlas = assets.add(TextureAtlas::from_grid(
        asset_server.load("terminal8x8.png"),
        (8., 8.).into(),
        16,
        16,
        None,
        None,
    ));

    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
        Player,
        SpriteSheetBundle {
            texture_atlas: atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 64,
                color: Color::YELLOW,
                anchor: Anchor::TopLeft,
                ..default()
            },
            transform: pos(40, 25),
            ..default()
        },
    ));

    // Smilies
    for i in 0..10 {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    color: Color::RED,
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                transform: pos(i * 7, 20),
                ..default()
            },
            LeftMover,
        ));
    }
}

fn pos(x: i32, y: i32) -> Transform {
    let x = (x - 40) as f32 * 16.;
    let y = -(y - 25) as f32 * 16.;

    Transform {
        translation: (x, y, 0.).into(),
        scale: Vec3::splat(2.),
        ..default()
    }
}

#[derive(Component)]
struct LeftMover;

fn move_left(mut query: Query<&mut Transform, With<LeftMover>>) {
    for mut transform in &mut query {
        transform.translation.x -= 16.;

        if transform.translation.x < 16. * -40. {
            transform.translation.x = 16. * 39.;
        }
    }
}

#[derive(Component)]
struct Player;

fn player_input(mut query: Query<&mut Transform, With<Player>>, keys: Res<Input<KeyCode>>) {
    for mut transform in &mut query {
        if keys.just_pressed(KeyCode::Up) {
            try_move(0, -1, &mut transform);
        }
        if keys.just_pressed(KeyCode::Right) {
            try_move(1, 0, &mut transform);
        }
        if keys.just_pressed(KeyCode::Down) {
            try_move(0, 1, &mut transform);
        }
        if keys.just_pressed(KeyCode::Left) {
            try_move(-1, 0, &mut transform);
        }
    }
}

fn try_move(delta_x: i32, delta_y: i32, transform: &mut Transform) {
    let delta_x = delta_x as f32 * 16.;
    let delta_y = -delta_y as f32 * 16.;

    transform.translation.x = (transform.translation.x + delta_x).clamp(-40. * 16., 39. * 16.);
    transform.translation.y = (transform.translation.y + delta_y).clamp(-24. * 16., 25. * 16.);
}
