use std::{mem::size_of, ops::Mul};

use crate::vec3;

use super::Vector3;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Matrix4 {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m24: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m34: f32,
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
    pub m44: f32,
}

#[macro_export]
macro_rules! mat4 {
    (
        $a11:ident, $a12:ident, $a13:ident, $a14:ident,
        $a21:ident, $a22:ident, $a23:ident, $a24:ident,
        $a31:ident, $a32:ident, $a33:ident, $a34:ident,
        $a41:ident, $a42:ident, $a43:ident, $a44:ident
    ) => (Matrix4 {
        m11: $a11, m12: $a12, m13: $a13, m14: $a14,
        m21: $a21, m22: $a22, m23: $a23, m24: $a24,
        m31: $a31, m32: $a32, m33: $a33, m34: $a34,
        m41: $a41, m42: $a42, m43: $a43, m44: $a44
    });
    (
        $a11:expr, $a12:expr, $a13:expr, $a14:expr,
        $a21:expr, $a22:expr, $a23:expr, $a24:expr,
        $a31:expr, $a32:expr, $a33:expr, $a34:expr,
        $a41:expr, $a42:expr, $a43:expr, $a44:expr
    ) => (Matrix4 {
        m11: $a11, m12: $a12, m13: $a13, m14: $a14,
        m21: $a21, m22: $a22, m23: $a23, m24: $a24,
        m31: $a31, m32: $a32, m33: $a33, m34: $a34,
        m41: $a41, m42: $a42, m43: $a43, m44: $a44
    })
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Mul for Matrix4 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    match (self, rhs) {
       (mat4!(
          a11, a12, a13, a14,
          a21, a22, a23, a24,
          a31, a32, a33, a34,
          a41, a42, a43, a44
       ), mat4!(
          b11, b12, b13, b14,
          b21, b22, b23, b24,
          b31, b32, b33, b34,
          b41, b42, b43, b44)
      ) =>
        mat4!(
          a11*b11 + a12*b21 + a13*b31 + a14*b41,
          a11*b12 + a12*b22 + a13*b32 + a14*b42,
          a11*b13 + a12*b23 + a13*b33 + a14*b43,
          a11*b14 + a12*b24 + a13*b34 + a14*b44,
          a21*b11 + a22*b21 + a23*b31 + a24*b41,
          a21*b12 + a22*b22 + a23*b32 + a24*b42,
          a21*b13 + a22*b23 + a23*b33 + a24*b43,
          a21*b14 + a22*b24 + a23*b34 + a24*b44,
          a31*b11 + a32*b21 + a33*b31 + a34*b41,
          a31*b12 + a32*b22 + a33*b32 + a34*b42,
          a31*b13 + a32*b23 + a33*b33 + a34*b43,
          a31*b14 + a32*b24 + a33*b34 + a34*b44,
          a41*b11 + a42*b21 + a43*b31 + a44*b41,
          a41*b12 + a42*b22 + a43*b32 + a44*b42,
          a41*b13 + a42*b23 + a43*b33 + a44*b43,
          a41*b14 + a42*b24 + a43*b34 + a44*b44            
        )
    }
  }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
      match (self, rhs) {
        (mat4!(
          a11, a12, a13, a14,
          a21, a22, a23, a24,
          a31, a32, a33, a34,
          a41, a42, a43, a44
        ), vec3!(x, y, z)) => {
          let w = 1.0/(a41*x + a42*y + a43*z + a44);
          vec3!(
            (a11*x + a12*y + a13*z + a14) * w,
            (a21*x + a22*y + a23*z + a24) * w,
            (a31*x + a32*y + a33*z + a34) * w)
        }
      }
    }
}

impl AsRef<[f32]> for Matrix4 {
    fn as_ref(&self) -> &[f32] {
        unsafe {
          std::slice::from_raw_parts(&self.m11 as *const f32, size_of::<Self>() / size_of::<f32>())
        }
    }
}

impl Matrix4 {
    #[allow(dead_code)]
    pub fn transpose(self) -> Matrix4 {
        match self {
            mat4!(
                a11, a12, a13, a14,
                a21, a22, a23, a24,
                a31, a32, a33, a34,
                a41, a42, a43, a44
            ) => {
                mat4!(
                    a11, a21, a31, a41,
                    a12, a22, a32, a42,
                    a13, a23, a33, a43,
                    a14, a24, a34, a44
                )
            }
        }
    }

    #[allow(dead_code)]
    pub const IDENT: Matrix4 = mat4!(
      1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0
    );

    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4 {
        mat4!(
          x, 0.0, 0.0, 0.0,
          0.0, y, 0.0, 0.0,
          0.0, 0.0, z, 0.0,
          0.0, 0.0, 0.0, 1.0
        )
    } 

    #[rustfmt::skip]
    pub fn pitch_rotation(r: f32) -> Matrix4 {
        mat4!(
          1.0,     0.0,      0.0, 0.0,
          0.0, r.cos(), -r.sin(), 0.0,
          0.0, r.sin(),  r.cos(), 0.0,
          0.0,     0.0,      0.0, 1.0
        )
    }

    #[rustfmt::skip]
    pub fn yaw_rotation(r: f32) -> Matrix4 {
        mat4!(
           r.cos(), 0.0, r.sin(), 0.0,
               0.0, 1.0,     0.0, 0.0,
          -r.sin(), 0.0, r.cos(), 0.0,
               0.0, 0.0,     0.0, 1.0
        )
    }

    #[allow(dead_code)]
    #[rustfmt::skip]
    pub fn roll_rotation(r: f32) -> Matrix4 {
        mat4!(
          r.cos(), -r.sin(), 0.0, 0.0,
          r.sin(),  r.cos(), 0.0, 0.0,
              0.0,      0.0, 1.0, 0.0,
              0.0,      0.0, 0.0, 1.0
        )
    }

    #[allow(dead_code)]
    pub fn rotation(pitch: f32, yaw: f32, roll: f32) -> Matrix4 {
        Self::roll_rotation(roll)*Self::yaw_rotation(yaw)*Self::pitch_rotation(pitch)
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
        mat4!(
          1.0, 0.0, 0.0, x,
          0.0, 1.0, 0.0, y,
          0.0, 0.0, 1.0, z,
          0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn look_at(camera_position: Vector3, camera_look_at: Vector3, camera_up: Vector3) -> Matrix4 {
        let cam_axis_z = (camera_position - camera_look_at).normalize();
        let cam_axis_x = camera_up.cross(cam_axis_z).normalize();
        let cam_axis_y = cam_axis_z.cross(cam_axis_x).normalize();

        match (camera_position, cam_axis_x, cam_axis_y, cam_axis_z) {
            (
              vec3!(cx, cy, cz),
              vec3!(rxx, rxy, rxz),
              vec3!(ryx, ryy, ryz),
              vec3!(rzx, rzy, rzz)
            ) => {
                mat4!(
                  rxx, rxy, rxz, 0.0,
                  ryx, ryy, ryz, 0.0,
                  rzx, rzy, rzz, 0.0,
                  0.0, 0.0, 0.0, 1.0
                ) * Self::translation(-cx, -cy, -cz)
            }
        }
    }

    pub fn perspective_fov(fov_y: f32, aspect_ratio: f32, near: f32, far: f32) -> Matrix4 {
        let sy = 1.0f32/(fov_y/2.0).tan();
        let sx = sy / aspect_ratio;

        mat4!(
            sx, 0.0, 0.0, 0.0,
            0.0, sy, 0.0, 0.0,
            0.0, 0.0, -(far + near)/(far - near), - 2.0 * near * far/(far - near),
            0.0, 0.0, -1.0, 0.0
        )
    } 
}
