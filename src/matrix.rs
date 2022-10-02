#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn add(self: &Self, v2: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + v2.x,
            y: self.y + v2.y,
            z: self.z + v2.z,
        }
    }
    pub fn sub(self: &Self, v2: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - v2.x,
            y: self.y - v2.y,
            z: self.z - v2.z,
        }
    }

    pub fn mul(self: &Self, v2: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * v2.x,
            y: self.y * v2.y,
            z: self.z * v2.z,
        }
    }

    pub fn scale(self: &Self, amount: f64) -> Vec3 {
        Vec3 {
            x: self.x * amount,
            y: self.y * amount,
            z: self.z * amount,
        }
    }

    pub fn length(self: &Self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // pub fn length_sq(self: &Self) -> f64 {
    //     self.x * self.x + self.y * self.y + self.z * self.z
    // }

    pub fn normalize(self: &Self) -> Vec3 {
        let l = self.length();
        Vec3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    pub fn dot(self: &Self, v2: Vec3) -> f64 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    pub fn cross(self: &Self, v2: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v2.z - self.z * v2.y,
            y: self.z * v2.x - self.x * v2.z,
            z: self.x * v2.y - self.y * v2.x,
        }
    }

    pub fn xz(self: &Self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: 0.0,
            z: self.z,
        }
    }
}

// ------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
pub struct Mat3x3 {
    pub xx: f64,
    pub xy: f64,
    pub xz: f64,
    pub yx: f64,
    pub yy: f64,
    pub yz: f64,
    pub zx: f64,
    pub zy: f64,
    pub zz: f64,
}

impl Mat3x3 {
    pub fn mul_v(self: &Self, v: Vec3) -> Vec3 {
        return Vec3 {
            x: self.xx * v.x + self.xy * v.y + self.xz * v.z,
            y: self.yx * v.x + self.yy * v.y + self.yz * v.z,
            z: self.zx * v.x + self.zy * v.y + self.zz * v.z,
        };
    }

    // RightMulV is 1x3 = 1x3 3x3
    // It's can also be thought of as transpose and multiply.
    pub fn right_mul_v(self: &Self, v: Vec3) -> Vec3 {
        return Vec3 {
            x: self.xx * v.x + self.yx * v.y + self.zx * v.z,
            y: self.xy * v.x + self.yy * v.y + self.zy * v.z,
            z: self.xz * v.x + self.yz * v.y + self.zz * v.z,
        };
    }

    pub fn mul(self: &Self, m2: Mat3x3) -> Mat3x3 {
        return Mat3x3 {
            xx: self.xx * m2.xx + self.xy * m2.yx + self.xz * m2.zx,
            xy: self.xx * m2.xy + self.xy * m2.yy + self.xz * m2.zy,
            xz: self.xx * m2.xz + self.xy * m2.yz + self.xz * m2.zz,

            yx: self.yx * m2.xx + self.yy * m2.yx + self.yz * m2.zx,
            yy: self.yx * m2.xy + self.yy * m2.yy + self.yz * m2.zy,
            yz: self.yx * m2.xz + self.yy * m2.yz + self.yz * m2.zz,

            zx: self.zx * m2.xx + self.zy * m2.yx + self.zz * m2.zx,
            zy: self.zx * m2.xy + self.zy * m2.yy + self.zz * m2.zy,
            zz: self.zx * m2.xz + self.zy * m2.yz + self.zz * m2.zz,
        };
    }

    pub fn rot(axis: Vec3, angle: f64) -> Mat3x3 {
        return Mat3x3 {
            xx: angle.cos() + axis.x * axis.x * (1.0 - angle.cos()),
            xy: axis.x * axis.y * (1.0 - angle.cos()) - axis.z * angle.sin(),
            xz: axis.x * axis.z * (1.0 - angle.cos()) + axis.y * angle.sin(),

            yx: axis.y * axis.x * (1.0 - angle.cos()) + axis.z * angle.sin(),
            yy: angle.cos() + axis.y * axis.y * (1.0 - angle.cos()),
            yz: axis.y * axis.z * (1.0 - angle.cos()) - axis.x * angle.sin(),

            zx: axis.z * axis.x * (1.0 - angle.cos()) - axis.y * angle.sin(),
            zy: axis.z * axis.y * (1.0 - angle.cos()) + axis.x * angle.sin(),
            zz: angle.cos() + axis.z * axis.z * (1.0 - angle.cos()),
        };
    }
}
