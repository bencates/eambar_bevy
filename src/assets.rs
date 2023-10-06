use bevy::sprite::Mesh2dHandle;

use {
    crate::level::TILE_RADIUS,
    bevy::prelude::{shape::RegularPolygon, *},
};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextSprite>()
            .init_resource::<HexagonMesh>();
    }
}

#[derive(Clone, Resource)]
pub struct TextSprite(Handle<TextureAtlas>);

impl FromWorld for TextSprite {
    fn from_world(world: &mut World) -> Self {
        let texture = world.resource::<AssetServer>().load("terminal8x8.png");

        let texture_atlas = world
            .resource_mut::<Assets<_>>()
            .add(TextureAtlas::from_grid(
                texture,
                (8., 8.).into(),
                16,
                16,
                None,
                None,
            ));

        Self(texture_atlas)
    }
}

impl TextSprite {
    pub fn char_index(c: char) -> usize {
        match c {
            '#' => 35,
            '.' => 46,
            '@' => 64,
            _ => unimplemented!(),
        }
    }
}

impl From<TextSprite> for Handle<TextureAtlas> {
    fn from(sprites: TextSprite) -> Handle<TextureAtlas> {
        sprites.0
    }
}

#[derive(Clone, Resource)]
pub struct HexagonMesh(Handle<Mesh>);

impl FromWorld for HexagonMesh {
    fn from_world(world: &mut World) -> Self {
        let hexagon = world
            .resource_mut::<Assets<_>>()
            .add(RegularPolygon::new(TILE_RADIUS, 6).into());

        Self(hexagon)
    }
}

impl HexagonMesh {
    /// 30° around the z axis, i.e. from pointy-top to flat-top
    pub const ROTATION: Quat = Quat::from_xyzw(0.0, 0.0, 0.25881904, 0.9659258);
}

impl From<HexagonMesh> for Mesh2dHandle {
    fn from(hex: HexagonMesh) -> Mesh2dHandle {
        hex.0.into()
    }
}
