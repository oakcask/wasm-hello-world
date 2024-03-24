

use log::trace;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlTexture, WebGlVertexArrayObject};

use crate::{error::Error, mat4, math::{Matrix4, Rectangle, Size, Vector2, Vector3, Vector4}, vec2, vec4};

use super::{EphemeralPrimitive, Shader, UVMappedSliceTriangleList, GL};

pub struct Sprite {
    gl: GL,
    shader: Shader,
    vao: WebGlVertexArrayObject,
    vbuf: WebGlBuffer,
    screen_size: Size,
}

impl Sprite {
    pub fn new(gl: &GL, screen_size: Size) -> Result<Sprite, Error> {
        trace!("Initializing Sprite...");
        let ctx = gl.context();
        let vert_shader_source = r##"#version 300 es
            in vec4 position;
            in vec2 textureCoord;
            uniform mat4 mvp;
            out vec2 vTextureCoord;
            void main() {
                gl_Position = mvp * vec4(position.xy, 0.0, 1.0);
                vTextureCoord = textureCoord;
            }
            "##;
        let frag_shader_source = r##"#version 300 es
            precision mediump float;
            in vec2 vTextureCoord;
            uniform sampler2D texture0;
            out vec4 outColor;
            void main() {
                outColor = texture(texture0, vTextureCoord);
            }
        "##;

        let shader = Shader::new(
            gl,
            vert_shader_source,
            frag_shader_source
        )?;
        trace!("Sprite shader compiled.");
        let vbuf = ctx.create_buffer().ok_or("glCreateBuffer failed")?;
        let vao = ctx.create_vertex_array()
            .ok_or("glCreateVertexArray failed")?;

        Ok(Sprite { gl: gl.clone(), shader, vbuf, vao, screen_size })
    }

    // Transform matrix to normalize destination positions from
    // pixel coordinate ([0, width or height]) to OpenGL screen coordinate ([-1, 1]).
    #[rustfmt::skip]
    fn normalizer(size: Size) -> Matrix4 {
        let w = size.w as f32;
        let h = size.h as f32;
        mat4!(
            2.0/w,    0.0, 0.0, -1.0,
              0.0, -2.0/h, 0.0,  1.0,
              0.0,    0.0, 0.0,  0.0,
              0.0,    0.0, 0.0,  1.0
        )
    }

    pub fn draw(&self, batch: SpriteBatch) {
        let ctx = self.gl.context();
        let transform = Self::normalizer(self.screen_size);

        ctx.disable(WebGl2RenderingContext::DEPTH_TEST);
        ctx.disable(WebGl2RenderingContext::CULL_FACE);

        for (tex, _src, verts) in batch.commands {
            let verts = verts.as_slice();
            let verts = UVMappedSliceTriangleList(verts);

            let obj = EphemeralPrimitive::transfer(&self.gl, verts, &self.vao, &self.vbuf);
            self.shader.enable_vertex_attribute(&obj);

            ctx.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex));
            ctx.active_texture(WebGl2RenderingContext::TEXTURE0); 
            self.shader.enable();

            self.shader.set_uniform_model_view_perspective(&transform);
            self.shader.set_uniform_texture(0);
            self.shader.draw(&obj);
            self.shader.disable();
            ctx.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        }
    } 
}

pub struct SpriteBatch {
    commands: Vec<(WebGlTexture, Vector4, Vec<(Vector3, Vector2)>)>
}

impl SpriteBatch {
    pub fn new() -> SpriteBatch {
        SpriteBatch {
            commands: Vec::new()
        }
    }

    fn push_vertices(v: &mut Vec<(Vector3, Vector2)>, source: Vector4, destination: Rectangle) {
        // destination (Screen Coordination)
        // x-----------------*------>
        // |(x, y)           |(x + w, y)
        // |                 |
        // |                 |
        // |(x, y + h)       |
        // +-----------------+(x + w, y + h)
        // |
        // V
        //
        // source (texture coordination)
        // 
        // A
        // |                  (u2, v2)
        // +-----------------+
        // |                 |
        // |                 |
        // |                 |
        // |(u1, v1)         |
        // x-----------------+----->
        //
        let vec4!(x, y, w, h) = destination.into();
        let dtl = vec2!(x, y); // (0, 0) goes to (-1, 1); top left corner
        let dtr = vec2!(x + w, y); // (w, 0) goes to (1, 1); top right corner
        let dbl = vec2!(x, y + h); // (0, h) goes to (-1, -1); bottom left corner
        let dbr = vec2!(x + w, y + h); // (w, h) goes to (1, -1); bottom right corner
        
        let vec4!(u1, v1, u2, v2) = source;
        let stl = vec2!(u1, v2);
        let str = vec2!(u2, v2);
        let sbl = vec2!(u1, v1);
        let sbr = vec2!(u2, v1);

        // debug
        // let n = Sprite::normalizer(size!(1920, 1080));
        // debug!("{:?} => {:?} @ {:?}", dtl, n * vec3!(dtl.x, dtl.y, 0.0), stl);
        // debug!("{:?} => {:?} @ {:?}", dtr, n * vec3!(dtr.x, dtr.y, 0.0), str);
        // debug!("{:?} => {:?} @ {:?}", dbl, n * vec3!(dbl.x, dbl.y, 0.0), sbl);
        // debug!("{:?} => {:?} @ {:?}", dbr, n * vec3!(dbr.x, dbr.y, 0.0), sbr);
        // panic!("stop");        
        

        // we are going to invert y-position (see normalizer()),
        // so rendered rectangle will face backward.
        // Needs to place vertices in reverse-clockwise order.
        v.push((dtl.into(), stl));
        v.push((dbl.into(), sbl));
        v.push((dtr.into(), str));

        v.push((dbr.into(), sbr));
        v.push((dtr.into(), str));
        v.push((dbl.into(), sbl));
    }

    pub fn add(&mut self, texture: &WebGlTexture, source: Vector4, destination: Rectangle) {
        if let Some((tex, src, _)) = self.commands.last() {
            if texture == tex && &source == src {
                if let Some((_, _, verts)) = self.commands.last_mut() {
                    Self::push_vertices(verts, source, destination);
                    return;
                }
            }
        }

        let mut v = Vec::new();
        Self::push_vertices(&mut v, source, destination);
        self.commands.push((texture.clone(), source, v));
    }
}