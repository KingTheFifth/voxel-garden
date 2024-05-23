use crate::models::terrain::SpawnType;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

pub struct BiomeSpawnData {
    pub spawn_type: SpawnType,
    pub spawn_rate: f32,
    pub group_spawn_rate: f32,
}

impl BiomeSpawnData {
    pub fn new(spawn_type: SpawnType, spawn_rate: f32, group_spawn_rate: f32) -> BiomeSpawnData {
        BiomeSpawnData {
            spawn_type,
            spawn_rate,
            group_spawn_rate,
        }
    }
}

pub enum Biome {
    Forest,
    Field,
    Desert,
}

impl Biome {
    pub fn get_spawn_data(&self) -> Vec<BiomeSpawnData> {
        match self {
            Biome::Forest => vec![
                BiomeSpawnData {
                    spawn_type: SpawnType::Tree,
                    spawn_rate: 0.0001,
                    group_spawn_rate: 0.33,
                },
                BiomeSpawnData {
                    spawn_type: SpawnType::Flower,
                    spawn_rate: 0.01,
                    group_spawn_rate: 0.1,
                },
            ],
            Biome::Field => vec![BiomeSpawnData {
                spawn_type: SpawnType::Flower,
                spawn_rate: 0.02,
                group_spawn_rate: 0.7,
            }],
            Biome::Desert => vec![BiomeSpawnData {
                spawn_type: SpawnType::Cactus,
                spawn_rate: 0.02,
                group_spawn_rate: 0.3,
            }],
        }
    }
}

pub struct BiomeConfig {
    pub noise: Perlin,
    pub biome_sample_rate: f32,
    pub plant_sample_rate: f32,
}

impl BiomeConfig {
    fn sample(&self, sample_rate: f32, x: f32, z: f32) -> f32 {
        let px = x * sample_rate;
        let pz = z * sample_rate;
        let sample = (self.noise.get([px as f64, pz as f64]) as f32 + 1.0) / 2.0;
        sample
    }

    fn sample_biome(&self, x: i32, z: i32) -> f32 {
        self.sample(self.biome_sample_rate, x as f32, z as f32)
    }

    fn sample_plant(&self, x: i32, z: i32) -> f32 {
        self.sample(self.plant_sample_rate, x as f32, z as f32)
    }

    pub fn get_spawn_type(&self, x: i32, z: i32) -> Option<SpawnType> {
        let mut rng = rand::thread_rng();
        let rand: f32 = rng.gen();
        // Flower

        let biome = self.get_biome(x, z);

        // Use the variable p to basically divide the interval [0, 1] into subintervals
        // for each spawn type, e.g. flower [0, 0.2], tree [0.2, 0.5] ...
        let mut p: f32 = 0.0;
        for spawn_data in biome.get_spawn_data() {
            // Random maths that seems to give some nice controlled randomness for spawning different plants
            // TODO: Some less ugly maths
            let group_p = 2.0 * spawn_data.group_spawn_rate * self.sample_plant(x, z);
            let p2 = p + spawn_data.spawn_rate + (if group_p > 0.6 { group_p } else { 0.0 });

            if 0.005 * p2 >= rand {
                return Some(spawn_data.spawn_type);
            } else {
                p += spawn_data.spawn_rate
            }
        }
        None
    }

    pub fn get_biome(&self, x: i32, z: i32) -> Biome {
        let s = 100.0 * self.sample_biome(x, z);

        if s <= 20.0 {
            Biome::Desert
        } else if s <= 50.0 {
            Biome::Field
        } else {
            Biome::Forest
        }
    }
}
