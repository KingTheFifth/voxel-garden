use glam::{Vec3, Vec4};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

use crate::models::flower::proc_gen_flower;
use crate::models::Model;
use crate::utils::RED;
use crate::InstanceData;

type Object = Vec<Model>;

pub enum SpawnType {
    Tree,
    Flower,
}

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

pub struct SpawnPoint {
    pub instance_data: InstanceData,
    pub spawn_type: SpawnType,
}

impl SpawnPoint {
    pub fn new(instance_data: InstanceData, spawn_type: SpawnType) -> SpawnPoint {
        SpawnPoint {
            instance_data,
            spawn_type,
        }
    }
}

pub struct GenerationPositions {
    pub ground: Vec<InstanceData>,
    pub spawn_points: Vec<SpawnPoint>,
    pub objects: Vec<Object>,
}

impl GenerationPositions {
    fn new(ground: Vec<InstanceData>, spawn_points: Vec<SpawnPoint>) -> GenerationPositions {
        let objects = Vec::new();
        GenerationPositions {
            ground,
            spawn_points,
            objects,
        }
    }

    fn new_with_object(
        ground: Vec<InstanceData>,
        spawn_points: Vec<SpawnPoint>,
        objects: Vec<Object>,
    ) -> GenerationPositions {
        GenerationPositions {
            ground,
            spawn_points,
            objects,
        }
    }
}

pub fn generate_terrain(x: i32, z: i32, config: &TerrainConfig) -> GenerationPositions {
    let mut rng = rand::thread_rng();
    let mut instance_data = Vec::new();
    let mut spawn_points: Vec<SpawnPoint> = Vec::new();
    let depth = config.depth;
    let width = config.width;
    let mut flowers: Vec<Object> = Vec::new();

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
                let instance_data = InstanceData::new(
                    Vec3::new(
                        position.x,
                        position.y + 1., // We place thing on the ground, not in in
                        position.z,
                    ),
                    RED,
                );
                spawn_points.push(SpawnPoint::new(instance_data, SpawnType::Flower));

                let flower = proc_gen_flower(0, Vec3::new(position.x, position.y + 1., position.z));
                flowers.push(flower);
            }
        }
    }
    GenerationPositions::new_with_object(instance_data, spawn_points, flowers)
    //GenerationPositions::new(instance_data, spawn_points)
}
