use glam::{Quat, Vec3};

use crate::utils::{BROWN, YELLOW};
use crate::{InstanceData, Model};

pub fn flower(_seed: u64) -> Vec<Model> {
    let stem = Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new(0.0, 0.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 1.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 2.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 3.0, 0.0), color: BROWN },
        ],
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };
    let flower = Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new( 0.0, 0.0,  0.0), color: BROWN },
            InstanceData { position: Vec3::new( 1.0, 0.0,  0.0), color: YELLOW },
            InstanceData { position: Vec3::new(-1.0, 0.0,  0.0), color: YELLOW },
            InstanceData { position: Vec3::new( 0.0, 0.0,  1.0), color: YELLOW },
            InstanceData { position: Vec3::new( 0.0, 0.0, -1.0), color: YELLOW },
        ],
        rotation: Quat::from_rotation_arc(Vec3::Y, Vec3::new(1., 1., 0.).normalize()),
        translation: Vec3::new(0.8, 3.8, 0.),
    };
    vec![stem, flower]
}
