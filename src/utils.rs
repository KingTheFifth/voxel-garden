#![allow(unused)]

use std::f32::EPSILON;

use glam::{Mat4, Vec3, Vec4};

use crate::{Point, Voxel};

pub const RED: Vec4 = Vec4::new(1.0, 0.0, 0.0, 1.0);
pub const GREEN: Vec4 = Vec4::new(0.0, 1.0, 0.0, 1.0);
pub const BLUE: Vec4 = Vec4::new(0.0, 0.0, 1.0, 1.0);
pub const WHITE: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
pub const BLACK: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
pub const ORANGE: Vec4 = Vec4::new(1.0, 0.5, 0.0, 1.0);
pub const BROWN: Vec4 = Vec4::new(0.5, 0.2, 0.0, 1.0);
pub const YELLOW: Vec4 = Vec4::new(1.0, 1.0, 0.0, 1.0);

/// Profile a function.
///
/// Put this at the top of a function. The name of the scope is
/// taken from the current function. IDs can be passed as expressions.
///
/// ```
/// fn a_function() {
///   profile_function!();
/// }
///
/// fn another_function(arg: u32) {
///   profile_function!(arg);
/// }
/// ```
#[macro_export]
macro_rules! profile_function {
    ($($expr:expr)?) => {
        #[cfg(feature = "egui")]
        puffin::profile_function!($($expr)?);
    };
}

/// Profile from the current program point until the end of the current scope.
///
/// ```
/// fn a_function() {
///   // some work
///   for a in 0..5 {
///     profile_in_scope!("name of span");  // start of profile span
///     // do something here
///   }                                     // end of profile span
/// }
/// ```
#[macro_export]
macro_rules! profile_in_scope {
    ($name:literal $(, $expr:expr)?) => {
        #[cfg(feature = "egui")]
        puffin::profile_scope!($name $(, $expr)?);
    };
}

/// Profile an expression.
///
/// ```
/// fn a_function() {
///   // some work
///   let abc = profile!("addition", {
///     1 + 2
///   });
/// }
/// ```
#[macro_export]
macro_rules! profile {
    ($name:literal, $what:expr) => {{
        profile_in_scope!($name);
        $what
    }};
}

/// Profile an expression, with an ID.
///
/// Same as [profile] but with an ID.
///
/// ```
/// fn a_function() {
///   // some work
///   let abc = profile!("addition", 123, {
///     1 + 2
///   });
/// }
/// ```
macro_rules! profile_id {
    ($name:literal, $expr:expr, $what:tt) => {{
        profile_in_scope!($name, $expr);
        $what
    }};
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

pub fn voxel_to_vec(iv: &Point) -> Vec3 {
    Vec3::new(iv.x as f32, iv.y as f32, iv.z as f32)
}
