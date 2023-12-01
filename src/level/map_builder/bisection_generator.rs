use super::{is_blocked, Tiles, ORIGIN};
use crate::prelude::MapTile;
use hex2d::{Coordinate, Direction, Direction::*, Spin};
use rand::prelude::Rng;
use std::iter::zip;

const MIN_ROOM_SIZE: i32 = 4;

pub(super) fn run(tiles: &mut Tiles, radius: i32, rng: &mut impl Rng) {
    let two_edges = radius as usize * 2 + 1;

    let x_chords = zip(
        ORIGIN.ring_iter(radius, Spin::CW(YX)),
        ORIGIN.ring_iter(radius, Spin::CCW(ZX)),
    )
    .take(two_edges);

    let y_chords = zip(
        ORIGIN.ring_iter(radius, Spin::CW(ZY)),
        ORIGIN.ring_iter(radius, Spin::CCW(XY)),
    )
    .take(two_edges);

    let z_chords = zip(
        ORIGIN.ring_iter(radius, Spin::CW(XZ)),
        ORIGIN.ring_iter(radius, Spin::CCW(YZ)),
    )
    .take(two_edges);

    let mut doors: Vec<Coordinate> = Vec::new();
    let mut done = (false, false, false);

    while !(done.0 && done.1 && done.2) {
        if !done.0 {
            let longest_x_path = longest_path(tiles, x_chords.clone());
            match bisect_path(tiles, longest_x_path, &[ZX, XY], rng) {
                Some(c) => doors.push(c),
                None => done.0 = true,
            };
        }

        if !done.1 {
            let longest_y_path = longest_path(tiles, y_chords.clone());
            match bisect_path(tiles, longest_y_path, &[ZY, XY], rng) {
                Some(c) => doors.push(c),
                None => done.1 = true,
            };
        }

        if !done.2 {
            let longest_z_path = longest_path(tiles, z_chords.clone());
            match bisect_path(tiles, longest_z_path, &[ZY, ZX], rng) {
                Some(c) => doors.push(c),
                None => done.2 = true,
            };
        }
    }

    tiles.extend(doors.into_iter().map(|c| (c, MapTile::Floor)));
}

fn longest_path(
    tiles: &Tiles,
    lines: impl Iterator<Item = (Coordinate, Coordinate)>,
) -> (Coordinate, Coordinate) {
    let mut paths: Vec<(Coordinate, Coordinate)> = Vec::new();

    for (c1, c2) in lines {
        let mut path_start: Option<Coordinate> = None;
        let mut line = c1.line_to_iter(c2).peekable();

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

            tiles.insert(center, MapTile::Wall);

            let mut coord = center - dir;
            while !is_blocked(tiles, &coord) {
                tiles.insert(coord, MapTile::Wall);
                coord = coord - dir;
            }

            coord = center + dir;
            while !is_blocked(tiles, &coord) {
                tiles.insert(coord, MapTile::Wall);
                coord = coord + dir;
            }

            return Some(center);
        }
    }

    None
}
