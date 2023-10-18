use super::{Level, MapTile};
use bevy::prelude::*;
use hex2d::{Coordinate, Direction, Spacing, Spin};
use std::{fmt, ops::Add};

const TILE_SPACING: Spacing = Spacing::FlatTop(super::TILE_RADIUS);

#[derive(Clone, Copy, Component, Debug, Hash, Eq, PartialEq)]
pub struct Position(Coordinate);

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
    pub fn ring_iter(&self, radius: i32) -> impl Iterator<Item = Position> + '_ {
        self.0
            .ring_iter(radius, Spin::CW(Direction::ZY))
            .map(Into::into)
    }

    pub fn line_to_with_edge_detection_iter(
        &self,
        dest: &Self,
    ) -> impl Iterator<Item = (Position, Position)> + '_ {
        self.0
            .line_to_with_edge_detection_iter(dest.0)
            .map(|(c1, c2)| (c1.into(), c2.into()))
    }

    pub(super) fn to_pixel(self) -> (f32, f32) {
        self.0.to_pixel(TILE_SPACING)
    }

    pub fn distance(&self, other: &Position) -> i32 {
        self.0.distance(other.0)
    }

    pub fn neighbors(&self) -> [Position; 6] {
        self.0.neighbors().map(Into::into)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0.x, self.0.y, self.0.z())
    }
}

impl fmt::Display for CompassDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            CompassDirection::North => "north",
            CompassDirection::NorthEast => "north east",
            CompassDirection::SouthEast => "south east",
            CompassDirection::South => "south",
            CompassDirection::SouthWest => "south west",
            CompassDirection::NorthWest => "north west",
        })
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
        Self(self.0 + Direction::from(dir))
    }
}

impl From<Coordinate> for Position {
    fn from(coord: Coordinate) -> Self {
        Self(coord)
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self(Coordinate::new(x, y))
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn move_to_location(
    mut commands: Commands,
    level_query: Query<&Level>,
    mut query: Query<(Entity, &Position), (Changed<Position>, Without<MapTile>)>,
) {
    let level = level_query.single();

    for (entity, pos) in &mut query {
        if let Some(&map_tile_id) = level.get(pos) {
            commands.entity(entity).set_parent(map_tile_id);
        }
    }
}
