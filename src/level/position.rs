use {
    bevy::prelude::{Component, Vec3},
    hex2d::{Coordinate, Direction, Spacing},
    std::ops::Add,
};

const TILE_SPACING: Spacing = Spacing::FlatTop(super::TILE_RADIUS);

#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub struct Position(pub(super) Coordinate, f32);

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Coordinate::new(x, y), z as f32)
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, dir: Direction) -> Self::Output {
        Self(self.0 + dir, self.1)
    }
}

impl AsRef<Coordinate> for Position {
    fn as_ref(&self) -> &Coordinate {
        &self.0
    }
}

impl From<Position> for Vec3 {
    fn from(Position(coord, z): Position) -> Self {
        let (x, y) = coord.to_pixel(TILE_SPACING);

        Vec3::new(x, y, z)
    }
}
