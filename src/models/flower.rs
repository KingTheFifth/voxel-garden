use glam::{Quat, Vec3};

use crate::utils::{BROWN, YELLOW};
use crate::{Model, Point, Voxel};

pub fn flower(_seed: u64) -> Vec<Model> {
    let stem = Model {
        #[rustfmt::skip]
        points: vec![
            Voxel { position: Point::new(0, 0, 0), color: BROWN },
            Voxel { position: Point::new(0, 1, 0), color: BROWN },
            Voxel { position: Point::new(0, 2, 0), color: BROWN },
            Voxel { position: Point::new(0, 3, 0), color: BROWN },
        ],
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };
    let flower = Model {
        #[rustfmt::skip]
        points: vec![
            Voxel { position: Point::new( 0, 0,  0), color: BROWN },
            Voxel { position: Point::new( 1, 0,  0), color: YELLOW },
            Voxel { position: Point::new(-1, 0,  0), color: YELLOW },
            Voxel { position: Point::new( 0, 0,  1), color: YELLOW },
            Voxel { position: Point::new( 0, 0, -1), color: YELLOW },
        ],
        rotation: Quat::from_rotation_arc(Vec3::Y, Vec3::new(1., 1., 0.).normalize()),
        translation: Vec3::new(0.8, 3.8, 0.),
    };
    vec![stem, flower]
}
