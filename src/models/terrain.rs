use crate::models::biomes::{Biome, BiomeConfig};
use crate::models::flower::proc_gen_flower;
use crate::models::{tree, Model};
use crate::utils::{BROWN, GREEN, RED};
use crate::InstanceData;
use glam::{Vec3, Vec4};
use noise::{NoiseFn, Perlin};

type Object = Vec<Model>;

#[derive(Clone)]
pub enum SpawnType {
    Tree,
    Flower,
    Cactus,
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

pub fn generate_terrain(
    x: i32,
    z: i32,
    config: &TerrainConfig,
    biome_config: &BiomeConfig,
) -> GenerationPositions {
    let mut instance_data = Vec::new();
    let mut spawn_points: Vec<SpawnPoint> = Vec::new();
    let depth = config.depth;
    let width = config.width;
    let mut objects = Vec::new();

    for z in z..z + depth {
        for x in x..x + width {
            let current_height = config.sample(x as f32, z as f32).trunc();

            // Generate instance data for ground voxels
            let biome = biome_config.get_biome(x, z);
            let color = match biome {
                Biome::Forest | Biome::Field => Vec4::new(0.1, 0.5, 0.2, 1.0),
                Biome::Desert => Vec4::new(0.7, 0.7, 0.1, 1.0),
            };
            let position = Vec3::new(x as f32, current_height, z as f32);
            instance_data.push(InstanceData::new(position, color));

            // Biome_config will give some plant to spawn here or not depending on rng
            if let Some(spawn_type) = biome_config.get_spawn_type(x, z) {
                let (color, object) = match spawn_type {
                    SpawnType::Flower => (
                        RED,
                        Some(proc_gen_flower(
                            0,
                            Vec3::new(position.x, position.y + 1., position.z),
                        )),
                    ),
                    SpawnType::Tree => (
                        BROWN,
                        Some(tree(0, Vec3::new(position.x, position.y + 1.0, position.z))),
                    ),
                    SpawnType::Cactus => (GREEN, None),
                };
                spawn_points.push(SpawnPoint::new(
                    InstanceData::new(
                        Vec3::new(
                            position.x,
                            position.y + 1., // We place thing on the ground, not in in
                            position.z,
                        ),
                        color,
                    ),
                    spawn_type,
                ));
                // TODO not option when we generate tree and cactus too
                if let Some(object) = object {
                    objects.push(object);
                }
            }
        }
    }
    GenerationPositions::new_with_object(instance_data, spawn_points, objects)
    //GenerationPositions::new(instance_data, spawn_points)
}
