use crate::InstanceData;
use glam::{Vec3, Vec4};
use noise::{NoiseFn, Perlin};

pub fn generate_terrain(
    pos_x: i32,
    pos_z: i32,
    width: i32,
    height: i32,
    depth: i32,
    sample_rate: f32,
    max_height: f32,
    perlin: Perlin,
) -> Vec<InstanceData> {
    let mut instance_data = Vec::new();

    for z in pos_z..depth {
        for y in 0..height {
            for x in pos_x..width {
                let sample_x: f32 = x as f32 * sample_rate;
                let sample_z: f32 = z as f32 * sample_rate;
                let sample: f32 = (perlin.get([sample_x as f64, sample_z as f64]) as f32 + 1.) / 2.;
                let height = sample * max_height;

                if y <= height as i32 {
                    let color = Vec4::new(0.1, 0.5, 0.2, 1.0);
                    let position = Vec3::new((pos_x + x) as f32, height, (pos_z + z) as f32);
                    instance_data.push(InstanceData::new(position, color));
                }
            }
        }
    }

    return instance_data;
}
