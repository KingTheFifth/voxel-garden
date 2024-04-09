use glam::Vec4;

use crate::{Color, Voxel};

pub fn flower(_seed: u64) -> Vec<(Voxel, Color)> {
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
