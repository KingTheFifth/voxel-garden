use crate::Voxel;

pub fn line(start: Voxel, end: Voxel) -> Vec<Voxel> {
    let mut voxels = Vec::new();

    let (x1, y1, z1) = (start.x, start.y, start.z);
    let (x2, y2, z2) = (end.x, end.y, end.z);
    let my = 2 * (y2 - y1);
    let mut ey = my - (x2 - x1);
    let mz = 2 * (z2 - z1);
    let mut ez = mz - (x2 - x1);
    let mut y = y1;
    let mut z = z1;
    for x in x1..=x2 {
        // add all neighbours
        // for dx in -1..=1i64 {
        //     for dy in -1..=1i64 {
        //         for dz in -1..=1i64 {
        //             if (dx.abs() + dy.abs() + dz.abs() != 3) {
        //                 voxels.push(Voxel::new(x + dx, y + dy, z + dz));
        //             }
        //         }
        //     }
        // }
        voxels.push(Voxel::new(x, y, z));
        ey += my;
        ez += mz;
        if ey >= 0 {
            y += 1;
            ey -= 2 * (x2 - x1);
        }
        if ez >= 0 {
            z += 1;
            ez -= 2 * (x2 - x1);
        }
    }

    voxels
}
