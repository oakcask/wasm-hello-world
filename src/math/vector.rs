use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{vec3, vec4};

#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<Vector3> for Vector4 {
    fn from(value: Vector3) -> Self {
        match value {
            vec3!(x, y, z) => vec4!(x, y, z, 1.0),
        }
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(value: (f32, f32, f32)) -> Self {
        match value {
            (x, y, z) => vec3!(x, y, z),
        }
    }
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        match value {
            (x, y, z, w) => vec4!(x, y, z, w),
        }
    }
}

impl Vector3 {
    pub fn norm(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vector3 {
        self / self.norm()
    }

    pub fn dot(self, rhs: Vector3) -> f32 {
        match (self, rhs) {
            (vec3!(ax, ay, az), vec3!(bx, by, bz)) => ax * bx + ay * by + az * bz,
        }
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        match (self, rhs) {
            (vec3!(ax, ay, az), vec3!(bx, by, bz)) => {
                vec3!(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
            }
        }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        match (self, rhs) {
            (vec3!(ax, ay, az), vec3!(bx, by, bz)) => {
                vec3!(ax + bx, ay + by, az + bz)
            }
        }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        match (self, rhs) {
            (vec3!(ax, ay, az), vec3!(bx, by, bz)) => {
                vec3!(ax - bx, ay - by, az - bz)
            }
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            vec3!(x, y, z) => vec3!(x * rhs, y * rhs, z * rhs),
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Self::Output {
        match self {
            vec3!(x, y, z) => vec3!(x / rhs, y / rhs, z / rhs),
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self::Output {
        match self {
            vec3!(x, y, z) => vec3!(-x, -y, -z),
        }
    }
}

mod macros {
    #[macro_export]
macro_rules! vec2 {
    ($x:ident, $y:ident) => {
        $crate::math::Vector2 { x: $x, y: $y }
    };
    ($x:expr, $y:expr) => {
        $crate::math::Vector2 { x: $x, y: $y }
    };
}

#[macro_export]
macro_rules! vec3 {
    ($x:ident, $y:ident, $z:ident) => {
               $crate::math:: Vector3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
    ($x:expr, $y:expr, $z:expr) => {
         $crate::math::       Vector3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}

#[macro_export]
macro_rules! vec4 {
    ($x:ident, $y:ident, $z:ident, $w:ident) => {
         $crate::math::       Vector4 {
            x: $x,
            y: $y,
            z: $z,
            w: $w,
        }
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
         $crate::math::       Vector4 {
            x: $x,
            y: $y,
            z: $z,
            w: $w,
        }
    };
}
}
