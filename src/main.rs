use std::f32::consts::PI;
use std::mem::size_of;
use std::time::Instant;

use glam::{IVec3, Mat3, Mat4, Quat, Vec3, Vec4};
use miniquad::{
    conf, date, window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, Comparison,
    CullFace, EventHandler, PassAction, Pipeline, PipelineParams, RenderingBackend, ShaderSource,
    UniformsSource, VertexAttribute, VertexFormat, VertexStep,
};
use models::flower::flower;
use models::terrain::generate_terrain;
use noise::Perlin;
use ringbuffer::{AllocRingBuffer, RingBuffer as _};
use utils::arb_rotate;

mod models;
mod utils;

type Point = IVec3;
type Color = Vec4;
type Object = Vec<Model>;

const MAX_INSTANCE_DATA: usize = size_of::<InstanceData>() * 100_000;

struct App {
    ctx: Box<dyn RenderingBackend>,
    #[cfg(feature = "egui")]
    egui_mq: egui_miniquad::EguiMq,
    pipeline: Pipeline,
    prev_t: f64,

    frame_times: AllocRingBuffer<f32>,
    rotation_speed: f64,

    ground: Vec<Voxel>,
    flowers: Vec<Object>,
    cube: (Bindings, i32),
    mouse_left_down: bool,
    mouse_right_down: bool,
    movement: Movement,
}

enum Movement {
    Trackball {
        down_pos: (f32, f32),
        prev_pos: (f32, f32),
        matrix: Mat4,
    },
    Flying,
}

#[derive(Clone, Copy)]
struct Voxel {
    position: Point,
    color: Color,
}

impl Voxel {
    fn new(position: Point, color: Vec4) -> Voxel {
        Voxel { position, color }
    }
}

#[derive(Clone)]
struct Model {
    points: Vec<Voxel>,
    rotation: Quat,
    translation: Vec3,
}

#[repr(C)]
struct InstanceData {
    position: Vec3,
    color: Vec4,
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
        let geometry_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );
        #[rustfmt::skip]
        let indices = [
            // Back
            0, 2, 1,   1, 2, 3,
            // Front
            4, 5, 7,   4, 7, 6,
            // Right
            1, 3, 5,   5, 3, 7,
            // Left
            4, 6, 0,   0, 6, 2,
            // Top
            7, 3, 6,   6, 3, 2,
            // Bottom
            5, 4, 1,   4, 0, 1,

        ];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let positions_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream, // TODO: dynamic?
            BufferSource::empty::<InstanceData>(MAX_INSTANCE_DATA),
        );

        let bindings = Bindings {
            vertex_buffers: vec![geometry_vertex_buffer, positions_vertex_buffer],
            index_buffer,
            images: vec![],
        };

        let pipeline = ctx.new_pipeline(
            &[
                BufferLayout::default(),
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..BufferLayout::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("in_position", VertexFormat::Float3, 0),
                VertexAttribute::with_buffer("in_inst_position", VertexFormat::Float3, 1), // TODO: VertexFormat::Int32?
                VertexAttribute::with_buffer("in_inst_color", VertexFormat::Float4, 1),
            ],
            shader,
            PipelineParams {
                depth_test: Comparison::Less,
                depth_write: true,
                cull_face: CullFace::Back,
                ..Default::default()
            },
        );
        // let voxels = bresenham(Voxel::ZERO, Voxel::new(10, 5, 3));

        Self {
            #[cfg(feature = "egui")]
            egui_mq: egui_miniquad::EguiMq::new(&mut *ctx),
            ctx,
            pipeline,
            prev_t: 0.0,
            frame_times: AllocRingBuffer::new(10),
            rotation_speed: 1.0,
            ground: generate_terrain(-50, -50, 200, 20, 200, 0.013, 20.0, Perlin::new(555)),
            cube: (bindings, indices.len() as i32),
            flowers: vec![flower(0)],
            mouse_left_down: false,
            mouse_right_down: false,
            movement: Movement::Trackball {
                down_pos: (0.0, 0.0),
                prev_pos: (0.0, 0.0),
                matrix: Mat4::IDENTITY,
            },
        }
    }

    #[cfg(feature = "egui")]
    fn egui_ui(&mut self) {
        self.egui_mq.run(&mut *self.ctx, |_ctx, egui_ctx| {
            egui::TopBottomPanel::top("top bar").show(egui_ctx, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        unimplemented!("this is ironic");
                    }
                });
            });

            egui::Window::new("Debug").show(egui_ctx, |ui| {
                // temporary, to show how to change values
                ui.add(
                    egui::Slider::new(&mut self.rotation_speed, (0.1)..=10.0).clamp_to_range(true),
                );

                ui.label(format!(
                    "Average frame time: {:.2} ms",
                    self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32
                ));
            });
        });

        self.egui_mq.draw(&mut *self.ctx);
    }

    fn camera_matrix(&mut self) -> Mat4 {
        let scale = 5.0;
        Mat4::look_at_rh(
            scale * Vec3::new(0.0, 0.0, 5.0),
            scale * Vec3::ZERO,
            Vec3::Y,
        )
    }
}

fn trackball_control(camera_matrix: Mat4, screen_pos: (f32, f32), prev_pos: (f32, f32)) -> Mat4 {
    let axis = Vec3::new(screen_pos.1 - prev_pos.1, prev_pos.0 - screen_pos.0, 0.0);
    let axis = Mat3::from_mat4(camera_matrix).inverse() * axis;
    arb_rotate(axis, axis.length() / 50.0)
}

impl EventHandler for App {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let draw_start = Instant::now();

        let t = date::now();
        let _delta = (t - self.prev_t) as f32;
        self.prev_t = t;

        self.ctx
            .begin_default_pass(PassAction::clear_color(0.1, 0.1, 0.1, 1.0));
        // Beware the pipeline
        self.ctx.apply_pipeline(&self.pipeline);

        let proj_matrix = Mat4::perspective_rh_gl(PI / 2.0, 1.0, 0.1, 1000.0);
        let camera = match self.movement {
            Movement::Trackball {
                down_pos: _,
                prev_pos: _,
                matrix: trackball_matrix,
            } => self.camera_matrix() * trackball_matrix,
            Movement::Flying => self.camera_matrix(),
        };

        self.ctx.apply_bindings(&self.cube.0);
        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                proj_matrix,
                model_matrix: camera,
            }));
        self.ctx.apply_bindings(&self.cube.0);

        // Draw ground
        self.ctx.buffer_update(
            self.cube.0.vertex_buffers[1],
            BufferSource::slice(
                &self
                    .ground
                    .iter()
                    .map(|voxel| InstanceData {
                        position: Vec3::new(
                            voxel.position.x as f32,
                            voxel.position.y as f32,
                            voxel.position.z as f32,
                        ),
                        color: Vec4::new(
                            voxel.color.x,
                            voxel.color.y,
                            voxel.color.z,
                            voxel.color.w,
                        ),
                    })
                    .collect::<Vec<_>>(),
            ),
        );
        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                proj_matrix,
                model_matrix: camera,
            }));
        self.ctx.draw(0, self.cube.1, self.ground.len() as i32);

        // Draw objects
        let objects = self.flowers.iter();
        for model in objects.flatten() {
            let instance_data: Vec<_> = model
                .points
                .iter()
                .copied()
                .map(
                    |Voxel {
                         position: Point { x, y, z },
                         color,
                     }| InstanceData {
                        position: Vec3::new(x as f32, y as f32, z as f32),
                        color,
                    },
                )
                .collect();
            self.ctx.buffer_update(
                self.cube.0.vertex_buffers[1],
                BufferSource::slice(&instance_data),
            );
            self.ctx
                .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                    proj_matrix,
                    model_matrix: camera
                        * Mat4::from_rotation_translation(model.rotation, model.translation),
                }));
            self.ctx.draw(0, self.cube.1, model.points.len() as i32);
        }

        self.ctx.end_render_pass();

        #[cfg(feature = "egui")]
        self.egui_ui();

        self.ctx.commit_frame();

        let draw_end = Instant::now();
        self.frame_times
            .push(draw_end.duration_since(draw_start).as_secs_f32() * 1000.0)
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        #[cfg(feature = "egui")]
        self.egui_mq.mouse_motion_event(x, y);

        let camera_matrix = self.camera_matrix();
        match &mut self.movement {
            Movement::Trackball {
                down_pos: _,
                prev_pos,
                matrix,
            } => {
                if self.mouse_left_down {
                    *matrix = trackball_control(camera_matrix, (x, y), *prev_pos) * *matrix;
                }
                *prev_pos = (x, y);
            }
            Movement::Flying => todo!(),
        }
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        #[cfg(feature = "egui")]
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: miniquad::MouseButton, x: f32, y: f32) {
        #[cfg(feature = "egui")]
        self.egui_mq.mouse_button_down_event(mb, x, y);

        match &mut self.movement {
            Movement::Trackball {
                down_pos,
                prev_pos,
                matrix: _,
            } => {
                *down_pos = (x, y);
                *prev_pos = (x, y);
            }
            Movement::Flying => todo!(),
        }
        match mb {
            miniquad::MouseButton::Left => self.mouse_left_down = true,
            miniquad::MouseButton::Right => self.mouse_right_down = true,
            _ => {}
        }
    }

    fn mouse_button_up_event(&mut self, mb: miniquad::MouseButton, x: f32, y: f32) {
        #[cfg(feature = "egui")]
        self.egui_mq.mouse_button_up_event(mb, x, y);

        match mb {
            miniquad::MouseButton::Left => self.mouse_left_down = false,
            miniquad::MouseButton::Right => self.mouse_right_down = false,
            _ => {}
        }
    }

    fn char_event(&mut self, character: char, _keymods: miniquad::KeyMods, _repeat: bool) {
        #[cfg(feature = "egui")]
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        keycode: miniquad::KeyCode,
        keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        #[cfg(feature = "egui")]
        self.egui_mq.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        #[cfg(feature = "egui")]
        self.egui_mq.key_up_event(keycode, keymods);
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
