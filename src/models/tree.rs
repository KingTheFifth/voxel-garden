use glam::{Quat, Vec3};

use crate::{
    models::primitives::{circle, sphere},
    utils::{BROWN, GREEN},
    Model, Point, Voxel,
};

pub fn tree(_seed: u64) -> Vec<Model> {
    let mut trunk = vec![];
    for y in 0..40 {
        trunk.extend(
            circle(Point::new(0, y, 0), 2.0)
                .into_iter()
                .map(|position| Voxel {
                    position,
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
            .map(|position| Voxel {
                position,
                color: GREEN,
            })
            .collect(),
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };

    vec![trunk, shrub]
}
