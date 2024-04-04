use glam::{I64Vec3, Vec4};

use crate::{util::RED, Voxel};

type Color = Vec4;

pub struct Flower {
    pub voxels: Vec<Voxel>,
    pub debug_points: Vec<(I64Vec3, Color)>,
}

pub fn flower(_seed: u64) -> Flower {
    let root = I64Vec3::new(0, 0, 0);
    let debug_points = vec![(root, RED)];

    Flower {
        voxels: Vec::new(),
        debug_points,
    }
}
