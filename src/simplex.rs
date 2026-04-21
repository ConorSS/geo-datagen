

use std::ops::Add;

use glam::{IVec2, IVec3, Vec2, Vec2Swizzles, Vec3};

// Adapted from academic code (Stefan Gustavson).
// src: https://www.itn.liu.se/~stegu76/aqsis/aqsis-newnoise/simplexnoise1234.cpp

// Permutation table for simplex noise.
const PERM: [u8; 512] = [
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
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

// Directional gradient generator for simplex2.
fn grad2(hash: u8, point: &Vec2) -> f32 {
    // I believe this algorithm is trying to produce 8 directional gradients from the 8 discrete cardinals derived from the hash value.
    let h = hash & 7;
    let im = if h < 4 { point.xy() } else { point.yx() };
    (if h & 1 > 0 { -im.x } else { im.x })
        + (if h & 2 > 0 { -2f32 * im.y } else { 2f32 * im.y })
}

// Directional gradient generator for simplex3.
fn grad3(hash: u8, point: &Vec3) -> f32 {
    let h = hash & 7;
    let im = Vec2 {
        x: if h < 8 { point.x } else { point.y },
        y: if h < 4 {
            point.y
        } else if h == 12 || h == 14 {
            point.x
        } else {
            point.z
        }, // According to example, this branch fixes some repeat values.
    };
    (if h & 1 != 0 { -im.x } else { im.x }) + (if h & 2 != 0 { -im.y } else { im.y })
}

#[allow(clippy::excessive_precision)]
pub fn simplex2(point: &Vec2) -> f32 {
    // the example is very c-like, expect very linear code

    const F2: f32 = 0.366025403; // precalc 0.5*(sqrt(3.0)-1.0)
    const G2: f32 = 0.211324865; // precalc (3.0-Math.sqrt(3.0))/6.0

    // skew calculation for cell coords
    let xys = point.add(point.element_sum() * F2);
    let ij = IVec2 {
        x: xys.x.floor() as i32,
        y: xys.y.floor() as i32,
    };

    let t = (ij.element_sum() as f32) * G2;
    // Collect distance from origin to source point
    let xy0 = point
        - Vec2 {
            // The origin, in local space
            x: ij.x as f32 - t,
            y: ij.y as f32 - t,
        };

    // In 2D, the simplex is present as an equilateral triangle,
    // which may be present in one of two states dependent on the distance's ordering.
    let ij1 = if xy0.x < xy0.y {
        Vec2 { x: 1.0, y: 0.0 }
    } else {
        Vec2 { x: 0.0, y: 1.0 }
    };

    let xy1 = xy0 - ij1 + G2;
    let xy2 = xy0 - 1.0 + 2.0 * G2;

    // Integer indeces clamped to 256.
    let ii = (ij.x & 0xff) as usize;
    let jj = (ij.y & 0xff) as usize;

    // Collect values for each of the three corners.
    let t0 = 0.5 - xy0.x * xy0.x - xy0.y * xy0.y;
    let n0 = if t0 < 0.0 {
        0.0
    } else {
        t0 * t0 * t0 * t0 * grad2(PERM[ii + PERM[jj] as usize], &xy0)
    };

    let t1 = 0.5 - xy1.x * xy1.x - xy1.y * xy1.y;
    let n1 = if t1 < 0.0 {
        0.0
    } else {
        t1 * t1
            * t1
            * t1
            * grad2(
                PERM[ii + ij1.x as usize + PERM[jj + ij1.y as usize] as usize],
                &xy1,
            )
    };

    let t2 = 0.5 - xy2.x * xy2.x - xy2.y * xy2.y;
    let n2 = if t2 < 0.0 {
        0.0
    } else {
        t2 * t2 * t2 * t2 * grad2(PERM[ii + 1 + PERM[jj + 1] as usize], &xy2)
    };

    // Combine for rational result.
    (n0 + n1 + n2) * 30.0
}

#[allow(clippy::excessive_precision)]
pub fn simplex3(point: &Vec3) -> f32 {
    // The skew factors are simpler here.
    const F3: f32 = 0.333333333;
    const G3: f32 = 0.166666667;

    // skew calculation for cell coords
    let xyzs = point.add(point.element_sum() * F3);
    let ijk = IVec3 {
        x: xyzs.x.floor() as i32,
        y: xyzs.y.floor() as i32,
        z: xyzs.z.floor() as i32,
    };

    let t = (ijk.element_sum() as f32) * G3;
    // Collect distance from origin to source point
    let xyz0 = point
        - Vec3 {
            // The origin, in local space
            x: ijk.x as f32 - t,
            y: ijk.y as f32 - t,
            z: ijk.z as f32 - t,
        };

    // In 3D, the shape is now a "slightly irregular tetrahedron".
    // This shape may be in 6 configurations depending on ordering of indices.
    let (ijk1, ijk2) = match (xyz0.x >= xyz0.y, xyz0.y >= xyz0.z, xyz0.x >= xyz0.z) {
        (true, true, _) => (Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0)),
        (true, false, true) => (Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0)),
        (true, false, false) => (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 1.0)),
        (false, false, _) => (Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 1.0)),
        (false, true, false) => (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 1.0)),
        (false, true, true) => (Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0)),
    };

    let xyz1 = xyz0 - ijk1 + G3;
    let xyz2 = xyz0 - ijk2 + 2.0 * G3;
    let xyz3 = xyz0 - 1.0 + 3.0 * G3;

    // Integer indeces clamped to 256.
    let ii = (ijk.x & 0xff) as usize;
    let jj = (ijk.y & 0xff) as usize;
    let kk = (ijk.z & 0xff) as usize;

    // Collect values for the four vertices.
    let t0 = 0.6 - xyz0.x * xyz0.x - xyz0.y * xyz0.y - xyz0.z * xyz0.z;
    let n0 = if t0 < 0.0 {
        0.0
    } else {
        t0 * t0 * t0 * grad3(PERM[ii + PERM[jj + PERM[kk] as usize] as usize], &xyz0)
    };

    let t1 = 0.6 - xyz1.x * xyz1.x - xyz1.y * xyz1.y - xyz1.z * xyz1.z;
    let n1 = if t1 < 0.0 {
        0.0
    } else {
        t1 * t1
            * t1
            * grad3(
                PERM[ii
                    + ijk1.x as usize
                    + PERM[jj + ijk1.y as usize + PERM[kk + ijk1.z as usize] as usize] as usize],
                &xyz1,
            )
    };

    let t2 = 0.6 - xyz2.x * xyz2.x - xyz2.y * xyz2.y - xyz2.z * xyz2.z;
    let n2 = if t2 < 0.0 {
        0.0
    } else {
        t2 * t2
            * t2
            * grad3(
                PERM[ii
                    + ijk2.x as usize
                    + PERM[jj + ijk2.y as usize + PERM[kk + ijk2.z as usize] as usize] as usize],
                &xyz2,
            )
    };

    let t3 = 0.6 - xyz3.x * xyz3.x - xyz3.y * xyz3.y - xyz3.z * xyz3.z;
    let n3 = if t3 < 0.0 {
        0.0
    } else {
        t3 * t3
            * t3
            * grad3(
                PERM[ii + 1 + PERM[jj + 1 + PERM[kk + 1] as usize] as usize],
                &xyz3,
            )
    };

    // Combine for rational result.
    (n0 + n1 + n2 + n3) * 30.0
}
