use crate::vec4;

use super::Vector4;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub w: i32,
    pub h: i32
}

#[macro_export]
macro_rules! rect {
    (
        $x:ident, $y:ident, $w:ident, $h:ident
    ) => ($crate::math::Rectangle {
        x: $x, y: $y, w: $w, h: $h
    });
    (
        $x:expr, $y:expr, $w:expr, $h:expr
    ) => ($crate::math::Rectangle {
        x: $x, y: $y, w: $w, h: $h
    });
}

#[macro_export]
macro_rules! size {
    (
        $w:ident, $h:ident
    ) => ($crate::math::Size {
        w: $w, h: $h
    });
    (
        $w:expr, $h:expr
    ) => ($crate::math::Size {
        w: $w, h: $h
    });
}

impl From<Size> for Rectangle {
    fn from(value: Size) -> Self {
        rect!(0, 0, value.w, value.h)
    }
}

impl From<Rectangle> for Size {
    fn from(value: Rectangle) -> Self {
        size!(value.w, value.h)
    }
}

impl From<(i32, i32)> for Size {
    fn from(value: (i32, i32)) -> Self {
        size!(value.0, value.1)
    }
}

impl From<Rectangle> for Vector4 {
    fn from(val: Rectangle) -> Self {
        match val {
            rect!(x, y, z, w) => vec4!(x as f32, y as f32, z as f32, w as f32)
        }  
    }
}