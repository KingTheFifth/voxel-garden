use glam::{Vec3, Vec4};

use crate::Voxel;

pub const RED: Vec4 = Vec4::new(1.0, 0.0, 0.0, 1.0);
pub const GREEN: Vec4 = Vec4::new(0.0, 1.0, 0.0, 1.0);
pub const BLUE: Vec4 = Vec4::new(0.0, 0.0, 1.0, 1.0);
pub const WHITE: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
pub const BLACK: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);

pub fn voxel_to_vec(iv: &Voxel) -> Vec3 {
    Vec3::new(iv.x as f32, iv.y as f32, iv.z as f32)
}
