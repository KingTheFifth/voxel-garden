use glam::{Quat, Vec3};
use rand::Rng;

use crate::models::Model;
use crate::utils::{BLUE, BROWN, ORANGE, PURPLE, RED, YELLOW};
use crate::InstanceData;

fn generate_rand_rot() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen::<f32>() * 2. - 1.;
    let z: f32 = rng.gen::<f32>() * 2. - 1.;
    Vec3::new(x, 0.8, z).normalize()
}

fn gen_blue_flower(translation: Vec3) -> Model {
    Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new( 0.0, 0.0,  0.0), color: YELLOW, is_water: 0},
            InstanceData { position: Vec3::new( 1.0, 0.0,  0.0), color: BLUE,   is_water: 0},
            InstanceData { position: Vec3::new(-1.0, 0.0,  0.0), color: BLUE,   is_water: 0},
            InstanceData { position: Vec3::new( 0.0, 0.0,  1.0), color: BLUE,   is_water: 0},
            InstanceData { position: Vec3::new( 0.0, 0.0, -1.0), color: BLUE,   is_water: 0},
        ],

        rotation: Quat::from_rotation_arc(Vec3::Y, generate_rand_rot()),
        translation,
    }
}

fn gen_red_flower(translation: Vec3) -> Model {
    Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new( 0.0, 0.0,  0.0), color: ORANGE, is_water: 0},
            InstanceData { position: Vec3::new( 1.0, 0.0,  0.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new( 1.0, 0.0,  1.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new( 1.0, 0.0,  -1.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new( -1.0, 0.0,  1.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new( -1.0, 0.0,  -1.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new(-1.0, 0.0,  0.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new( 0.0, 0.0,  1.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new( 0.0, 0.0, -1.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new( 1.0, 1.0,  0.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new(-1.0, 1.0,  0.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new( 0.0, 1.0,  1.0), color: RED, is_water: 0},
            InstanceData { position: Vec3::new( 0.0, 1.0, -1.0), color: RED, is_water: 0},
        ],

        rotation: Quat::from_rotation_arc(Vec3::Y, generate_rand_rot()),
        translation,
    }
}

fn gen_purple_flower(translation: Vec3) -> Model {
    Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new( 0.0, 0.0,  0.0), color: YELLOW, is_water: 0},
            InstanceData { position: Vec3::new( 1.0, 0.0,  0.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new(-1.0, 0.0,  0.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new( 0.0, 0.0,  1.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new( 0.0, 0.0, -1.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new( 1.0, 1.0,  -1.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new( 1.0, 1.0,  1.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new( -1.0, 1.0,  -1.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new( -1.0, 1.0,  1.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new( 2.0, 1.0,  0.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new(-2.0, 1.0,  0.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new( 0.0, 1.0,  2.0), color: PURPLE, is_water: 0},
            InstanceData { position: Vec3::new( 0.0, 1.0, -2.0), color: PURPLE, is_water: 0},
        ],

        rotation: Quat::from_rotation_arc(Vec3::Y, generate_rand_rot()),
        translation,
    }
}

pub fn proc_gen_flower(_seed: u64, translation: Vec3) -> Vec<Model> {
    let mut rng = rand::thread_rng();
    let stem_length = rand::thread_rng().gen_range(3..9);
    let mut flower_stem: Vec<InstanceData> = vec![];
    for y in 0..stem_length {
        flower_stem.push(InstanceData {
            position: Vec3::new(0.0, y as f32, 0.0),
            color: BROWN,
            is_water: 0,
        })
    }
    let stem = Model {
        points: flower_stem,
        rotation: Quat::IDENTITY,
        translation,
    };
    let flower: Model;
    let rand: f32 = rng.gen::<f32>();
    let mut flower_translation = translation;
    flower_translation.y += stem_length as f32 - 0.2;

    if rand < 0.33 {
        flower = gen_red_flower(flower_translation);
    } else if rand < 0.67 {
        flower = gen_blue_flower(flower_translation);
    } else {
        flower = gen_purple_flower(flower_translation);
    }

    vec![stem, flower]
}
