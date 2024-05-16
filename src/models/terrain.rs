use crate::utils::RED;
use crate::InstanceData;
use glam::{Vec3, Vec4};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

#[derive(Clone)]
pub struct TerrainConfig {
    pub sample_rate: f32,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub max_height: f32,
    pub noise: Perlin,
}

impl TerrainConfig {
    pub fn sample(&self, x: f32, z: f32) -> f32 {
        let px = x * self.sample_rate;
        let pz = z * self.sample_rate;
        let sample = (self.noise.get([px as f64, pz as f64]) as f32 + 1.0) / 2.0;
        self.max_height * sample
    }
}

pub struct GenerationPositions {
    pub ground: Vec<InstanceData>,
    pub spawn_points: Vec<InstanceData>,
}

impl GenerationPositions {
    fn new(ground: Vec<InstanceData>, spawn_points: Vec<InstanceData>) -> GenerationPositions {
        GenerationPositions {
            ground,
            spawn_points,
        }
    }
}

pub fn generate_terrain(x: i32, z: i32, config: &TerrainConfig) -> GenerationPositions {
    let mut rng = rand::thread_rng();
    let mut instance_data = Vec::new();
    let mut spawn_points: Vec<InstanceData> = Vec::new();
    let depth = config.depth;
    let width = config.width;

    for z in z..z + depth {
        for x in x..x + width {
            let current_height = config.sample(x as f32, z as f32).trunc();

            // Generate instance data for ground voxels
            let color = Vec4::new(0.1, 0.5, 0.2, 1.0);
            let position = Vec3::new(x as f32, current_height, z as f32);
            instance_data.push(InstanceData::new(position, color));

            let rand: f64 = rng.gen();
            // Flower
            if rand < 0.05 {
                spawn_points.push(InstanceData::new(
                    Vec3::new(
                        position.x,
                        position.y + 1., // We place thing on the ground, not in it
                        position.z,
                    ),
                    RED,
                ));
            }
        }
    }
    GenerationPositions::new(instance_data, spawn_points)
}
