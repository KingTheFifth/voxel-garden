use crate::Voxel;

pub fn generate_flat_terrain(pos: i64, width: i64, depth: i64) -> Vec<Voxel> {
    let mut voxels = Vec::new();

    for z in 0..depth {
        for x in 0..width {
            voxels.push(Voxel::new(pos + x, 0, pos + z));
        }
    }

    return voxels;
}
