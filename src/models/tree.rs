use glam::{Quat, Vec3};

use crate::models::primitives::{circle, sphere};
use crate::utils::{BROWN, GREEN};
use crate::{InstanceData, Model, Point};

pub fn tree(_seed: u64) -> Vec<Model> {
    let mut trunk = vec![];
    for y in 0..40 {
        trunk.extend(
            circle(Point::new(0, y, 0), 2.0)
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
        translation: Vec3::ZERO,
    };

    let shrub = Model {
        points: sphere(Point::new(0, 40, 0), 10.0)
            .into_iter()
            .map(|Point { x, y, z }| InstanceData {
                position: Vec3::new(x as f32, y as f32, z as f32),
                color: GREEN,
            })
            .collect(),
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };

    vec![trunk, shrub]
}
