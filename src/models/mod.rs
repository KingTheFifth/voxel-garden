use glam::{Quat, Vec3};

use crate::InstanceData;

pub mod biomes;
pub mod flower;
pub mod primitives;
pub mod rock;
pub mod terrain;
pub mod tree;

pub use rock::rock;
pub use tree::tree;

#[derive(Clone)]
pub struct Model {
    pub points: Vec<InstanceData>,
    pub rotation: Quat,
    pub translation: Vec3,
}
