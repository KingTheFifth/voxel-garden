use crate::Voxel;
use glam::{Vec3, Vec4};
use noise::{NoiseFn, Perlin};

pub fn generate_flat_terrain(pos_x: i64, pos_z: i64, width: i64, depth: i64) -> Vec<Voxel> {
    let mut voxels = Vec::new();

    for z in 0..depth {
        for x in 0..width {
            let position = Vec3::new((pos_x + x) as f32, 0., (pos_z + z) as f32);
            let color = Vec4::new(0.1, 0.5, 0.2, 1.0);
            voxels.push(Voxel::new(position, color));
        }
    }
    return voxels;
}

pub fn generate_terrain(
    pos_x: i64,
    pos_z: i64,
    width: i64,
    height: i64,
    depth: i64,
    sample_rate: f32,
    max_height: f32,
    perlin: Perlin,
) -> Vec<Voxel> {
    let mut voxels = Vec::new();

    for z in pos_z..depth {
        for y in 0..height {
            for x in pos_x..width {
                let sample_x: f32 = x as f32 * sample_rate;
                let sample_z: f32 = z as f32 * sample_rate;
                let sample: f32 = (perlin.get([sample_x as f64, sample_z as f64]) as f32 + 1.) / 2.;
                let height = sample * max_height;

                if y <= height as i64 {
                    let color = Vec4::new(0.1, 0.5, 0.2, 1.0);
                    let position = Vec3::new((pos_x + x) as f32, height as f32, (pos_z + z) as f32);
                    voxels.push(Voxel::new(position, color));
                }
            }
        }
    }

    return voxels;
}
