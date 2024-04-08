use crate::Voxel;
use noise::{NoiseFn, Perlin};

pub fn generate_flat_terrain(pos: i64, width: i64, depth: i64) -> Vec<Voxel> {
    let mut voxels = Vec::new();

    for z in 0..depth {
        for x in 0..width {
            voxels.push(Voxel::new(pos + x, pos, pos + z));
        }
    }
    return voxels;
}

pub fn generate_terrain(
    pos_x: i64,
    pos_z: i64,
    width: i64,
    height: i64,
    depth: i64,
    max_height: f64,
    perlin: Perlin,
) -> Vec<Voxel> {
    let mut voxels = Vec::new();

    for z in pos_z..depth {
        for y in 0..height {
            for x in pos_x..width {
                let sample_x: f64 = x as f64 * 0.02;
                let sample_z: f64 = z as f64 * 0.02;
                let sample = (perlin.get([sample_x, sample_z]) + 1.) / 2.;
                let height = sample * max_height;

                if y <= height as i64 {
                    voxels.push(Voxel::new(pos_x + x, height as i64, pos_z + z));
                }
            }
        }
    }

    return voxels;
}

/*
fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    /* // You may want clamping by inserting:
     * if (0.0 > w) return a0;
     * if (1.0 < w) return a1;
     */
    return (a1 - a0) * w + a0;
    /* // Use this cubic interpolation [[Smoothstep]] instead, for a smooth appearance:
     * return (a1 - a0) * (3.0 - w * 2.0) * w * w + a0;
     *
     * // Use [[Smootherstep]] for an even smoother result with a second derivative equal to zero on boundaries:
     * return (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0;
     */
}

/* Create pseudorandom direction vector
 */
fn random_gradient(ix: i32, iy: i32) -> Vec2 {
    // No precomputed gradients mean this works for any number of grid coordinates
    const W: u64 = 8 * std::mem::size_of::<u32>() as u64;
    const S: u64 = W / 2; // rotation width
    let mut a = ix as u64;
    let mut b = iy as u64;
    a = a.wrapping_mul(3284157443);
    b ^= a << S | a >> W - S;
    b = b.wrapping_mul(1911520717);
    a ^= b << S | b >> W - S;
    a = a.wrapping_mul(2048419325);
    let random: f32 = a as f32 * (3.14159265 / (std::i32::MAX as f32 / 2.0)); // in [0, 2*Pi]
    Vec2::new(random.cos(), random.sin())
}

// Computes the dot product of the distance and gradient vectors.
fn dot_grid_gradient(ix: i32, iy: i32, x: f32, y: f32) -> f32 {
    // Get gradient from integer coordinates
    let gradient = random_gradient(ix, iy);

    // Compute the distance vector
    let dx: f32 = x - ix as f32;
    let dy: f32 = y - iy as f32;

    // Compute the dot-product
    dx * gradient.x + dy * gradient.y
}

// Compute Perlin noise at coordinates x, y
pub fn perlin(x: f32, y: f32) -> f32 {
    // Determine grid cell coordinates
    let x0 = x as i32;
    let x1 = x0 + 1;
    let y0 = y as i32;
    let y1 = y0 + 1;

    // Determine interpolation weights
    // Could also use higher order polynomial/s-curve here
    let sx = x - x0 as f32;
    let sy = y - y0 as f32;

    // Interpolate between grid point gradients
    let n0 = dot_grid_gradient(x0, y0, x, y);
    let n1 = dot_grid_gradient(x1, y0, x, y);
    let ix0 = interpolate(n0, n1, sx);

    let n0 = dot_grid_gradient(x0, y1, x, y);
    let n1 = dot_grid_gradient(x1, y1, x, y);
    let ix1 = interpolate(n0, n1, sx);

    // Will return in range -1 to 1. To make it in range 0 to 1, multiply by 0.5 and add 0.5
    interpolate(ix0, ix1, sy)
}
const PERMUTATIONTABLE: [i32; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];
*/
