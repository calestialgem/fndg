use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::Add,
};

use bevy::{
    prelude::Mesh,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};
use hex2d::{Coordinate, Spacing};

pub(super) struct Lines {
    spacing: Spacing,
    verticies: Vec<[f32; 2]>,
    points: HashMap<Point, u16>,
    lines: HashSet<Line>,
    offsets: [Point; 6],
}

impl Lines {
    pub(super) fn new(spacing: Spacing) -> Self {
        Self {
            spacing,
            verticies: Vec::with_capacity(0),
            points: HashMap::with_capacity(0),
            lines: HashSet::with_capacity(0),
            offsets: [
                Point::with_angle(30.0 + 0.0 * 60.0, 0.5),
                Point::with_angle(30.0 + 1.0 * 60.0, 0.5),
                Point::with_angle(30.0 + 2.0 * 60.0, 0.5),
                Point::with_angle(30.0 + 3.0 * 60.0, 0.5),
                Point::with_angle(30.0 + 4.0 * 60.0, 0.5),
                Point::with_angle(30.0 + 5.0 * 60.0, 0.5),
            ],
        }
    }

    pub(super) fn add(&mut self, coord: Coordinate) {
        let corners = self.find_corners(coord);
        let mut indicies = Vec::with_capacity(corners.len());
        for corner in corners {
            indicies.push(self.point_index(corner));
        }
        for i in 0..indicies.len() {
            self.lines.insert(Line {
                a: indicies[i],
                b: indicies[(i + 1) % indicies.len()],
            });
        }
    }

    pub(super) fn into_mesh(self) -> Mesh {
        println!("Corners: {}/24", self.points.len());
        println!("Lines:   {}/30", self.lines.len());
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.set_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x2(self.verticies),
        );
        let mut indicies = Vec::with_capacity(self.lines.len() * 2);
        for line in self.lines {
            indicies.push(line.a);
            indicies.push(line.b);
        }
        mesh.set_indices(Some(Indices::U16(indicies)));
        mesh
    }

    fn point_index(&mut self, point: Point) -> u16 {
        let index = self.points.get(&point);
        if let Some(index) = index {
            return *index;
        }
        let index = self.verticies.len() as u16;
        self.verticies.push([point.x, point.y]);
        self.points.insert(point, index);
        index
    }

    fn find_corners(&self, coord: Coordinate) -> Vec<Point> {
        let center = Point::with_center(coord, self.spacing);
        let mut corners = Vec::with_capacity(self.offsets.len());
        for offset in self.offsets {
            corners.push(center + offset);
        }
        corners
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn with_center(coord: Coordinate, spacing: Spacing) -> Self {
        let point = coord.to_pixel(spacing);
        Point {
            x: point.0,
            y: point.1,
        }
    }

    fn with_angle(angle: f32, side: f32) -> Self {
        let angle = angle.to_radians();
        Point {
            x: angle.cos() * side,
            y: angle.sin() * side,
        }
    }

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

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Fixed {
    i: i32,
}

impl Fixed {
    const SCALE: f32 = 1000.0;

    fn new(n: f32) -> Self {
        let n = n * Self::SCALE;
        Self { i: n as i32 }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Line {
    a: u16,
    b: u16,
}
