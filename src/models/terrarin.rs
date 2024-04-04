use crate::Voxel;
use noise::{NoiseFn, Perlin};

pub fn generate_flat_terrain(
    pos: i64,
    width: i64,
    height: i64,
    depth: i64,
    max_height: f64,
    perlin: Perlin,
) -> Vec<Voxel> {
    let mut voxels = Vec::new();

    for z in 0..depth {
        for y in 0..height {
            for x in 0..width {
                if (y as f64 / max_height) < perlin.get([x as f64 / 100., z as f64 / 100.]) {
                    voxels.push(Voxel::new(pos + x, pos + y, pos + z));
                }
            }
        }
    }

    return voxels;
}
