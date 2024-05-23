use glam::{Quat, Vec3};
use rand::{thread_rng, Rng};

use crate::models::primitives::{circle, sphere};
use crate::models::Model;
use crate::utils::{BROWN, GREEN};
use crate::{InstanceData, Point};

pub fn tree(_seed: u64, translation: Vec3) -> Vec<Model> {
    let mut trunk = vec![];
    let tree_height = 50;
    for y in 0..tree_height {
        trunk.extend(
            circle(Point::new(0, y, 0), 4.0)
                .into_iter()
                .map(|Point { x, y, z }| InstanceData {
                    position: Vec3::new(x as f32, y as f32, z as f32),
                    color: BROWN,
                }),
        );
    }
    let trunk = Model {
        points: trunk,
        rotation: Quat::IDENTITY,
        translation,
    };

    let mut rng = thread_rng();
    let layers: i32 = 3;
    let mut shrub = vec![];
    for l in 0..layers {
        for _n in 1..=6 - 2 * l {
            shrub.extend(sphere(
                Point::new(
                    rng.gen_range(-(7 - 2 * l)..(7 - 2 * l)),
                    tree_height + 5 * l,
                    rng.gen_range(-(7 - 2 * l)..(7 - 2 * l)),
                ),
                15.0 - (2 * (l + 1)) as f32,
            ));
        }
    }
    let shrub = Model {
        //points: sphere(Point::new(0, tree_height, 0), 15.0)
        points: shrub
            .into_iter()
            .map(|Point { x, y, z }| InstanceData {
                position: Vec3::new(x as f32, y as f32, z as f32),
                color: GREEN,
            })
            .collect(),
        rotation: Quat::IDENTITY,
        translation,
    };

    vec![trunk, shrub]
}
