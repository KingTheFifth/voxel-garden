use std::sync::atomic;

use glam::{Quat, Vec3};

use crate::InstanceData;

pub mod flower;
pub mod primitives;
pub mod terrain;
pub mod tree;

pub use flower::flower;
pub use terrain::generate_terrain;
pub use tree::tree;

static NEXT_ID: atomic::AtomicU64 = atomic::AtomicU64::new(0);

#[derive(Clone)]
pub struct Model {
    pub points: Vec<InstanceData>,
    pub rotation: Quat,
    pub translation: Vec3,
}

#[derive(Clone)]
pub struct Object {
    // objects having unique IDs could be useful for debugging at a later stage
    pub _id: String,
    pub models: Vec<Model>,
}

impl Object {
    pub fn new(kind: &str, models: Vec<Model>) -> Self {
        Self {
            _id: format!(
                "{}-{}",
                NEXT_ID.fetch_add(1, atomic::Ordering::SeqCst),
                kind
            ),
            models,
        }
    }
}
