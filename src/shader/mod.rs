use glam::{Mat4, Vec3, Vec4};
use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc, UniformType};

pub const VERTEX: &str = include_str!("shader.vert");
pub const FRAGMENT: &str = include_str!("shader.frag");

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["water_random".to_string()],
        uniforms: UniformBlockLayout {
            // The order here needs to match the order in the Uniforms struct
            uniforms: vec![
                UniformDesc::new("proj_matrix", UniformType::Mat4),
                UniformDesc::new("model_matrix", UniformType::Mat4),
                UniformDesc::new("camera_matrix", UniformType::Mat4),
                UniformDesc::new("sun_direction", UniformType::Float3),
                UniformDesc::new("time", UniformType::Float1),
                UniformDesc::new("sun_color", UniformType::Float4),
                UniformDesc::new("ambient_light_color", UniformType::Float4),
                UniformDesc::new("ambient_water_activity", UniformType::Float1),
                UniformDesc::new("wave_water_peak", UniformType::Float1),
                UniformDesc::new("wave_water_pow", UniformType::Float1),
                UniformDesc::new("wave_water_x_factor", UniformType::Float1),
                UniformDesc::new("wave_water_z_factor", UniformType::Float1),
                UniformDesc::new("wave_water_frequency", UniformType::Float1),
            ],
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub proj_matrix: Mat4,
    pub model_matrix: Mat4,
    pub camera_matrix: Mat4,
    pub sun_direction: Vec3,
    pub time: f32,
    pub sun_color: Vec4,
    pub ambient_light_color: Vec4,
    pub ambient_water_activity: f32,
    pub wave_water_peak: f32,
    pub wave_water_pow: f32,
    pub wave_water_x_factor: f32,
    pub wave_water_z_factor: f32,
    pub wave_water_frequency: f32,
}
