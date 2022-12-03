use web_sys::WebGl2RenderingContext;
use web_sys::WebGlShader;
use web_sys::WebGlProgram;

pub struct Shader<'a> {
  ctx: &'a WebGl2RenderingContext,
  program: WebGlProgram,
  _vertex_shader: WebGlShader,
  _fragment_shader: WebGlShader,
}

impl <'a> Shader<'a> {
  pub fn get_attrib_location(&self, name: &str) -> u32 {
    self.ctx.get_attrib_location(&self.program, name) as u32
  }

  pub fn enable(&self) {
    self.ctx.use_program(Some(&self.program));
  }

  pub fn disable(&self) {
    self.ctx.use_program(None);
  }
}

fn compile_shader(ctx: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
  let glshader = ctx.create_shader(shader_type)
    .ok_or_else(|| String::from("createShader failed."))?; 

  ctx.shader_source(&glshader, source);
  ctx.compile_shader(&glshader);

  if ctx.get_shader_parameter(&glshader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
    Ok(glshader)
  } else {
    Err(ctx.get_shader_info_log(&glshader).unwrap_or_else(|| String::from("compleShader failed.")))    
  }
}

pub fn create_shader<'a>(ctx: &'a WebGl2RenderingContext, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Shader<'a>, String> {
  let program = ctx.create_program().ok_or_else(|| String::from("createProgram failed."))?;
  let vertex_shader = compile_shader(ctx, WebGl2RenderingContext::VERTEX_SHADER, vertex_shader_source)?;
  let fragment_shader = compile_shader(ctx, WebGl2RenderingContext::FRAGMENT_SHADER, fragment_shader_source)?;

  ctx.attach_shader(&program, &vertex_shader);
  ctx.attach_shader(&program, &fragment_shader);
  ctx.link_program(&program);

  Ok(Shader{
    ctx,
    program,
    _vertex_shader: vertex_shader,
    _fragment_shader: fragment_shader,
  })
}