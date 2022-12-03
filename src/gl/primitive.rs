use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlVertexArrayObject;
use super::shader::Shader;

pub trait VertexArray {
  fn as_slice(&self) -> &[f32];
  fn draw_array_mode(&self) -> u32;
  fn vertex_count(&self) -> i32;
  fn stride(&self) -> i32;
}

pub struct TriangleStrip {
  pub vertices: Vec<f32> 
}

impl VertexArray for TriangleStrip {
  fn as_slice(&self) -> &[f32] {
    self.vertices.as_slice()
  }

  fn draw_array_mode(&self) -> u32 {
    WebGl2RenderingContext::TRIANGLE_STRIP
  }

  fn vertex_count(&self) -> i32 {
    (self.vertices.len() / 3) as i32
  }

  fn stride(&self) -> i32 {
    0
  }
}

pub struct Primitive<'a, T: VertexArray> {
  ctx: &'a WebGl2RenderingContext,
  data: T,
  vao: WebGlVertexArrayObject,
  _vertex_buffer: WebGlBuffer,
}

pub fn create_primitive<'a, T: VertexArray>(ctx: &'a WebGl2RenderingContext, data: T) -> Result<Primitive<'a, T>, String> {
    let vao = ctx.create_vertex_array()
      .ok_or_else(|| String::from("createVertexArray failed."))?;
    let buffer = ctx.create_buffer()
      .ok_or_else(|| String::from("createBuffer failed."))?;
 
    ctx.bind_vertex_array(Some(&vao));
    ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    unsafe {
      let view = js_sys::Float32Array::view(data.as_slice());
      ctx.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &view, WebGl2RenderingContext::STATIC_DRAW);
    }

    ctx.bind_vertex_array(None);

    Ok(Primitive{
      ctx,
      data,
      vao,
      _vertex_buffer: buffer,
    })
}

impl <'a, T: VertexArray> Primitive<'a, T> {
  pub fn draw(&self, shader: &Shader<'a>) {
    shader.enable();
    self.ctx.bind_vertex_array(Some(&self.vao));

    let position_attrib_location = shader.get_attrib_location("position");
    self.ctx.vertex_attrib_pointer_with_i32(
      position_attrib_location,
      3,
      WebGl2RenderingContext::FLOAT,
      false,
      0,
      self.data.stride()
    );
    self.ctx.enable_vertex_attrib_array(position_attrib_location);
    self.ctx.draw_arrays(self.data.draw_array_mode(), 0, self.data.vertex_count());
    self.ctx.bind_vertex_array(None);
    shader.disable();
  }
}