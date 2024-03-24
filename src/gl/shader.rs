use std::rc::Rc;

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::WebGlVertexArrayObject;

use crate::math::Matrix4;
use super::Screen;

pub struct Shader {
    screen: Rc<Screen>,
    program: WebGlProgram,
    _vertex_shader: WebGlShader,
    _fragment_shader: WebGlShader,
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
}

impl From<DrawArrayMode> for u32 {
    fn from(val: DrawArrayMode) -> Self {
        match val {
            DrawArrayMode::TriangleStrip => WebGl2RenderingContext::TRIANGLE_STRIP
        }
    }
}

pub trait Drawable {
    fn draw_array_mode(&self) -> DrawArrayMode;
    fn position(&self) -> Option<VertexAttribute>;
    fn color(&self) -> Option<VertexAttribute>;
    fn texture_coordination(&self) -> Option<VertexAttribute>;
    // Number of vertices in the vertex attribute array.
    fn vertex_count(&self) -> i32;
    fn vertex_array_object(&self) -> &WebGlVertexArrayObject;
}

impl Shader {
    fn ctx(&self) -> &WebGl2RenderingContext {
        self.screen.context()
    }

    pub fn draw<T: Drawable>(&self, obj: &T) {
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

        ctx 
            .draw_arrays(obj.draw_array_mode().into(), 0, obj.vertex_count());
        ctx.bind_vertex_array(None);
    }

    fn enable_vertex_attribute_array(ctx: &WebGl2RenderingContext, program: &WebGlProgram, name: &str, attr: VertexAttribute) {
        let idx = ctx.get_attrib_location(program, name) as u32;
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

    pub fn set_uniform_matrix4(&self, pname: &str, uniform: &Matrix4) {
        let ctx = self.ctx();
        let idx = ctx.get_uniform_location(&self.program, pname);
        ctx.uniform_matrix4fv_with_f32_array(idx.as_ref(), true, uniform.as_ref());
    }

    #[allow(dead_code)]
    pub fn set_uniform_sampler2d(&self, pname: &str, texture_unit: i32) {
        let ctx = self.ctx();
        let idx = ctx.get_uniform_location(&self.program, pname);
        ctx.uniform1i(idx.as_ref(), texture_unit);
    }

    pub fn enable(&self) {
        self.ctx().use_program(Some(&self.program));
    }

    pub fn disable(&self) {
        self.ctx().use_program(None);
    }
}

fn compile_shader(
    ctx: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let glshader = ctx
        .create_shader(shader_type)
        .ok_or_else(|| String::from("createShader failed."))?;

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
            .unwrap_or_else(|| String::from("compleShader failed.")))
    }
}

pub fn create_shader(
    screen: &Rc<Screen>,
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> Result<Shader, String> {
    let ctx = screen.context();
    let program = ctx
        .create_program()
        .ok_or_else(|| String::from("createProgram failed."))?;
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
        screen: screen.clone(),
        program,
        _vertex_shader: vertex_shader,
        _fragment_shader: fragment_shader,
    })
}
