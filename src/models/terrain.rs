use glam::{IVec3, Vec4};

use crate::Voxel;

pub fn generate_flat_terrain(pos_x: i32, pos_z: i32, width: i32, depth: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();

    for z in 0..depth {
        for x in 0..width {
            let position = IVec3::new(pos_x + x, 0, pos_z + z);
            let color = Vec4::new(0.1, 0.5, 0.2, 1.0);
            voxels.push(Voxel::new(position, color));
        }
    }

    return voxels;
}
