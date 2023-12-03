#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "macros")]
pub mod macros;


// Traits implementation

use std::ops::{Add,Sub,Mul};


impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x : rhs * self.x,
            y : rhs * self.y
        }
    }
}

impl Mul for Vector2 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }

}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: rhs * self.x ,
            y: rhs * self.y ,
            z: rhs * self.z ,
        }
    }
}

impl Mul for Vector3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

}

impl Add for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Self) -> Self::Output {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector4 {
            x: rhs * self.x ,
            y: rhs * self.y ,
            z: rhs * self.z ,
            w: rhs * self.w ,
        }
    }
}

impl Mul for Vector4 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        Matrix {
            m0: self.m0 + rhs.m0,
            m1: self.m1 + rhs.m1,
            m2: self.m2 + rhs.m2,
            m3: self.m3 + rhs.m3,
            m4: self.m4 + rhs.m4,
            m5: self.m5 + rhs.m5,
            m6: self.m6 + rhs.m6,
            m7: self.m7 + rhs.m7,
            m8: self.m8 + rhs.m8,
            m9: self.m9 + rhs.m9,
            m10: self.m10 + rhs.m10,
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m13: self.m13 + rhs.m13,
            m14: self.m14 + rhs.m14,
            m15: self.m15 + rhs.m15,
        }
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix {
            m0: self.m0 - rhs.m0,
            m1: self.m1 - rhs.m1,
            m2: self.m2 - rhs.m2,
            m3: self.m3 - rhs.m3,
            m4: self.m4 - rhs.m4,
            m5: self.m5 - rhs.m5,
            m6: self.m6 - rhs.m6,
            m7: self.m7 - rhs.m7,
            m8: self.m8 - rhs.m8,
            m9: self.m9 - rhs.m9,
            m10: self.m10 - rhs.m10,
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m13: self.m13 - rhs.m13,
            m14: self.m14 - rhs.m14,
            m15: self.m15 - rhs.m15,
        }
    }
}

impl Mul<f32> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f32) -> Self::Output {
        Matrix {
            m0: rhs * self.m0 ,
            m1: rhs * self.m1 ,
            m2: rhs * self.m2 ,
            m3: rhs * self.m3 ,
            m4: rhs * self.m4 ,
            m5: rhs * self.m5 ,
            m6: rhs * self.m6 ,
            m7: rhs * self.m7 ,
            m8: rhs * self.m8 ,
            m9: rhs * self.m9 ,
            m10: rhs * self.m10 ,
            m11: rhs * self.m11 ,
            m12: rhs * self.m12 ,
            m13: rhs * self.m13 ,
            m14: rhs * self.m14 ,
            m15: rhs * self.m15 ,
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix {
            m0:   self.m0 * rhs.m0 + self.m4 * rhs.m1 + self.m8 * rhs.m2 + self.m12 * rhs.m3 ,
            m1:   self.m1 * rhs.m0 + self.m5 * rhs.m1 + self.m9 * rhs.m2 + self.m13 * rhs.m3 ,
            m2:   self.m2 * rhs.m0 + self.m6 * rhs.m1 + self.m10 * rhs.m2 + self.m14 * rhs.m3 ,
            m3:   self.m3 * rhs.m0 + self.m7 * rhs.m1 + self.m11 * rhs.m2 + self.m15 * rhs.m3 ,
            m4:   self.m0 * rhs.m4 + self.m4 * rhs.m5 + self.m8 * rhs.m6 + self.m12 * rhs.m7 ,
            m5:   self.m1 * rhs.m4 + self.m5 * rhs.m5 + self.m9 * rhs.m6 + self.m13 * rhs.m7 ,
            m6:   self.m2 * rhs.m4 + self.m6 * rhs.m5 + self.m10 * rhs.m6 + self.m14 * rhs.m7 ,
            m7:   self.m3 * rhs.m4 + self.m7 * rhs.m5 + self.m11 * rhs.m6 + self.m15 * rhs.m7 ,
            m8:   self.m0 * rhs.m8 + self.m4 * rhs.m9 + self.m8 * rhs.m10 + self.m12 * rhs.m11 ,
            m9:   self.m1 * rhs.m0 + self.m5 * rhs.m9 + self.m9 * rhs.m10 + self.m13 * rhs.m11 ,
            m10:  self.m2 * rhs.m0 + self.m6 * rhs.m9 + self.m10 * rhs.m10 + self.m14 * rhs.m11 ,
            m11:  self.m3 * rhs.m0 + self.m7 * rhs.m9 + self.m11 * rhs.m10 + self.m15 * rhs.m11 ,
            m12:  self.m0 * rhs.m12 + self.m4 * rhs.m13 + self.m8 * rhs.m14 + self.m12 * rhs.m15 ,
            m13:  self.m1 * rhs.m0 + self.m5 * rhs.m13 + self.m9 * rhs.m14 + self.m13 * rhs.m15 ,
            m14:  self.m2 * rhs.m0 + self.m6 * rhs.m13 + self.m10 * rhs.m14 + self.m14 * rhs.m15 ,
            m15:  self.m3 * rhs.m0 + self.m7 * rhs.m13 + self.m11 * rhs.m14 + self.m15 * rhs.m15 ,
        }
    }
}
