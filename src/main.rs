use std::collections::HashSet;
use std::sync::{mpsc, Arc, Mutex};
use std::time::SystemTime;
use std::{collections::HashMap, f32::consts::PI};

use glam::{IVec2, IVec3, Mat4, Quat, Vec2, Vec3, Vec4};
use miniquad::{
    conf, date, window, EventHandler, GlContext, KeyCode, PassAction, RenderingBackend as _,
};
use noise::Perlin;
use ringbuffer::{AllocRingBuffer, RingBuffer as _};

use crate::camera::{trackball_control, Movement};
use crate::models::biomes::BiomeConfig;
use crate::models::terrain::{generate_terrain, GenerationPositions, TerrainConfig};
pub use crate::shader::InstanceData;
use crate::shader::{Shader, Uniforms};

mod camera;
mod models;
mod shader;
mod utils;

type Point = IVec3;
type Terrain = HashMap<IVec2, GenerationPositions>;

const CHUNK_SIZE: i32 = 32;

/// Contains state used by the application.
struct App {
    /// The rendering context contains all state related to OpenGL managed by miniquad.
    ctx: GlContext,
    shader: Shader,
    /// Current aspect ratio of the window. Used to calculate the perspective matrix.
    aspect_ratio: f32,
    /// Current target vertical FOV. Used to calculate the perspective matrix.
    fov_y_radians: f32,
    /// Contains state required for integrating the GUI library with miniquad.
    #[cfg(feature = "egui")]
    egui_mq: egui_miniquad::EguiMq,

    /// The time at the previous call to update()
    prev_update: f64,
    /// The time at the previous call to draw()
    prev_draw: f64,
    /// Collecst the N latest FPS values. Used for the FPS graph.
    fps_history: AllocRingBuffer<f32>,

    // This is per-chunk
    terrain: Arc<Mutex<Terrain>>,
    terrain_config: TerrainConfig,
    // Used to get timedifference for water waves
    system_time: SystemTime,
    terrain_chunk_gen_queue: mpsc::Sender<IVec2>,
    terrain_chunk_waiting: HashSet<IVec2>,

    sun_direction: Vec3,
    sun_color: Vec4,
    ambient_light_color: Vec4,
    ambient_water_activity: f32,
    wave_water_peak: f32,
    wave_water_pow: f32,
    wave_water_x_factor: f32,
    wave_water_z_factor: f32,
    wave_water_frequency: f32,

    keys_down: HashMap<KeyCode, bool>,
    keys_just_pressed: HashSet<KeyCode>,
    mouse_left_down: bool,
    mouse_right_down: bool,
    /// Mouse position previous frame (we only get absolute coordinates so need to calculate the delta movement manually).
    mouse_prev_pos: (f32, f32),
    /// Currently active camera movement (enum, so always one of trackball, flying and on-ground).
    movement: Movement,
    flying_movement_speed: f32,
    on_ground_movement_speed: f32,
    lock_mouse: bool,
    /// How many chunks to render in each direction from the camera.
    render_distance: i32,
}

impl App {
    fn new() -> Self {
        let mut ctx = GlContext::new();
        let (window_width, window_height) = window::screen_size();

        let shader = shader::Shader::new(&mut ctx);

        let terrain_config = TerrainConfig {
            sample_rate: 0.004,
            width: CHUNK_SIZE,
            height: 20,
            depth: CHUNK_SIZE,
            max_height: 40.,
            min_height: 6.,
            noise: Perlin::new(555),
        };

        let biome_config = BiomeConfig {
            biome_sample_rate: 0.001,
            plant_sample_rate: 0.3,
            noise: Perlin::new(666),
        };

        let terrain = Arc::new(Mutex::new(HashMap::new()));
        let terrain_chunk_gen_queue = mpsc::channel();

        {
            let terrain = terrain.clone();
            let terrain_config = terrain_config.clone();
            std::thread::spawn(move || {
                terrain_gen_thread(
                    biome_config,
                    terrain_config,
                    terrain,
                    terrain_chunk_gen_queue.1,
                )
            });
        }

        let mut app = Self {
            #[cfg(feature = "egui")]
            egui_mq: egui_miniquad::EguiMq::new(&mut ctx),

            ctx,
            shader,
            aspect_ratio: 1.0,
            fov_y_radians: 1.0,
            prev_update: 0.0,
            prev_draw: 0.0,
            fps_history: AllocRingBuffer::new(100),
            terrain,
            terrain_config,
            terrain_chunk_gen_queue: terrain_chunk_gen_queue.0,
            terrain_chunk_waiting: HashSet::new(),
            sun_direction: Vec3::new(1.0, 1.0, 0.0),
            sun_color: Vec4::new(1.0, 1.0, 0.2, 1.0),
            ambient_light_color: Vec4::new(0.7, 0.7, 0.7, 1.0),
            ambient_water_activity: 0.25,
            wave_water_peak: 0.7,
            wave_water_pow: 8.0,
            wave_water_x_factor: 0.0005,
            wave_water_z_factor: 0.00115,
            wave_water_frequency: 3.0,
            keys_down: HashMap::new(),
            keys_just_pressed: HashSet::new(),
            mouse_left_down: false,
            mouse_right_down: false,
            mouse_prev_pos: (0.0, 0.0),
            movement: Movement::OnGround {
                position: Vec3::ZERO,
                velocity: Vec3::ZERO,
                look_h: 0.0,
                look_v: 0.0,
            },
            lock_mouse: true,
            flying_movement_speed: 10.0,
            on_ground_movement_speed: 40.0,
            render_distance: 8,
            system_time: SystemTime::now(),
        };
        // Make sure aspect_ratio and fov_y_radians are correct at the first draw
        app.resize_event(window_width, window_height);
        app
    }

    #[cfg(feature = "egui")]
    fn egui_ui(&mut self) {
        use egui::{color_picker::color_edit_button_rgb, TopBottomPanel};
        use egui_plot::{Line, Plot, PlotPoints};

        self.egui_mq.run(&mut self.ctx, |_ctx, egui_ctx| {
            TopBottomPanel::top("top bar").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            unimplemented!("this is ironic");
                        }
                    });
                    ui.menu_button("View", |ui| {
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
                egui::Grid::new("sliders").num_columns(2).show(ui, |ui| {
                    ui.label("render distance");
                    ui.add(
                        egui::Slider::new(&mut self.render_distance, 1..=32).clamp_to_range(true),
                    );
                    ui.end_row();

                    ui.label("flying movement speed");
                    ui.add(
                        egui::Slider::new(&mut self.flying_movement_speed, (5.0)..=100.0)
                            .clamp_to_range(true),
                    );
                    ui.end_row();

                    ui.label("on ground movement speed");
                    ui.add(
                        egui::Slider::new(&mut self.on_ground_movement_speed, (5.0)..=100.0)
                            .clamp_to_range(true),
                    );
                    ui.end_row();

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
                        egui::Slider::new(&mut self.ambient_water_activity, (0.0)..=1.0)
                            .clamp_to_range(true),
                    );
                    ui.end_row();

                    ui.label("wave water peak");
                    ui.add(
                        egui::Slider::new(&mut self.wave_water_peak, (0.0)..=1.0)
                            .clamp_to_range(true),
                    );
                    ui.end_row();

                    ui.label("wave water pow");
                    ui.add(
                        egui::Slider::new(&mut self.wave_water_pow, (0.0)..=20.0)
                            .clamp_to_range(true),
                    );
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
                        egui::Slider::new(&mut self.wave_water_frequency, (0.0)..=20.0)
                            .clamp_to_range(true),
                    );
                    ui.end_row();
                });
            });

            egui::Window::new("Performance").show(egui_ctx, |ui| {
                ui.label(format!(
                    "Average FPS: {:.0}",
                    self.fps_history.iter().sum::<f32>() / self.fps_history.len() as f32
                ));

                ui.label(format!(
                    "Terrain render queue: {}",
                    self.terrain_chunk_waiting.len()
                ));

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
                    .height(300.0)
                    .width(350.0)
                    .show(ui, |plot_ui| plot_ui.line(Line::new(fps_points)));
            });
        });

        self.egui_mq.draw(&mut self.ctx);
    }

    fn uniforms(
        &self,
        proj_matrix: Mat4,
        model_matrix: Mat4,
        camera_matrix: Mat4,
        time: f32,
    ) -> Uniforms {
        Uniforms {
            proj_matrix,
            model_matrix,
            camera_matrix,
            sun_direction: self.sun_direction,
            time,
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

    fn draw_chunk_and_around(
        &mut self,
        projection: Mat4,
        camera: Mat4,
        camera_position: IVec2,
        camera_look_h: Option<f32>,
    ) {
        let time = SystemTime::now()
            .duration_since(self.system_time)
            .unwrap()
            .as_secs_f32();
        // Which chunk is the camera located in?
        let camera_chunk = IVec2::new(
            camera_position.x / CHUNK_SIZE,
            camera_position.y / CHUNK_SIZE,
        );
        for dy in -self.render_distance..=self.render_distance {
            for dx in -self.render_distance..=self.render_distance {
                let terrain = self.terrain.lock().unwrap();
                let d_chunk = IVec2::new(dx, dy);
                let chunk = camera_chunk + d_chunk;

                // remove generated chunks from queue
                if self.terrain_chunk_waiting.contains(&chunk) && terrain.contains_key(&chunk) {
                    self.terrain_chunk_waiting.remove(&chunk);
                }

                if !terrain.contains_key(&chunk) {
                    if !self.terrain_chunk_waiting.contains(&chunk) {
                        self.terrain_chunk_gen_queue.send(chunk).unwrap();
                        self.terrain_chunk_waiting.insert(chunk);
                    }
                    continue;
                }

                if let Some(camera_look_h) = camera_look_h {
                    let (vs, vc) = camera_look_h.sin_cos();
                    let look_v = Vec2::new(vs, vc).normalize();
                    if look_v.dot(d_chunk.as_vec2()) < 0.0 {
                        continue;
                    }
                }
                let chunk_data = terrain.get(&chunk).unwrap();

                // Draw ground
                let uniforms = self.uniforms(projection, camera, camera, time);
                self.shader
                    .draw_voxels(&mut self.ctx, &chunk_data.ground, &uniforms);

                // First collect all models (in the current chunk) in an iterator
                let models = chunk_data.objects.iter().flatten();

                // Then draw each model one at a time
                for model in models {
                    let uniforms = self.uniforms(
                        projection,
                        camera * Mat4::from_rotation_translation(model.rotation, model.translation),
                        camera,
                        time,
                    );
                    self.shader
                        .draw_voxels(&mut self.ctx, &model.points, &uniforms);
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

        if self.keys_just_pressed.contains(&KeyCode::F1) {
            self.lock_mouse ^= true;
        }

        // Apply camera movement
        match &mut self.movement {
            // Trackball camera cannot move
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
                    *position += (rot_mat * movement_vector.normalize()).truncate()
                        * delta
                        * self.flying_movement_speed;
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
                    *position += (rot_mat * movement_vector.normalize()).truncate()
                        * delta
                        * self.on_ground_movement_speed;
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

        self.keys_just_pressed.clear();
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

        self.shader.prepare_draw(&mut self.ctx);

        // sky
        self.ctx.begin_default_pass(PassAction::clear_color(
            0x87 as f32 / 255.0,
            0xCE as f32 / 255.0,
            0xEB as f32 / 255.0,
            1.0,
        ));

        let projection =
            Mat4::perspective_rh_gl(self.fov_y_radians, self.aspect_ratio, 0.1, 1000.0);
        // FIXME
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

        self.draw_chunk_and_around(projection, camera, camera_position_2d, camera_look_h);

        self.ctx.end_render_pass();

        #[cfg(feature = "egui")]
        self.egui_ui();

        self.ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        #[cfg(feature = "egui")]
        self.egui_mq.mouse_motion_event(x, y);

        if self.lock_mouse {
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

        self.keys_just_pressed.insert(keycode);
        self.keys_down.insert(keycode, true);
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        #[cfg(feature = "egui")]
        self.egui_mq.key_up_event(keycode, keymods);

        self.keys_down.insert(keycode, false);
    }
}

fn terrain_gen_thread(
    biome_config: BiomeConfig,
    terrain_config: TerrainConfig,
    terrain: Arc<Mutex<Terrain>>,
    gen_queue: mpsc::Receiver<IVec2>,
) {
    for chunk in gen_queue.iter() {
        if terrain.lock().unwrap().contains_key(&chunk) {
            continue;
        }
        let data = App::generate_chunk(&biome_config, &terrain_config, chunk);
        terrain.lock().unwrap().insert(chunk, data);
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
