use glam::{Mat4, Vec3, Vec4};
use miniquad::{
    Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, Comparison, CullFace, GlContext,
    Pipeline, PipelineParams, RenderingBackend as _, ShaderMeta, ShaderSource, TextureFormat,
    TextureId, TextureKind, TextureParams, TextureWrap, UniformBlockLayout, UniformDesc,
    UniformType, UniformsSource, VertexAttribute, VertexFormat, VertexStep,
};
use rand::{thread_rng, Rng as _};

use crate::utils::now_f32;

const VERTEX_SHADER: &str = include_str!("shader.vert");
const FRAGMENT_SHADER: &str = include_str!("shader.frag");

const MAX_INSTANCE_DATA: usize = std::mem::size_of::<InstanceData>() * 100_000;
pub struct Shader {
    /// A pipeline (rendering pipeline) collects information that is applied before draw
    /// calls. It contains:
    ///
    /// - reference to compiled shader program
    /// - vertex attribute configuration (stride, buffer index, etc)
    /// - parameters such as culling, depth test, blends, ...
    pipeline: Pipeline,
    /// The bindings contain vertex buffer IDs, index buffer ID and any texture IDs.
    bindings: Bindings,
    /// Amount of vertices in the cube.
    cube_vertices: i32,

    sun_direction: Vec3,
    sun_color: Vec4,
    ambient_light_color: Vec4,
    ambient_water_activity: f32,
    wave_water_peak: f32,
    wave_water_pow: f32,
    wave_water_x_factor: f32,
    wave_water_z_factor: f32,
    wave_water_frequency: f32,
}

impl Shader {
    pub fn new(ctx: &mut GlContext) -> Self {
        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: VERTEX_SHADER,
                    fragment: FRAGMENT_SHADER,
                },
                meta(),
            )
            .unwrap();

        let d = 0.5;
        #[rustfmt::skip]
        let cube_vertices = [
            VertexData { position: Vec3::new(-d, -d, -d), normal: Vec3::new( -d, 0.0, 0.0).normalize() },
            VertexData { position: Vec3::new( d, -d, -d), normal: Vec3::new(0.0, 0.0,  -d).normalize() },
            VertexData { position: Vec3::new(-d,  d, -d), normal: Vec3::new( -d,   d,  -d).normalize() },
            VertexData { position: Vec3::new( d,  d, -d), normal: Vec3::new(  d,   d,  -d).normalize() },
            VertexData { position: Vec3::new(-d, -d,  d), normal: Vec3::new(0.0,  -d, 0.0).normalize() },
            VertexData { position: Vec3::new( d, -d,  d), normal: Vec3::new(  d, 0.0, 0.0).normalize() },
            VertexData { position: Vec3::new(-d,  d,  d), normal: Vec3::new(0.0,   d, 0.0).normalize() },
            VertexData { position: Vec3::new( d,  d,  d), normal: Vec3::new(0.0, 0.0,   d).normalize() },
        ];
        let cube_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&cube_vertices),
        );

        // Flat shading uses the attributes of the last vertex of a triangle
        // for every fragment in it
        // By making sure that both triangles for a side of a voxel shares the
        // same last vertex, the entire side gets the same attributes such as
        // surface normal
        #[rustfmt::skip]
        let indices = [
            // Back
            0, 2, 1,   2, 3, 1,
            // Front
            4, 5, 7,   6, 4, 7,
            // Right
            1, 3, 5,   3, 7, 5,
            // Left
            4, 6, 0,   6, 2, 0,
            // Top
            7, 3, 6,   3, 2, 6,
            // Bottom
            1, 5, 4,   0, 1, 4,
        ];

        let cube_index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        // Even though this says VertexBuffer, a bit further down we specify a buffer
        // layout with `VertexStep::PerInstance`, meaning the data is the same for
        // every vertex in its instance.
        let instance_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream, // TODO: dynamic?
            BufferSource::empty::<InstanceData>(MAX_INSTANCE_DATA),
        );

        let water_random_tex = generate_random_texture(ctx, 1024, 1024);

        let bindings = Bindings {
            vertex_buffers: vec![cube_vertex_buffer, instance_buffer],
            index_buffer: cube_index_buffer,
            images: vec![water_random_tex],
        };

        let pipeline = ctx.new_pipeline(
            &[
                // buffer 0: geometry vertex buffer
                BufferLayout::default(),
                // buffer 1: instance "vertex" buffer
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..BufferLayout::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("in_position", VertexFormat::Float3, 0),
                VertexAttribute::with_buffer("in_normal", VertexFormat::Float3, 0),
                VertexAttribute::with_buffer("in_inst_position", VertexFormat::Float3, 1),
                VertexAttribute::with_buffer("in_inst_color", VertexFormat::Float4, 1),
                VertexAttribute::with_buffer("is_water", VertexFormat::Int1, 1),
            ],
            shader,
            PipelineParams {
                depth_test: Comparison::Less,
                depth_write: true,
                cull_face: CullFace::Back,
                ..Default::default()
            },
        );

        Shader {
            pipeline,
            bindings,
            cube_vertices: indices.len() as i32,

            sun_direction: Vec3::new(1.0, 1.0, 0.0),
            sun_color: Vec4::new(1.0, 1.0, 0.2, 1.0),
            ambient_light_color: Vec4::new(0.7, 0.7, 0.7, 1.0),
            ambient_water_activity: 0.25,
            wave_water_peak: 0.7,
            wave_water_pow: 8.0,
            wave_water_x_factor: 0.0005,
            wave_water_z_factor: 0.00115,
            wave_water_frequency: 3.0,
        }
    }

    pub fn prepare_draw(&self, ctx: &mut GlContext) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
    }

    pub fn draw_voxels(
        &self,
        ctx: &mut GlContext,
        data: &[InstanceData],
        proj_matrix: Mat4,
        model_matrix: Mat4,
        camera_matrix: Mat4,
    ) {
        ctx.buffer_update(self.bindings.vertex_buffers[1], BufferSource::slice(data));
        ctx.apply_uniforms(UniformsSource::table(&self.uniforms(
            proj_matrix,
            model_matrix,
            camera_matrix,
        )));
        ctx.draw(0, self.cube_vertices, data.len() as i32);
    }

    #[cfg(feature = "egui")]
    pub fn egui_uniform_slider_rows(&mut self, ui: &mut egui::Ui) {
        use egui::color_picker::color_edit_button_rgb;

        ui.label("ambient light color");
        let mut rgb = [
            self.ambient_light_color.x,
            self.ambient_light_color.y,
            self.ambient_light_color.z,
        ];
        color_edit_button_rgb(ui, &mut rgb);
        self.ambient_light_color.x = rgb[0];
        self.ambient_light_color.y = rgb[1];
        self.ambient_light_color.z = rgb[2];
        ui.end_row();

        ui.label("sun color");
        let mut rgb = [self.sun_color.x, self.sun_color.y, self.sun_color.z];
        color_edit_button_rgb(ui, &mut rgb);
        self.sun_color.x = rgb[0];
        self.sun_color.y = rgb[1];
        self.sun_color.z = rgb[2];
        ui.end_row();

        ui.label("ambient water activity");
        ui.add(
            egui::Slider::new(&mut self.ambient_water_activity, (0.0)..=1.0).clamp_to_range(true),
        );
        ui.end_row();

        ui.label("wave water peak");
        ui.add(egui::Slider::new(&mut self.wave_water_peak, (0.0)..=1.0).clamp_to_range(true));
        ui.end_row();

        ui.label("wave water pow");
        ui.add(egui::Slider::new(&mut self.wave_water_pow, (0.0)..=20.0).clamp_to_range(true));
        ui.end_row();

        ui.label("wave water x factor");
        ui.add(
            egui::Slider::new(&mut self.wave_water_x_factor, (0.0)..=0.01)
                .clamp_to_range(true)
                .logarithmic(true),
        );
        ui.end_row();

        ui.label("wave water z factor");
        ui.add(
            egui::Slider::new(&mut self.wave_water_z_factor, (0.0)..=0.01)
                .clamp_to_range(true)
                .logarithmic(true),
        );
        ui.end_row();

        ui.label("wave water frequency");
        ui.add(
            egui::Slider::new(&mut self.wave_water_frequency, (0.0)..=20.0).clamp_to_range(true),
        );
        ui.end_row();
    }

    fn uniforms(&self, proj_matrix: Mat4, model_matrix: Mat4, camera_matrix: Mat4) -> Uniforms {
        Uniforms {
            proj_matrix,
            model_matrix,
            camera_matrix,
            time: now_f32(),
            sun_direction: self.sun_direction,
            sun_color: self.sun_color,
            ambient_light_color: self.ambient_light_color,
            ambient_water_activity: self.ambient_water_activity,
            wave_water_peak: self.wave_water_peak,
            wave_water_pow: self.wave_water_pow,
            wave_water_x_factor: self.wave_water_x_factor,
            wave_water_z_factor: self.wave_water_z_factor,
            wave_water_frequency: self.wave_water_frequency,
        }
    }
}

fn generate_random_texture(ctx: &mut GlContext, width: usize, height: usize) -> TextureId {
    let mut random_bytes = vec![0u8; width * height * 4];
    let mut rng = thread_rng();
    for i in 0..(width * height) {
        random_bytes[i * 4] = rng.gen();
        random_bytes[i * 4 + 1] = rng.gen();
        random_bytes[i * 4 + 2] = rng.gen();
        random_bytes[i * 4 + 3] = 255;
    }
    ctx.new_texture_from_data_and_format(
        &random_bytes,
        TextureParams {
            kind: TextureKind::Texture2D,
            width: width as _,
            height: height as _,
            format: TextureFormat::RGBA8,
            wrap: TextureWrap::Repeat,
            ..Default::default()
        },
    )
}

/// Uploaded vertex data to the GPU
#[repr(C)]
struct VertexData {
    position: Vec3,
    normal: Vec3,
}

/// Uploaded instance data to the GPU
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InstanceData {
    pub position: Vec3,
    pub color: Vec4,
    pub is_water: u32,
}

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
