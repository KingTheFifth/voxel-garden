use glam::{Mat3, Mat4, Quat, Vec3, Vec4};

use crate::utils::arb_rotate;

pub enum Movement {
    Trackball {
        down_pos: (f32, f32),
        matrix: Mat4,
    },
    Flying {
        position: Vec3,
        look_h: f32,
        look_v: f32,
    },
}

impl Movement {
    pub fn camera_matrix(&self) -> Mat4 {
        match self {
            Movement::Trackball { down_pos, matrix } => {
                let scale = 5.0;
                Mat4::look_at_rh(
                    scale * Vec3::new(0.0, 0.0, 5.0),
                    scale * Vec3::ZERO,
                    Vec3::Y,
                )
            }
            Movement::Flying {
                position,
                look_h,
                look_v,
            } => Mat4::look_at_rh(
                *position,
                *position
                    + (Mat4::from_quat(
                        (Quat::from_rotation_y(*look_h) * Quat::from_rotation_x(*look_v))
                            .normalize(),
                    ) * Vec4::Z)
                        .truncate(),
                Vec3::Y,
            ),
        }
    }
}

pub fn trackball_control(
    camera_matrix: Mat4,
    screen_pos: (f32, f32),
    prev_pos: (f32, f32),
) -> Mat4 {
    let axis = Vec3::new(screen_pos.1 - prev_pos.1, prev_pos.0 - screen_pos.0, 0.0);
    let axis = Mat3::from_mat4(camera_matrix).inverse() * axis;
    arb_rotate(axis, axis.length() / 50.0)
}
