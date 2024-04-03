use std::f32::consts::PI;

use glam::{I64Vec3, Mat4, Vec3};
use miniquad::{
    conf, date, window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, Comparison,
    CullFace, EventHandler, PassAction, Pipeline, PipelineParams, RenderingBackend, ShaderSource,
    UniformsSource, VertexAttribute, VertexFormat,
};

mod models;

type Voxel = I64Vec3;
type Model = Vec<Voxel>;

struct App {
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    prev_t: f64,

    flowers: Vec<Model>,
    model: (Bindings, i32),
    // Beware of the pipeline
}

impl App {
    fn new() -> Self {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();
        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[VertexAttribute::new("in_position", VertexFormat::Float3)],
            shader,
            PipelineParams {
                depth_test: Comparison::Less,
                depth_write: true,
                cull_face: CullFace::Back,
                ..Default::default()
            },
        );

        let d = 0.5;
        #[rustfmt::skip]
        let vertices = [
            Vec3::new(-d, -d, -d),
            Vec3::new( d, -d, -d),
            Vec3::new(-d,  d, -d),
            Vec3::new( d,  d, -d),
            Vec3::new(-d, -d,  d),
            Vec3::new( d, -d,  d),
            Vec3::new(-d,  d,  d),
            Vec3::new( d,  d,  d),
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );
        #[rustfmt::skip]
        let indices = [
            // Back
            0, 2, 1,
            1, 2, 3,
            // Front
            4, 5, 7,
            4, 7, 6,
            // Right
            1, 3, 5,
            5, 3, 7,
            // Left
            4, 6, 0,
            0, 6, 2,
            // Top
            7, 3, 6,
            6, 3, 2,
            // Bottom
            5, 4, 1,
            4, 0, 1,

        ];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        Self {
            ctx,
            pipeline,
            prev_t: 0.0,
            model: (bindings, indices.len() as i32),
            flowers: Vec::new(),
        }
    }
}

impl EventHandler for App {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let t = date::now();
        let delta = (t - self.prev_t) as f32;
        self.prev_t = t;

        self.ctx
            .begin_default_pass(PassAction::clear_color(0.1, 0.1, 0.1, 1.0));
        // Beware the pipeline
        self.ctx.apply_pipeline(&self.pipeline);

        let proj_matrix = Mat4::perspective_rh_gl(PI / 2.0, 1.0, 0.1, 10.0);
        let camera = Mat4::look_at_rh(
            Vec3::new(
                2.0 * t.sin() as f32,
                (t / 2.0).sin() as f32,
                2.0 * t.cos() as f32,
            ),
            Vec3::ZERO,
            Vec3::Y,
        );

        self.ctx.apply_bindings(&self.model.0);
        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                proj_matrix,
                model_matrix: camera,
            }));
        self.ctx.draw(0, self.model.1, 1);

        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }
}

fn main() {
    let conf = conf::Conf {
        window_title: "voxel garden".to_string(),
        window_width: 800,
        window_height: 800,
        ..conf::Conf::default()
    };
    miniquad::start(conf, move || Box::new(App::new()));
}

mod shader {
    use glam::Mat4;
    use miniquad::ShaderMeta;
    use miniquad::UniformBlockLayout;
    use miniquad::UniformDesc;
    use miniquad::UniformType;

    pub const VERTEX: &str = include_str!("shaders/shader.vert");
    pub const FRAGMENT: &str = include_str!("shaders/shader.frag");

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("proj_matrix", UniformType::Mat4),
                    UniformDesc::new("model_matrix", UniformType::Mat4),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub proj_matrix: Mat4,
        pub model_matrix: Mat4,
    }
}
