use crate::matrix::Mat3x3;
use crate::matrix::Vec3;

fn torus_sd(p: Vec3, tx: f64, ty: f64) -> f64 {
    let q = Vec3 {
        x: p.xz().length() - tx,
        y: p.y,
        z: 0.0,
    };
    q.length() - ty
}
fn torus_sdg(p: Vec3, ra: f64, rb: f64) -> (f64, Vec3) {
    let h = p.xz().length();
    (
        Vec3 {
            x: h - ra,
            y: p.y,
            z: 0.0,
        }
        .length()
            - rb,
        p.mul(Vec3 {
            x: h - ra,
            y: h,
            z: h - ra,
        })
        .normalize(),
    )
}

pub fn scene_sd(p: Vec3, m: Mat3x3) -> f64 {
    let p1 = m.mul_v(p);
    let d = torus_sd(p1, 2.0, 0.75);
    return d;
}

pub fn normals_sd(p: Vec3, m: Mat3x3) -> Vec3 {
    let p1 = m.mul_v(p);
    let (_, n) = torus_sdg(p1, 2.0, 0.75);
    let n1 = m.right_mul_v(n);
    return n1;
    // dx1 := Scene_sd(p.Add(Vec3{EPS, 0, 0}), m)
    // dx2 := Scene_sd(p.Add(Vec3{-EPS, 0, 0}), m)

    // dy1 := Scene_sd(p.Add(Vec3{0, EPS, 0}), m)
    // dy2 := Scene_sd(p.Add(Vec3{0, -EPS, 0}), m)

    // dz1 := Scene_sd(p.Add(Vec3{0, 0, EPS}), m)
    // dz2 := Scene_sd(p.Add(Vec3{0, 0, -EPS}), m)

    // return Vec3{
    // 	X: dx2 - dx1/2*EPS,
    // 	Y: dy2 - dy1/2*EPS,
    // 	Z: dz2 - dz1/2*EPS,
    // }.Normalize()
}

pub fn get_rot_mat_at_time(t: f64) -> Mat3x3 {
    let r1 = Mat3x3::rot(
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        -0.1 * 0.7 * t,
    );
    let r2 = Mat3x3::rot(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        0.1 * std::f64::consts::PI * t,
    );
    return r2.mul(r1);
}
