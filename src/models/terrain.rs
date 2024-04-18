use crate::{InstanceData, Point};
use glam::{IVec3, Vec3, Vec4};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct TerrainConfig {
    pub sample_rate: f32,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub max_height: f32,
}

pub struct GenerationPositions {
    pub ground: Vec<InstanceData>,
    pub spawn_points: Vec<Point>,
}

impl GenerationPositions {
    fn new(ground: Vec<InstanceData>, spawn_points: Vec<Point>) -> GenerationPositions {
        GenerationPositions {
            ground,
            spawn_points,
        }
    }
}

pub fn generate_terrain(
    pos_x: i32,
    pos_z: i32,
    perlin: Perlin,
    config: TerrainConfig,
) -> GenerationPositions {
    let mut rng = rand::thread_rng();
    let mut instance_data = Vec::new();
    let mut spawn_points: Vec<Point> = Vec::new();
    let depth = config.depth;
    let width = config.width;
    let height = config.height;
    let sample_rate = config.sample_rate;
    let max_height = config.max_height;

    for z in pos_z..depth {
        for y in 0..height {
            for x in pos_x..width {
                let sample_x: f32 = x as f32 * sample_rate;
                let sample_z: f32 = z as f32 * sample_rate;
                let sample: f32 = (perlin.get([sample_x as f64, sample_z as f64]) as f32 + 1.) / 2.;
                let current_height = sample * max_height;

                if y == current_height as i32 {
                    // Generate instance data for ground voxels
                    let color = Vec4::new(0.1, 0.5, 0.2, 1.0);
                    let position =
                        Vec3::new((pos_x + x) as f32, current_height, (pos_z + z) as f32);
                    instance_data.push(InstanceData::new(position, color));

                    let rand: f64 = rng.gen();
                    // Flower
                    if rand < 0.05 {
                        spawn_points.push(IVec3::new(
                            position.x as i32,
                            position.y as i32,
                            position.z as i32,
                        ));
                    }
                }
            }
        }
    }
    GenerationPositions::new(instance_data, spawn_points)
}
