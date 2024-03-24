
use std::marker::PhantomData;
use std::mem::size_of;
use std::mem::size_of_val;

use crate::error::Error;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use super::DrawArrayMode;
use super::Drawable;


use super::VertexAttribute;
use super::GL;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlVertexArrayObject;

pub trait VertexAttributeArray {
    const DRAW_ARRAY_MODE: DrawArrayMode;
    const POSITION: Option<VertexAttribute>;
    const COLOR: Option<VertexAttribute>;
    const TEXTURE_COORDINATION: Option<VertexAttribute>;
    
    // Pointer to the first element of vertex attribute array.
    fn as_slice(&self) -> &[f32];
    // Number of vertices in the vertex attribute array.
    fn vertex_count(&self) -> i32;
}

pub struct UVMappedSliceTriangleStrip<'a>(pub &'a [(Vector3, Vector2)]);

impl<'a> VertexAttributeArray for UVMappedSliceTriangleStrip<'a> {
    const DRAW_ARRAY_MODE: DrawArrayMode = DrawArrayMode::TriangleStrip;
    const POSITION: Option<VertexAttribute> = Some(VertexAttribute { offset: 0, size: 3, stride: 20 });
    const COLOR: Option<VertexAttribute> = None;
    const TEXTURE_COORDINATION: Option<VertexAttribute> = Some(VertexAttribute { offset: 12, size: 2, stride: 20 });

    fn as_slice(&self) -> &[f32] {
        unsafe {
            // HACK:
            // Yes, this is really unsafe and causes undefined behavior.
            // Rust's tuple virtually has `repr(rust)` so compiler CAN reorder the member.
            // as far as I know, but, it seems that the order of the tuple members are kept.
            //
            // If we want to make this "safe", we should define Vector3 as repr(C) struct, instead of tuple. 
            // It'll be obvious hurt writing pattern maching against Vector3. Need macro...
            std::slice::from_raw_parts(&self.0[0].0.x as *const f32, size_of_val(self.0) / size_of::<f32>())
        }
    }

    fn vertex_count(&self) -> i32 {
        self.0.len() as i32
    }
}

pub struct UVMappedSliceTriangleList<'a>(pub &'a [(Vector3, Vector2)]);

impl<'a> VertexAttributeArray for UVMappedSliceTriangleList<'a> {
    const DRAW_ARRAY_MODE: DrawArrayMode = DrawArrayMode::TriangleList;
    const POSITION: Option<VertexAttribute> = Some(VertexAttribute { offset: 0, size: 3, stride: 20 });
    const COLOR: Option<VertexAttribute> = None;
    const TEXTURE_COORDINATION: Option<VertexAttribute> = Some(VertexAttribute { offset: 12, size: 2, stride: 20 });

    fn as_slice(&self) -> &[f32] {
        unsafe {
            // HACK:
            // Yes, this is really unsafe and causes undefined behavior.
            // Rust's tuple virtually has `repr(rust)` so compiler CAN reorder the member.
            // as far as I know, but, it seems that the order of the tuple members are kept.
            //
            // If we want to make this "safe", we should define Vector3 as repr(C) struct, instead of tuple. 
            // It'll be obvious hurt writing pattern maching against Vector3. Need macro...
            std::slice::from_raw_parts(&self.0[0].0.x as *const f32, self.vertex_count() as usize * 5)
        }
    }

    fn vertex_count(&self) -> i32 {
        self.0.len() as i32
    }
}

pub struct ColoredSliceTriangleStrip<'a>(pub &'a [(Vector3, Vector4)]);

impl<'a> VertexAttributeArray for ColoredSliceTriangleStrip<'a> {
    const DRAW_ARRAY_MODE: DrawArrayMode = DrawArrayMode::TriangleStrip;
    const POSITION: Option<VertexAttribute> = Some(VertexAttribute { offset: 0, size: 3, stride: 28 });
    const COLOR: Option<VertexAttribute> = Some(VertexAttribute { offset: size_of::<Vector3>(), size:  4, stride: 28 });
    const TEXTURE_COORDINATION: Option<VertexAttribute> = None;

    fn as_slice(&self) -> &[f32] {
        unsafe {
            // HACK:
            // Yes, this is really unsafe and causes undefined behavior.
            // Rust's tuple virtually has `repr(rust)` so compiler CAN reorder the member.
            // as far as I know, but, it seems that the order of the tuple members are kept.
            //
            // If we want to make this "safe", we should define Vector3 as repr(C) struct, instead of tuple. 
            // It'll be obvious hurt writing pattern maching against Vector3. Need macro...
            std::slice::from_raw_parts(&self.0[0].0.x as *const f32, size_of_val(self.0) / size_of::<f32>())
        }
    }

    fn vertex_count(&self) -> i32 {
        self.0.len() as i32
    }
}

pub struct TriangleStrip {
    pub vertices: Vec<f32>,
}

impl VertexAttributeArray for TriangleStrip {
    const DRAW_ARRAY_MODE: DrawArrayMode = DrawArrayMode::TriangleStrip;
    const POSITION: Option<VertexAttribute> = Some(VertexAttribute { offset: 0, size: 3, stride: 0 });
    const COLOR: Option<VertexAttribute> = None;
    const TEXTURE_COORDINATION: Option<VertexAttribute> = None;

    fn as_slice(&self) -> &[f32] {
        self.vertices.as_slice()
    }

    fn vertex_count(&self) -> i32 {
        (self.vertices.len() / 3) as i32
    }
}

pub struct Primitive {
    gl: GL,
    vao: WebGlVertexArrayObject,
    vertex_count: i32,
    draw_array_mode: DrawArrayMode,
    position: Option<VertexAttribute>,
    color: Option<VertexAttribute>,
    texture_coordination: Option<VertexAttribute>,
    _vertex_buffer: WebGlBuffer,
}

fn transfer<T: VertexAttributeArray>(gl: &GL, data: T, vao: &WebGlVertexArrayObject, buffer: &WebGlBuffer) {
    let ctx = gl.context();
    ctx.bind_vertex_array(Some(vao));
    ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(buffer));

    unsafe {
        let view = js_sys::Float32Array::view(data.as_slice());
        ctx.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    ctx.bind_vertex_array(None);
}

impl Primitive {
    pub fn new<T: VertexAttributeArray>(
        gl: &GL,
        data: T,
    ) -> Result<Primitive, Error> {
        let ctx = gl.context();
        let vao = ctx
            .create_vertex_array()
            .ok_or("createVertexArray failed.")?;
        let buffer = ctx
            .create_buffer()
            .ok_or("createBuffer failed.")?;
        let vertex_count = data.vertex_count();

        transfer(gl, data, &vao, &buffer);

        Ok(Primitive {
            gl: gl.clone(),
            vao,
            vertex_count,
            draw_array_mode: T::DRAW_ARRAY_MODE,
            position: T::POSITION,
            color: T::COLOR,
            texture_coordination: T::TEXTURE_COORDINATION,
            _vertex_buffer: buffer,
        })
    }
}

impl Drawable for Primitive {
    fn draw_array_mode(&self) -> DrawArrayMode {
        self.draw_array_mode
    }
    
    fn position(&self) -> Option<VertexAttribute> {
        self.position
    }
    
    fn color(&self) -> Option<VertexAttribute> {
        self.color
    }

    fn texture_coordination(&self) -> Option<VertexAttribute> {
        self.texture_coordination
    }

    fn vertex_count(&self) -> i32 {
        self.vertex_count
    }

    fn vertex_array_object(&self) -> &WebGlVertexArrayObject {
        &self.vao
    }
}

impl Drop for Primitive {
    fn drop(&mut self) {
        let ctx = self.gl.context();
        ctx.delete_vertex_array(Some(&self.vao));
        ctx.delete_buffer(Some(&self._vertex_buffer));
    }
}

pub struct EphemeralPrimitive<'a, T: VertexAttributeArray> {
    vao: &'a WebGlVertexArrayObject,
    _buffer: &'a WebGlBuffer,
    vertex_count: i32,
    _phantom_data: PhantomData<T>
}

impl<'a, T: VertexAttributeArray> EphemeralPrimitive<'a, T> {
    pub fn transfer(gl: &GL, data: T, vao: &'a WebGlVertexArrayObject, buffer: &'a WebGlBuffer) -> Self {
        let vertex_count = data.vertex_count();
        super::primitive::transfer(gl, data, vao, buffer);

        EphemeralPrimitive {
            vao,
            _buffer: buffer,
            vertex_count,
            _phantom_data: PhantomData
        }
    }
}

impl<'a, T: VertexAttributeArray> Drawable for EphemeralPrimitive<'a, T> {
    fn draw_array_mode(&self) -> DrawArrayMode {
        T::DRAW_ARRAY_MODE
    }

    fn position(&self) -> Option<VertexAttribute> {
        T::POSITION
    }

    fn color(&self) -> Option<VertexAttribute> {
        T::COLOR
    }

    fn texture_coordination(&self) -> Option<VertexAttribute> {
        T::TEXTURE_COORDINATION
    }

    fn vertex_count(&self) -> i32 {
        self.vertex_count
    }

    fn vertex_array_object(&self) -> &WebGlVertexArrayObject {
        self.vao
    }
}
