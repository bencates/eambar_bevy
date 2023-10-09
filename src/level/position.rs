use {
    bevy::prelude::{Component, Vec3},
    hex2d::{Coordinate, Direction, Spacing},
    std::ops::Add,
};

const TILE_SPACING: Spacing = Spacing::FlatTop(super::TILE_RADIUS);

#[derive(Clone, Copy, Component, Debug)]
pub struct Position(pub(super) Coordinate, f32);

#[derive(Clone, Copy, Debug)]
pub enum CompassDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Coordinate::new(x, y), z as f32)
    }
}

impl From<CompassDirection> for Direction {
    fn from(value: CompassDirection) -> Self {
        match value {
            CompassDirection::North => Direction::ZY,
            CompassDirection::NorthEast => Direction::XY,
            CompassDirection::SouthEast => Direction::XZ,
            CompassDirection::South => Direction::YZ,
            CompassDirection::SouthWest => Direction::YX,
            CompassDirection::NorthWest => Direction::ZX,
        }
    }
}

impl Add<CompassDirection> for Position {
    type Output = Position;

    fn add(self, dir: CompassDirection) -> Self::Output {
        Self(self.0 + Direction::from(dir), self.1)
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

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
