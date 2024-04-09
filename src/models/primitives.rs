use crate::Voxel;

struct BresenhamPoints {
    start: Voxel,
    end: Voxel,

    ey: i64,
    ez: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl BresenhamPoints {
    fn new(start: Voxel, end: Voxel) -> Self {
        let (x1, y1, z1) = (start.x, start.y, start.z);
        let (x2, y2, z2) = (end.x, end.y, end.z);

        let my = 2 * (y2 - y1);
        let mz = 2 * (z2 - z1);

        Self {
            start,
            end,
            ey: my - (x2 - x1),
            ez: mz - (x2 - x1),
            x: x1,
            y: y1,
            z: z1,
        }
    }
}

impl Iterator for BresenhamPoints {
    type Item = Voxel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x <= self.end.x {
            let vox = Voxel::new(self.x, self.y, self.z);
            self.x += 1;
            self.ey += 2 * (self.end.y - self.start.y);
            self.ez += 2 * (self.end.z - self.start.z);
            if self.ey >= 0 {
                self.y += 1;
                self.ey -= 2 * (self.end.x - self.start.x);
            }
            if self.ez >= 0 {
                self.z += 1;
                self.ez -= 2 * (self.end.x - self.start.x);
            }
            Some(vox)
        } else {
            None
        }
    }
}

/// Bresenham's line drawing algorithm extended into 3D.
pub fn bresenham(start: Voxel, end: Voxel) -> Vec<Voxel> {
    BresenhamPoints::new(start, end).collect()
}

/// A line with 3x3x3-crosses at every point.
pub fn line_cross(start: Voxel, end: Voxel) -> Vec<Voxel> {
    BresenhamPoints::new(start, end)
        .flat_map(|Voxel { x, y, z }| {
            [
                Voxel::new(x, y, z),
                Voxel::new(x + 1, y, z),
                Voxel::new(x - 1, y, z),
                Voxel::new(x, y + 1, z),
                Voxel::new(x, y - 1, z),
                Voxel::new(x, y, z + 1),
                Voxel::new(x, y, z - 1),
            ]
        })
        .collect()
}
