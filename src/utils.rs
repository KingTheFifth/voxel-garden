use std::f32::EPSILON;

use glam::{Mat4, Vec3};

#[macro_export]
macro_rules! profile_function {
    ($($expr:expr)?) => {
        #[cfg(feature = "egui")]
        puffin::profile_function!($($expr)?);
    };
}

#[macro_export]
macro_rules! profile_scope {
    ($name:literal $(, $expr:expr)?) => {
        #[cfg(feature = "egui")]
        puffin::profile_scope!($name $(, $expr)?);
    };
}

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
