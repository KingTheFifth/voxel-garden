#![allow(unused)]

use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use crate::Point;

static SPHERES: OnceLock<Mutex<HashMap<u32, Vec<Point>>>> = OnceLock::new();

struct BresenhamPoints {
    start: Point,
    end: Point,

    ey: i32,
    ez: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl BresenhamPoints {
    fn new(start: Point, end: Point) -> Self {
        let (x1, y1, z1) = (start.x, start.y, start.z);
        let (x2, y2, z2) = (end.x, end.y, end.z);

        let my = 2 * (y2 - y1);
        let mz = 2 * (z2 - z1);

        Self {
            start,
            end,
            ey: my - (x2 - x1),
            ez: mz - (x2 - x1),
            x: x1,
            y: y1,
            z: z1,
        }
    }
}

impl Iterator for BresenhamPoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x <= self.end.x {
            let vox = Point::new(self.x, self.y, self.z);
            self.x += 1;
            self.ey += 2 * (self.end.y - self.start.y);
            self.ez += 2 * (self.end.z - self.start.z);
            if self.ey >= 0 {
                self.y += 1;
                self.ey -= 2 * (self.end.x - self.start.x);
            }
            if self.ez >= 0 {
                self.z += 1;
                self.ez -= 2 * (self.end.x - self.start.x);
            }
            Some(vox)
        } else {
            None
        }
    }
}

/// Bresenham's line drawing algorithm extended into 3D.
pub fn bresenham(start: Point, end: Point) -> Vec<Point> {
    BresenhamPoints::new(start, end).collect()
}

/// A line with 3x3x3-crosses at every point.
pub fn line_cross(start: Point, end: Point) -> Vec<Point> {
    BresenhamPoints::new(start, end)
        .flat_map(|Point { x, y, z }| {
            [
                Point::new(x, y, z),
                Point::new(x + 1, y, z),
                Point::new(x - 1, y, z),
                Point::new(x, y + 1, z),
                Point::new(x, y - 1, z),
                Point::new(x, y, z + 1),
                Point::new(x, y, z - 1),
            ]
        })
        .collect()
}

pub fn circle(midpoint: Point, r: f32) -> Vec<Point> {
    let mut points = vec![];
    let Point {
        x: px,
        y: py,
        z: pz,
    } = midpoint;
    let bound = r.ceil() as i32;
    for xi in -bound..=bound {
        let x = xi as f32;
        for zi in -bound..=bound {
            let z = zi as f32;
            if x.powi(2) + z.powi(2) < r.powi(2) {
                points.push(Point {
                    x: px + xi,
                    y: py,
                    z: pz + zi,
                });
            }
        }
    }

    points
}

pub fn sphere(midpoint: Point, r: f32) -> Vec<Point> {
    let mut spheres = SPHERES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    if let Some(sphere) = spheres.get(&r.to_bits()) {
        return sphere.clone();
    }
    let mut points = vec![];
    let Point {
        x: px,
        y: py,
        z: pz,
    } = midpoint;
    let bound = r.ceil() as i32;
    for xi in -bound..=bound {
        let x = xi as f32;
        for yi in -bound..=bound {
            let y = yi as f32;
            for zi in -bound..=bound {
                let z = zi as f32;
                if x.powi(2) + y.powi(2) + z.powi(2) < r.powi(2) {
                    points.push(Point {
                        x: px + xi,
                        y: py + yi,
                        z: pz + zi,
                    });
                }
            }
        }
    }

    // remove voxels fully neighboured
    let orig = points.clone();
    points.retain(|p| {
        let has_all_neighbours = [
            Point::new(1, 0, 0),
            Point::new(-1, 0, 0),
            Point::new(0, 1, 0),
            Point::new(0, -1, 0),
            Point::new(0, 0, 0),
            Point::new(0, 0, 1),
            Point::new(0, 0, -1),
        ]
        .into_iter()
        .all(|d| orig.contains(&(*p + d)));
        !has_all_neighbours
    });

    spheres.insert(r.to_bits(), points.clone());

    points
}
