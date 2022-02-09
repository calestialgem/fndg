use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use hex2d::{Coordinate, Direction, Position, Spacing};

pub(super) struct Lines {
    spacing: Spacing,
    points: HashSet<Point>,
    lines: HashSet<Line>,
}

impl Lines {
    pub(super) fn new(spacing: Spacing) -> Self {
        Self {
            spacing,
            points: HashSet::with_capacity(0),
            lines: HashSet::with_capacity(0),
        }
    }

    fn add(&mut self, coord: Coordinate) {}

    fn find_positions(&self, coord: Coordinate) -> Vec<Position> {
        let mut positions = Vec::new();
        for dir in Direction::all() {
            positions.push(Position { coord, dir: *dir })
        }
        positions
    }
}

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn to_fixed(&self) -> (Fixed, Fixed) {
        (Fixed::new(self.x), Fixed::new(self.y))
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_fixed().hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.to_fixed() == other.to_fixed()
    }
}

impl Eq for Point {}

#[derive(Hash, PartialEq, Eq)]
struct Fixed {
    i: i64,
}

impl Fixed {
    const SCALE: f32 = 1000.0;

    fn new(n: f32) -> Self {
        let n = n * Self::SCALE;
        Self { i: n as i64 }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Line {
    a: usize,
    b: usize,
}
