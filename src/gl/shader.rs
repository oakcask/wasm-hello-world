use web_sys::WebGl2RenderingContext;

use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::WebGlVertexArrayObject;

use crate::error::Error;
use crate::math::Matrix4;
use crate::math::Vector4;
use crate::vec4;
use super::GL;

pub struct Shader {
    gl: GL,
    program: WebGlProgram,
    vertex_shader: WebGlShader,
    fragment_shader: WebGlShader,
}

#[derive(Clone, Copy)]
pub struct VertexAttribute {
    pub offset: usize,
    pub size: usize,
    pub stride: usize,
}

#[derive(Clone, Copy)]
pub enum DrawArrayMode {
    TriangleStrip,
    TriangleList
}

impl From<DrawArrayMode> for u32 {
    fn from(val: DrawArrayMode) -> Self {
        match val {
            DrawArrayMode::TriangleStrip => WebGl2RenderingContext::TRIANGLE_STRIP,
            DrawArrayMode::TriangleList => WebGl2RenderingContext::TRIANGLES
        }
    }
}

pub trait Drawable {
    fn position(&self) -> Option<VertexAttribute>;
    fn color(&self) -> Option<VertexAttribute>;
    fn texture_coordination(&self) -> Option<VertexAttribute>;

    fn draw_array_mode(&self) -> DrawArrayMode;
    // Number of vertices in the vertex attribute array.
    fn vertex_count(&self) -> i32;
    fn vertex_array_object(&self) -> &WebGlVertexArrayObject;
}

impl Shader {
    pub fn new(
        gl: &GL,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Result<Shader, Error> {
        let ctx = gl.context();
        let program = ctx
            .create_program()
            .ok_or("createProgram failed.")?;
        let vertex_shader = compile_shader(
            ctx,
            WebGl2RenderingContext::VERTEX_SHADER,
            vertex_shader_source,
        )?;
        let fragment_shader = compile_shader(
            ctx,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            fragment_shader_source,
        )?;

        ctx.attach_shader(&program, &vertex_shader);
        ctx.attach_shader(&program, &fragment_shader);
        ctx.link_program(&program);

        Ok(Shader {
            gl: gl.clone(),
            program,
            vertex_shader,
            fragment_shader,
        })
    }

    fn ctx(&self) -> &WebGl2RenderingContext {
        self.gl.context()
    }

    pub fn enable_vertex_attribute<T: Drawable>(&self, obj: &T) {
        let ctx = self.ctx();
        let vao = obj.vertex_array_object();

        ctx.bind_vertex_array(Some(vao));

        if let Some(position) = obj.position() {
            Self::enable_vertex_attribute_array(ctx, &self.program, "position", position)
        }
        if let Some(color) = obj.color() {
            Self::enable_vertex_attribute_array(ctx, &self.program, "color", color)
        }
        if let Some(texture_coordination) = obj.texture_coordination() {
            Self::enable_vertex_attribute_array(ctx, &self.program, "textureCoord", texture_coordination)
        }

        ctx.bind_vertex_array(None);
    }

    pub fn draw<T: Drawable>(&self, obj: &T) {
        let ctx = self.ctx();
        let vao = obj.vertex_array_object();
        ctx.bind_vertex_array(Some(vao));
        ctx 
            .draw_arrays(obj.draw_array_mode().into(), 0, obj.vertex_count());
        ctx.bind_vertex_array(None);
    }

    fn get_attrib_location(ctx: &WebGl2RenderingContext, program: &WebGlProgram, name: &str) -> Option<u32> {
        let idx = ctx.get_attrib_location(program, name);
        if idx == -1 {
            return None;
        }

        Some(idx as u32)
    }

    fn enable_vertex_attribute_array(ctx: &WebGl2RenderingContext, program: &WebGlProgram, name: &str, attr: VertexAttribute) {
        if let Some(idx) = Self::get_attrib_location(ctx, program, name) {
            ctx.vertex_attrib_pointer_with_i32(
                idx,
                attr.size as i32,
                WebGl2RenderingContext::FLOAT,
                false,
                attr.stride as i32,
                attr.offset as i32
            );
            ctx.enable_vertex_attrib_array(idx);
        }
    }

    pub fn set_uniform_model_view_perspective(&self, uniform: &Matrix4) {
        let ctx = self.ctx();
        let idx = ctx.get_uniform_location(&self.program, "mvp");
        ctx.uniform_matrix4fv_with_f32_array(idx.as_ref(), true, uniform.as_ref());
    }

    #[allow(dead_code)]
    pub fn set_uniform_texture(&self, texture_unit: i32) {
        let ctx = self.ctx();
        let idx = ctx.get_uniform_location(&self.program, "texture0");
        ctx.uniform1i(idx.as_ref(), texture_unit);
    }

    #[allow(dead_code)]
    pub fn set_uniform_color<Color: Into<Vector4>>(&self, color: Color) {
        let ctx = self.ctx();
        let idx = ctx.get_uniform_location(&self.program, "color");
        match color.into() {
            vec4!(x, y, z, w) => ctx.uniform4f(idx.as_ref(), x, y, z, w)
        }
    }

    pub fn enable(&self) {
        self.ctx().use_program(Some(&self.program));
    }

    pub fn disable(&self) {
        self.ctx().use_program(None);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        let ctx = self.ctx();
        ctx.delete_program(Some(&self.program));
        ctx.delete_shader(Some(&self.vertex_shader));
        ctx.delete_shader(Some(&self.fragment_shader));
    }
}

fn compile_shader(
    ctx: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, Error> {
    let glshader = ctx
        .create_shader(shader_type)
        .ok_or("createShader failed.")?;

    ctx.shader_source(&glshader, source);
    ctx.compile_shader(&glshader);

    if ctx
        .get_shader_parameter(&glshader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(glshader)
    } else {
        Err(ctx
            .get_shader_info_log(&glshader)
            .unwrap_or_else(|| String::from("compleShader failed.")).into())
    }
}
