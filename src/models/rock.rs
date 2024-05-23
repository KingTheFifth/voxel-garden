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
    let mut rocks = vec![];
    let midpoint: IVec3 = IVec3::new(0, 0, 0);
    let pebbles: i32 = rng.gen_range(1..=5);
    for _x in 1..=pebbles {
        let radius = rng.gen_range(1..=7);
        let offset = IVec3::new(
            rng.gen_range(-radius..radius),
            0,
            rng.gen_range(-radius..radius),
        );
        rocks.extend(sphere(midpoint + offset, radius as f32).into_iter().map(
            |Point { x, y, z }| InstanceData {
                position: Vec3::new(x as f32, y as f32, z as f32),
                color: GREY,
                is_water: 0,
            },
        ));
    }

    let pebble = Model {
        points: rocks,
        rotation: Quat::IDENTITY,
        translation,
    };
    vec![pebble]
}
