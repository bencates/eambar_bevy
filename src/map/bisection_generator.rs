use super::Tile;
use bevy::utils::HashMap;
use hex2d::{Coordinate, Direction, Direction::*, LineTo, Spin};
use rand::Rng;

type Tiles = HashMap<Coordinate, Tile>;

const ORIGIN: Coordinate = Coordinate { x: 0, y: 0 };
const MIN_ROOM_SIZE: i32 = 4;

pub fn build(radius: i32, rng: &mut impl Rng) -> Tiles {
    let radius_cw = ORIGIN.ring_iter(radius, Spin::CW(ZX));
    let radius_ccw = ORIGIN.ring_iter(radius, Spin::CCW(ZX));

    // Start with a ring of walls
    let mut tiles: Tiles = radius_cw.map(|c| (c, Tile::Wall)).collect();

    let x_lines = radius_cw
        .filter(|coord| ORIGIN.distance(*coord + YZ) < radius)
        .zip(radius_ccw.filter(|coord| ORIGIN.distance(*coord + ZY) < radius))
        .map(|(c1, c2)| c1.line_to_iter(c2));

    // Move where we calculate the radius from so that both arcs that y_lines
    // needs are contiguous.
    let radius_cw = ORIGIN.ring_iter(radius, Spin::CW(ZY));
    let radius_ccw = ORIGIN.ring_iter(radius, Spin::CCW(ZY));

    let y_lines = radius_cw
        .filter(|coord| ORIGIN.distance(*coord + XZ) < radius)
        .zip(radius_ccw.filter(|coord| ORIGIN.distance(*coord + ZX) < radius))
        .map(|(c1, c2)| c1.line_to_iter(c2));

    let z_lines = radius_cw
        .filter(|coord| ORIGIN.distance(*coord + YX) < radius)
        .zip(radius_ccw.filter(|coord| ORIGIN.distance(*coord + XY) < radius))
        .map(|(c1, c2)| c1.line_to_iter(c2));

    let mut doors: Vec<Coordinate> = Vec::new();
    let mut done = (false, false, false);

    while !(done.0 && done.1 && done.2) {
        if !done.0 {
            let longest_x_path = longest_path(&tiles, x_lines.clone());
            match bisect_path(&mut tiles, longest_x_path, &[ZX, XY], rng) {
                Some(c) => doors.push(c),
                None => done.0 = true,
            };
        }

        if !done.1 {
            let longest_y_path = longest_path(&tiles, y_lines.clone());
            match bisect_path(&mut tiles, longest_y_path, &[ZY, XY], rng) {
                Some(c) => doors.push(c),
                None => done.1 = true,
            };
        }

        if !done.2 {
            let longest_z_path = longest_path(&tiles, z_lines.clone());
            match bisect_path(&mut tiles, longest_z_path, &[ZY, ZX], rng) {
                Some(c) => doors.push(c),
                None => done.2 = true,
            };
        }
    }

    tiles.extend(doors.into_iter().map(|c| (c, Tile::Floor)));

    tiles
}

fn is_blocked(tiles: &Tiles, coord: &Coordinate) -> bool {
    tiles.get(coord) == Some(&Tile::Wall)
}

fn longest_path(
    tiles: &Tiles,
    lines: impl Iterator<Item = LineTo<i32>>,
) -> (Coordinate, Coordinate) {
    let mut paths: Vec<(Coordinate, Coordinate)> = Vec::new();

    for line in lines {
        let mut path_start: Option<Coordinate> = None;
        let mut line = line.peekable();

        while let Some(coord) = line.next() {
            if path_start.is_none() && !is_blocked(tiles, &coord) {
                path_start = Some(coord);
            }

            if line
                .peek()
                .is_some_and(|next_coord| is_blocked(tiles, next_coord))
            {
                if let Some(start) = path_start.take() {
                    paths.push((start, coord));
                }
            }
        }
    }

    paths
        .into_iter()
        .max_by_key(|(c1, c2)| c1.distance(*c2))
        .unwrap_or((ORIGIN, ORIGIN))
}

fn bisect_path(
    tiles: &mut Tiles,
    (c1, c2): (Coordinate, Coordinate),
    axes: &[Direction],
    rng: &mut impl Rng,
) -> Option<Coordinate> {
    let distance = c1.distance(c2);

    if distance > MIN_ROOM_SIZE * 2 {
        let offset = rng.gen_range(MIN_ROOM_SIZE..(distance - MIN_ROOM_SIZE));

        if let Some(center) = c1.line_to_iter(c2).nth(offset as usize) {
            let dir = axes[rng.gen_range(0..axes.len())];

            tiles.insert(center, Tile::Wall);

            let mut coord = center - dir;
            while !is_blocked(tiles, &coord) {
                tiles.insert(coord, Tile::Wall);
                coord = coord - dir;
            }

            coord = center + dir;
            while !is_blocked(tiles, &coord) {
                tiles.insert(coord, Tile::Wall);
                coord = coord + dir;
            }

            return Some(center);
        }
    }

    None
}
