use glam::{I64Vec3, Vec3, Vec4};

use crate::{
    util::{voxel_to_vec, RED},
    Voxel,
};

type Color = Vec4;

pub struct Flower {
    pub voxels: Vec<Voxel>,
    pub debug_points: Vec<(Vec3, Color)>,
}

pub fn flower(_seed: u64) -> Flower {
    let root = I64Vec3::new(0, 0, 0);
    let debug_points = vec![(voxel_to_vec(&root), RED)];

    Flower {
        voxels: Vec::new(),
        debug_points,
    }
}
