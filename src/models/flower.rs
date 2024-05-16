use glam::{Quat, Vec3};

use crate::models::Model;
use crate::utils::{BROWN, YELLOW};
use crate::InstanceData;

pub fn flower(_seed: u64, x: f32, y: f32, z: f32) -> Vec<Model> {
    let stem = Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new(x, y, z), color: BROWN },
            InstanceData { position: Vec3::new(x, y + 1.0, z), color: BROWN },
            InstanceData { position: Vec3::new(x, y + 2.0, z), color: BROWN },
            InstanceData { position: Vec3::new(x, y + 3.0, z), color: BROWN },
        ],
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };
    let flower = Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new(x,       y,     z), color: BROWN },
            InstanceData { position: Vec3::new(x +1.0,  y,     z), color: YELLOW },
            InstanceData { position: Vec3::new(x -1.0,  y,     z), color: YELLOW },
            InstanceData { position: Vec3::new(x,       y, z+1.0), color: YELLOW },
            InstanceData { position: Vec3::new(x,       y, z-1.0), color: YELLOW },
        ],
        rotation: Quat::from_rotation_arc(Vec3::Y, Vec3::new(1., 1., 0.).normalize()),
        translation: Vec3::new(0.8, 3.8, 0.),
    };
    vec![stem, flower]
}
