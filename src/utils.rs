#![allow(unused)]

use std::f32::EPSILON;

use glam::{Mat4, Vec3, Vec4};

use crate::Point;

pub const RED: Vec4 = Vec4::new(1.0, 0.0, 0.0, 1.0);
pub const GREEN: Vec4 = Vec4::new(0.0, 1.0, 0.0, 1.0);
pub const BLUE: Vec4 = Vec4::new(0.0, 0.0, 1.0, 1.0);
pub const WHITE: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
pub const BLACK: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
pub const ORANGE: Vec4 = Vec4::new(1.0, 0.5, 0.0, 1.0);
pub const BROWN: Vec4 = Vec4::new(0.5, 0.2, 0.0, 1.0);
pub const YELLOW: Vec4 = Vec4::new(1.0, 1.0, 0.0, 1.0);
pub const PURPLE: Vec4 = Vec4::new(0.5, 0.0, 0.5, 1.0);
pub const GREY: Vec4 = Vec4::new(0.2, 0.2, 0.2, 1.0);
pub const WATER_BLUE: Vec4 = Vec4::new(
    0x04 as f32 / 255.0,
    0xAD as f32 / 255.0,
    0xE2 as f32 / 255.0,
    1.0,
);

/// Port of arbRotate from lab material
pub fn arb_rotate(axis: Vec3, angle: f32) -> Mat4 {
    // Check if parrallel to Z
    if axis.x < EPSILON && axis.x > -EPSILON && axis.y < EPSILON && axis.y > -EPSILON {
        if axis.z > 0.0 {
            return glam::Mat4::from_rotation_z(angle);
        } else {
            return glam::Mat4::from_rotation_z(-angle);
        }
    }

    // Change of basis to basis with rotation axis as x-axis
    let x = axis.normalize();
    let z = glam::Vec3::new(0.0, 0.0, 1.0);
    let y = z.cross(x).normalize();
    let z = x.cross(y);

    #[rustfmt::skip]
    let rot_mat = Mat4::from_cols_array(&[
        x.x, x.y, x.z, 0.0,
        y.x, y.y, y.z, 0.0,
        z.x, z.y, z.z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]);
    let rot_mat_t = rot_mat.transpose();
    let rot_x_mat = Mat4::from_rotation_x(angle);
    rot_mat_t * rot_x_mat * rot_mat
}

pub fn now_f32() -> f32 {
    use std::time::SystemTime;

    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    time.as_secs_f64().rem_euclid(100000.0) as f32
}
