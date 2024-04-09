use glam::Vec4;

use crate::{
    util::{voxel_to_vec, RED},
    Voxel,
};

type Color = Vec4;

pub fn flower(_seed: u64) -> Vec<(Voxel, Vec4)> {
    let root = Voxel::new(0, 0, 0);
    let _debug_points = vec![(voxel_to_vec(&root), RED)];

    let brown = Vec4::new(1.0, 0.5, 0.0, 1.0);

    vec![
        // stem
        (Voxel::new(0, 0, 0), brown),
        (Voxel::new(0, 1, 0), brown),
        (Voxel::new(0, 2, 0), brown),
        (Voxel::new(0, 3, 0), brown),
        (Voxel::new(1, 4, 0), brown),
    ]
}
