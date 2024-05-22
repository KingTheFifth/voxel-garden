use glam::{Quat, Vec3};

use crate::models::primitives::{circle, sphere};
use crate::models::Model;
use crate::utils::{BROWN, GREEN};
use crate::{InstanceData, Point};

pub fn tree(_seed: u64, translation: Vec3) -> Vec<Model> {
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
        translation,
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
        translation,
    };

    vec![trunk, shrub]
}
