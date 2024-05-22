use std::mem::size_of;
use std::{collections::HashMap, f32::consts::PI};

use glam::{IVec2, IVec3, Mat4, Quat, Vec2, Vec3, Vec4};
use miniquad::{
    conf, date, window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, Comparison,
    CullFace, EventHandler, KeyCode, PassAction, Pipeline, PipelineParams, RenderingBackend,
    ShaderSource, UniformsSource, VertexAttribute, VertexFormat, VertexStep,
};
use models::biomes::BiomeConfig;
use models::terrain::{generate_terrain, GenerationPositions, TerrainConfig};
use noise::Perlin;
use ringbuffer::{AllocRingBuffer, RingBuffer as _};

use crate::camera::{trackball_control, Movement};

mod camera;
mod models;
mod utils;

type Point = IVec3;
type Color = Vec4;

const MAX_INSTANCE_DATA: usize = size_of::<InstanceData>() * 100_000;
const CHUNK_SIZE: i32 = 32;

struct App {
    ctx: Box<dyn RenderingBackend>,
    aspect_ratio: f32,
    fov_y_radians: f32,
    #[cfg(feature = "egui")]
    egui_mq: egui_miniquad::EguiMq,
    pipeline: Pipeline,
    cube: (Bindings, i32),

    frame_times: AllocRingBuffer<f32>,

    prev_update: f64,
    prev_draw: f64,
    fps_history: AllocRingBuffer<f32>,
    view_fps_graph: bool,

    // This is per-chunk
    biome_config: BiomeConfig,
    terrain_config: TerrainConfig,
    terrain: HashMap<IVec2, GenerationPositions>,
    voxels: Vec<Voxel>,

    sun_direction: Vec3,
    sun_color: Vec3,

    keys_down: HashMap<KeyCode, bool>,
    mouse_left_down: bool,
    mouse_right_down: bool,
    mouse_prev_pos: (f32, f32),
    movement: Movement,
    render_distance: i32,
}

#[derive(Clone, Copy, Debug)]
struct Voxel {
    position: Point,
    color: Color,
}

#[repr(C)]
struct VertexData {
    position: Vec3,
    normal: Vec3,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct InstanceData {
    position: Vec3,
    color: Vec4,
}

impl InstanceData {
    fn new(position: Vec3, color: Vec4) -> InstanceData {
        InstanceData { position, color }
    }
}

impl App {
    fn new() -> Self {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();
        let (window_width, window_height) = window::screen_size();
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
            VertexData { position: Vec3::new(-d, -d, -d), normal: Vec3::new( -d, 0.0, 0.0).normalize() },
            VertexData { position: Vec3::new( d, -d, -d), normal: Vec3::new(  0.0, 0.0, -d).normalize() },
            VertexData { position: Vec3::new(-d,  d, -d), normal: Vec3::new( -d,  d, -d).normalize() },
            VertexData { position: Vec3::new( d,  d, -d), normal: Vec3::new(  d,  d, -d).normalize() },
            VertexData { position: Vec3::new(-d, -d,  d), normal: Vec3::new( 0.0, -d,  0.0).normalize() },
            VertexData { position: Vec3::new( d, -d,  d), normal: Vec3::new(  d, 0.0,  0.0).normalize() },
            VertexData { position: Vec3::new(-d,  d,  d), normal: Vec3::new( 0.0,  d,  0.0).normalize() },
            VertexData { position: Vec3::new( d,  d,  d), normal: Vec3::new(  0.0,  0.0,  d).normalize() },
        ];

        let geometry_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
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
                VertexAttribute::with_buffer("in_normal", VertexFormat::Float3, 0),
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

        let terrain_config = TerrainConfig {
            sample_rate: 0.004,
            width: CHUNK_SIZE,
            height: 20,
            depth: CHUNK_SIZE,
            max_height: 40.,
            noise: Perlin::new(555),
        };

        let biome_config = BiomeConfig {
            biome_sample_rate: 0.001,
            plant_sample_rate: 0.3,
            noise: Perlin::new(666),
        };

        let mut app = Self {
            #[cfg(feature = "egui")]
            egui_mq: egui_miniquad::EguiMq::new(&mut *ctx),
            ctx,
            aspect_ratio: 1.0,
            fov_y_radians: 1.0,
            pipeline,
            frame_times: AllocRingBuffer::new(10),
            biome_config,
            terrain_config,
            prev_update: 0.0,
            prev_draw: 0.0,
            fps_history: AllocRingBuffer::new(256),
            view_fps_graph: false,
            terrain: HashMap::new(),
            cube: (bindings, indices.len() as i32),
            sun_direction: Vec3::new(0.0, 1.0, 0.0),
            sun_color: Vec3::new(0.99, 0.72, 0.075),
            voxels: vec![],
            keys_down: HashMap::new(),
            mouse_left_down: false,
            mouse_right_down: false,
            mouse_prev_pos: (0.0, 0.0),
            movement: Movement::OnGround {
                position: Vec3::ZERO,
                velocity: Vec3::ZERO,
                look_h: 0.0,
                look_v: 0.0,
            },
            render_distance: 8,
            // movement: Movement::Trackball {
            //     down_pos: (0.0, 0.0),
            //     matrix: Mat4::IDENTITY,
            // },
        };
        app.resize_event(window_width, window_height);
        app
    }

    #[cfg(feature = "egui")]
    fn egui_ui(&mut self) {
        use egui::{TopBottomPanel, Window};
        use egui_plot::{Line, Plot, PlotPoints};

        self.egui_mq.run(&mut *self.ctx, |_ctx, egui_ctx| {
            TopBottomPanel::top("top bar").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            unimplemented!("this is ironic");
                        }
                    });
                    ui.menu_button("View", |ui| {
                        ui.checkbox(&mut self.view_fps_graph, "FPS graph");
                        ui.separator();
                        if ui.button("Switch to trackball camera").clicked() {
                            self.movement = Movement::Trackball {
                                down_pos: (0.0, 0.0),
                                matrix: Mat4::IDENTITY,
                            }
                        }
                        if ui.button("Switch to flying camera").clicked() {
                            self.movement = Movement::Flying {
                                position: Vec3::ZERO,
                                look_h: 0.0,
                                look_v: 0.0,
                            };
                        }
                    });
                });
            });

            egui::Window::new("Debug").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("render distance");
                    ui.add(
                        egui::Slider::new(&mut self.render_distance, 1..=32).clamp_to_range(true),
                    );
                });

                ui.label(format!(
                    "Average frame time: {:.2} ms",
                    self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32
                ));
            });
            Window::new("FPS")
                .collapsible(false)
                .open(&mut self.view_fps_graph)
                .default_height(200.0)
                .default_width(300.0)
                .show(egui_ctx, |ui| {
                    let fps_points: PlotPoints = self
                        .fps_history
                        .iter()
                        .enumerate()
                        .map(|(x, y)| [x as f64, *y as f64])
                        .collect();
                    Plot::new("fps plot")
                        .include_y(0.0)
                        .include_y(60.0)
                        .allow_drag(false)
                        .allow_scroll(false)
                        .show_background(false)
                        .show_x(false)
                        .show(ui, |plot_ui| plot_ui.line(Line::new(fps_points)));
                });
        });

        self.egui_mq.draw(&mut *self.ctx);
    }

    fn generate_chunk(
        biome_config: &BiomeConfig,
        terrain_config: &TerrainConfig,
        chunk: IVec2,
    ) -> GenerationPositions {
        generate_terrain(
            chunk.x * CHUNK_SIZE,
            chunk.y * CHUNK_SIZE,
            terrain_config,
            biome_config,
        )
    }

    fn draw_chunk(
        &mut self,
        projection: Mat4,
        camera: Mat4,
        camera_position: IVec2,
        camera_look_h: Option<f32>,
    ) {
        let camera_chunk = IVec2::new(
            camera_position.x / CHUNK_SIZE,
            camera_position.y / CHUNK_SIZE,
        );
        for dy in -self.render_distance..=self.render_distance {
            for dx in -self.render_distance..=self.render_distance {
                if let Some(camera_look_h) = camera_look_h {
                    let (vs, vc) = camera_look_h.sin_cos();
                    let look_v = Vec2::new(vs, vc).normalize();
                    if {
                        let d_chunk = Vec2::new(dx as f32, dy as f32);
                        look_v.dot(d_chunk) < 0.0
                    } {
                        continue;
                    }
                }
                let d_chunk = IVec2::new(dx, dy);
                let chunk_data = self
                    .terrain
                    .entry(camera_chunk + d_chunk)
                    .or_insert_with(|| {
                        Self::generate_chunk(
                            &self.biome_config,
                            &self.terrain_config,
                            camera_chunk + d_chunk,
                        )
                    });

                let spawn_point_instance_data: Vec<_> = chunk_data
                    .spawn_points
                    .iter()
                    .map(|sp| sp.instance_data)
                    .collect();

                // Draw ground
                self.ctx.buffer_update(
                    self.cube.0.vertex_buffers[1],
                    BufferSource::slice(&chunk_data.ground),
                );
                self.ctx
                    .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                        proj_matrix: projection,
                        model_matrix: camera,
                        camera_matrix: camera,
                        sun_direction: self.sun_direction,
                        sun_color: self.sun_color,
                    }));
                self.ctx
                    .draw(0, self.cube.1, chunk_data.ground.len() as i32);

                // draw spawn points
                // dont need to apply uniforms since spawn points
                // can be treated as ground voxels
                self.ctx.buffer_update(
                    self.cube.0.vertex_buffers[1],
                    BufferSource::slice(&spawn_point_instance_data),
                );
                self.ctx
                    .draw(0, self.cube.1, chunk_data.spawn_points.len() as i32);

                // draw models
                let models = self
                    .terrain
                    .entry(camera_chunk + d_chunk)
                    .or_insert_with(|| {
                        Self::generate_chunk(
                            &self.biome_config,
                            &self.terrain_config,
                            camera_chunk + d_chunk,
                        )
                    })
                    .objects
                    .iter()
                    .flatten();

                for model in models {
                    self.ctx.buffer_update(
                        self.cube.0.vertex_buffers[1],
                        BufferSource::slice(&model.points),
                    );
                    self.ctx
                        .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                            proj_matrix: projection,
                            model_matrix: camera
                                * Mat4::from_rotation_translation(
                                    model.rotation,
                                    model.translation,
                                ),
                            camera_matrix: camera,
                            sun_direction: self.sun_direction,
                            sun_color: self.sun_color,
                        }));
                    self.ctx.draw(0, self.cube.1, model.points.len() as i32);
                }
            }
        }
    }
}

impl EventHandler for App {
    fn update(&mut self) {
        let now = date::now();
        let delta = (now - self.prev_update) as f32;
        self.prev_update = now;

        match &mut self.movement {
            Movement::Trackball { .. } => {}
            Movement::Flying {
                position,
                look_h,
                look_v,
            } => {
                let mut movement_vector = Vec4::ZERO;
                if self.keys_down.get(&KeyCode::W).copied().unwrap_or(false) {
                    // forward
                    movement_vector += Vec4::Z;
                }
                if self.keys_down.get(&KeyCode::S).copied().unwrap_or(false) {
                    // backward
                    movement_vector += -Vec4::Z;
                }
                if self.keys_down.get(&KeyCode::A).copied().unwrap_or(false) {
                    // left
                    movement_vector += Vec4::X;
                }
                if self.keys_down.get(&KeyCode::D).copied().unwrap_or(false) {
                    // right
                    movement_vector += -Vec4::X;
                }
                if self.keys_down.get(&KeyCode::R).copied().unwrap_or(false) {
                    movement_vector += Vec4::Y;
                }
                if self.keys_down.get(&KeyCode::F).copied().unwrap_or(false) {
                    movement_vector += -Vec4::Y;
                }
                if movement_vector.length_squared() != 0.0 {
                    let rot_mat = Mat4::from_quat(
                        (Quat::from_rotation_y(*look_h) * Quat::from_rotation_x(*look_v))
                            .normalize(),
                    );
                    *position += (rot_mat * movement_vector.normalize()).truncate() * delta * 10.0;
                }
            }
            Movement::OnGround {
                position,
                velocity,
                look_h,
                look_v: _,
            } => {
                let mut movement_vector = Vec4::ZERO;
                if self.keys_down.get(&KeyCode::W).copied().unwrap_or(false) {
                    movement_vector += Vec4::Z;
                }
                if self.keys_down.get(&KeyCode::S).copied().unwrap_or(false) {
                    movement_vector += -Vec4::Z;
                }
                if self.keys_down.get(&KeyCode::A).copied().unwrap_or(false) {
                    movement_vector += Vec4::X;
                }
                if self.keys_down.get(&KeyCode::D).copied().unwrap_or(false) {
                    movement_vector += -Vec4::X;
                }
                if movement_vector.length_squared() != 0.0 {
                    let rot_mat = Mat4::from_quat(Quat::from_rotation_y(*look_h).normalize());
                    *position += (rot_mat * movement_vector.normalize()).truncate() * delta * 40.0;
                }

                let height_at_p = self.terrain_config.sample(position.x, position.z) + 20.0;

                let mut on_ground = position.y <= height_at_p;
                if on_ground
                    && self
                        .keys_down
                        .get(&KeyCode::Space)
                        .copied()
                        .unwrap_or(false)
                {
                    velocity.y = 50.0;
                    on_ground = false;
                }

                if on_ground {
                    *velocity = Vec3::ZERO;
                    position.y = height_at_p;
                } else {
                    velocity.y -= 100.0 * delta; // gravity
                    *position += delta * *velocity;
                }
            }
        }
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
        self.fov_y_radians = height / 1000.0;
    }

    fn draw(&mut self) {
        let now = date::now();
        let draw_delta = (now - self.prev_draw) as f32;
        self.prev_draw = now;
        self.fps_history.push(1.0 / draw_delta);

        self.ctx
            .begin_default_pass(PassAction::clear_color(0.1, 0.1, 0.1, 1.0));
        // Beware the pipeline
        self.ctx.apply_pipeline(&self.pipeline);

        let projection =
            Mat4::perspective_rh_gl(self.fov_y_radians, self.aspect_ratio, 0.1, 1000.0);
        // FIXME: uh oh, sthinky
        let camera = self.movement.camera_matrix()
            * match self.movement {
                Movement::Trackball { matrix, .. } => matrix,
                _ => Mat4::IDENTITY,
            };

        let camera_position_2d = match self.movement {
            Movement::Trackball { .. } => IVec2::new(0, 0),
            Movement::Flying { position, .. } | Movement::OnGround { position, .. } => {
                IVec2::new(position.x.trunc() as i32, position.z.trunc() as i32)
            }
        };
        let camera_look_h = match self.movement {
            Movement::Trackball { .. } => None,
            Movement::Flying { look_h, .. } | Movement::OnGround { look_h, .. } => Some(look_h),
        };

        self.ctx.apply_bindings(&self.cube.0);
        self.draw_chunk(projection, camera, camera_position_2d, camera_look_h);
        //self.draw_voxels(projection, camera); Use this when?

        self.ctx.end_render_pass();

        #[cfg(feature = "egui")]
        self.egui_ui();

        self.ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        #[cfg(feature = "egui")]
        self.egui_mq.mouse_motion_event(x, y);

        let camera_matrix = self.movement.camera_matrix();
        match &mut self.movement {
            Movement::Trackball {
                down_pos: _,
                matrix,
            } => {
                if self.mouse_left_down {
                    *matrix =
                        trackball_control(camera_matrix, (x, y), self.mouse_prev_pos) * *matrix;
                }
            }
            Movement::Flying {
                position: _,
                look_h,
                look_v,
            }
            | Movement::OnGround {
                position: _,
                velocity: _,
                look_h,
                look_v,
            } => {
                *look_h += (self.mouse_prev_pos.0 - x) / 100.0;
                *look_v = (*look_v - (self.mouse_prev_pos.1 - y) / 100.0)
                    .clamp(-PI / 2.0 + 0.01, PI / 2.0 - 0.01);
            }
        }
        self.mouse_prev_pos = (x, y);
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
                matrix: _,
            } => {
                *down_pos = (x, y);
            }
            Movement::Flying { .. } => {}
            Movement::OnGround { .. } => {}
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

        self.keys_down.insert(keycode, true);
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        #[cfg(feature = "egui")]
        self.egui_mq.key_up_event(keycode, keymods);

        self.keys_down.insert(keycode, false);
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
    use glam::Vec3;
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
                    UniformDesc::new("camera_matrix", UniformType::Mat4),
                    UniformDesc::new("sun_direction", UniformType::Float3),
                    UniformDesc::new("sun_color", UniformType::Float3),
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
        pub sun_color: Vec3,
    }
}
