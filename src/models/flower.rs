use glam::{Quat, Vec3};

use crate::{utils::BROWN, Model, Point, Voxel};

pub fn flower(_seed: u64) -> Model {
    #[rustfmt::skip]
    let stem = vec![
        Voxel { position: Point::new(0, 0, 0), color: BROWN },
        Voxel { position: Point::new(0, 1, 0), color: BROWN },
        Voxel { position: Point::new(0, 2, 0), color: BROWN },
        Voxel { position: Point::new(0, 3, 0), color: BROWN },
        Voxel { position: Point::new(1, 4, 0), color: BROWN },
    ];
    Model {
        points: stem
            .into_iter() /*.chain()*/
            .collect(),
        rotation: Quat::from_rotation_z(1.0),
        translation: Vec3::ZERO,
    }
}
