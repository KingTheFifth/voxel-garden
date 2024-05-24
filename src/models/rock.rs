use crate::models::primitives::sphere;
use crate::models::Model;
use rand::{thread_rng, Rng};
// use rand_chacha::ChaCha8Rng;

use crate::utils::GREY;
use crate::{InstanceData, Point};
use glam::{IVec3, Quat, Vec3};

pub fn rock(_seed: u64, translation: Vec3) -> Vec<Model> {
    // let rng = ChaCha8Rng::from_seed(_seed);
    let mut rng = thread_rng();
    let mut rocks: Vec<Model> = vec![];
    let pebbles: i32 = rng.gen_range(1..=5);
    for _x in 1..=pebbles {
        // Choose random size and position for this pebble
        let radius = rng.gen_range(1..=7);
        let x_offs = rng.gen_range(-radius..radius);
        let y_offs = 0;
        let z_offs = rng.gen_range(-radius..radius);

        // Create a sphere that is the pebble and turn it into instance data
        let sphere = sphere(radius as f32)
            .into_iter()
            .map(|Point { x, y, z }| InstanceData {
                position: Vec3::new(
                    (x + x_offs) as f32,
                    (y + y_offs) as f32,
                    (z + z_offs) as f32,
                ),
                color: GREY,
            })
            .collect();

        // Create a model for the pebble
        rocks.push(Model {
            points: sphere,
            rotation: Quat::IDENTITY,
            translation,
        });
    }
    rocks
}
