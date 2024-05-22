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
            InstanceData { position: Vec3::new( 0.0, 0.0,  0.0), color: YELLOW},
            InstanceData { position: Vec3::new( 1.0, 0.0,  0.0), color: BLUE},
            InstanceData { position: Vec3::new(-1.0, 0.0,  0.0), color: BLUE},
            InstanceData { position: Vec3::new( 0.0, 0.0,  1.0), color: BLUE},
            InstanceData { position: Vec3::new( 0.0, 0.0, -1.0), color: BLUE},
        ],

        rotation: Quat::from_rotation_arc(Vec3::Y, generate_rand_rot()),
        translation,
    }
}

fn gen_red_flower(translation: Vec3) -> Model {
    Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new( 0.0, 0.0,  0.0), color: ORANGE},
            InstanceData { position: Vec3::new( 1.0, 0.0,  0.0), color: RED},
            InstanceData { position: Vec3::new( 1.0, 0.0,  1.0), color: RED},
            InstanceData { position: Vec3::new( 1.0, 0.0,  -1.0), color: RED},
            InstanceData { position: Vec3::new( -1.0, 0.0,  1.0), color: RED},
            InstanceData { position: Vec3::new( -1.0, 0.0,  -1.0), color: RED},
            InstanceData { position: Vec3::new(-1.0, 0.0,  0.0), color: RED},
            InstanceData { position: Vec3::new( 0.0, 0.0,  1.0), color: RED},
            InstanceData { position: Vec3::new( 0.0, 0.0, -1.0), color: RED},
            InstanceData { position: Vec3::new( 1.0, 1.0,  0.0), color: RED},
            InstanceData { position: Vec3::new(-1.0, 1.0,  0.0), color: RED},
            InstanceData { position: Vec3::new( 0.0, 1.0,  1.0), color: RED},
            InstanceData { position: Vec3::new( 0.0, 1.0, -1.0), color: RED},
        ],

        rotation: Quat::from_rotation_arc(Vec3::Y, generate_rand_rot()),
        translation,
    }
}

fn gen_purple_flower(translation: Vec3) -> Model {
    Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new( 0.0, 0.0,  0.0), color: YELLOW},
            InstanceData { position: Vec3::new( 1.0, 0.0,  0.0), color: PURPLE},
            InstanceData { position: Vec3::new(-1.0, 0.0,  0.0), color: PURPLE},
            InstanceData { position: Vec3::new( 0.0, 0.0,  1.0), color: PURPLE},
            InstanceData { position: Vec3::new( 0.0, 0.0, -1.0), color: PURPLE},
            InstanceData { position: Vec3::new( 1.0, 1.0,  -1.0), color: PURPLE},
            InstanceData { position: Vec3::new( 1.0, 1.0,  1.0), color: PURPLE},
            InstanceData { position: Vec3::new( -1.0, 1.0,  -1.0), color: PURPLE},
            InstanceData { position: Vec3::new( -1.0, 1.0,  1.0), color: PURPLE},
            InstanceData { position: Vec3::new( 2.0, 1.0,  0.0), color: PURPLE},
            InstanceData { position: Vec3::new(-2.0, 1.0,  0.0), color: PURPLE},
            InstanceData { position: Vec3::new( 0.0, 1.0,  2.0), color: PURPLE},
            InstanceData { position: Vec3::new( 0.0, 1.0, -2.0), color: PURPLE},
        ],

        rotation: Quat::from_rotation_arc(Vec3::Y, generate_rand_rot()),
        translation,
    }
}

pub fn flower(_seed: u64, translation: Vec3) -> Vec<Model> {
    let stem = Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new(0., 0.,  0.), color: BROWN },
            InstanceData { position: Vec3::new(0., 1.0, 0.), color: BROWN },
            InstanceData { position: Vec3::new(0., 2.0, 0.), color: BROWN },
            InstanceData { position: Vec3::new(0., 3.0, 0.), color: BROWN },
        ],
        rotation: Quat::IDENTITY,
        translation,
    };
    let flower = Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new(0.,   0.,  0.), color: BROWN },
            InstanceData { position: Vec3::new(1.0,  0.,  0.), color: YELLOW },
            InstanceData { position: Vec3::new(-1.0, 0.,  0.), color: YELLOW },
            InstanceData { position: Vec3::new(0.,   0.,  1.0), color: YELLOW },
            InstanceData { position: Vec3::new(0.,   0., -1.0), color: YELLOW },
        ],
        rotation: Quat::from_rotation_arc(Vec3::Y, Vec3::new(1., 1., 0.).normalize()),
        translation: Vec3::new(0.8, 3.8, 0.) + translation,
    };
    vec![stem, flower]
}

pub fn flower_blue(_seed: u64, translation: Vec3) -> Vec<Model> {
    let stem = Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new(0.0, 0.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 1.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 2.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 3.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 4.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 5.0, 0.0), color: BROWN },
        ],
        rotation: Quat::IDENTITY,
        translation,
    };

    let mut flower_translation = translation;
    flower_translation.y += 5.8;
    let flower = gen_blue_flower(flower_translation);
    vec![stem, flower]
}

pub fn flower_red(_seed: u64, translation: Vec3) -> Vec<Model> {
    let stem = Model {
        #[rustfmt::skip]
        points: vec![
            InstanceData { position: Vec3::new(0.0, 0.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 1.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 2.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 3.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 4.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 5.0, 0.0), color: BROWN },
            InstanceData { position: Vec3::new(0.0, 6.0, 0.0), color: BROWN },
        ],
        rotation: Quat::IDENTITY,
        translation,
    };
    let mut flower_translation = translation;
    flower_translation.y += 6.8;
    let flower = gen_red_flower(flower_translation);
    vec![stem, flower]
}

pub fn proc_gen_flower(_seed: u64, translation: Vec3) -> Vec<Model> {
    let mut rng = rand::thread_rng();
    let stem_length = rand::thread_rng().gen_range(3..9);
    let mut flower_stem: Vec<InstanceData> = vec![];
    for y in 0..stem_length {
        flower_stem.push(InstanceData {
            position: Vec3::new(0.0, y as f32, 0.0),
            color: BROWN,
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
