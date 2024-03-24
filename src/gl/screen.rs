use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlRenderbuffer, WebGlTexture};
use crate::{error::Error, math::Size};
use crate::size;

use super::GL;

pub trait Screen {
    fn frame_buffer(&self) -> Option<&WebGlFramebuffer>;
}

pub struct FrameBuffer {
    gl: GL,
    frame_buffer: WebGlFramebuffer,
    render_buffer: WebGlRenderbuffer,
    texture: WebGlTexture,
}

impl FrameBuffer {
    pub fn new(gl: &GL, size: Size) -> Result<FrameBuffer, Error> {
        let size!(width, height) = size;
        let ctx = gl.context();

        let render_buffer = ctx.create_renderbuffer().ok_or_else(||
            String::from("createRenderBuffer failed")
        )?;
        ctx.renderbuffer_storage(WebGl2RenderingContext::RENDERBUFFER, WebGl2RenderingContext::DEPTH_STENCIL, width, height);

        let texture = ctx.create_texture().ok_or_else(|| String::from("createTexture failed"))?;
        ctx.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
    
        ctx.tex_storage_2d(WebGl2RenderingContext::TEXTURE_2D, 1, WebGl2RenderingContext::RGBA32F,
            width,
            height);
        ctx.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        ctx.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        ctx.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
        ctx.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);

        let frame_buffer = ctx.create_framebuffer().ok_or_else(||
            String::from("createFramebuffer failed")
        )?;
        ctx.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&frame_buffer));

        ctx.framebuffer_renderbuffer(
           WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::DEPTH_STENCIL_ATTACHMENT,
            WebGl2RenderingContext::RENDERBUFFER,
            Some(&render_buffer),
        );
        ctx.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::COLOR_ATTACHMENT0,
            WebGl2RenderingContext::TEXTURE_2D,
            Some(&texture),
            0);

        ctx.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

        Ok(FrameBuffer {
            gl: gl.clone(),
            frame_buffer,
            render_buffer,
            texture,
        })
    }

    pub fn texture(&self) -> &WebGlTexture {
        &self.texture
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        let ctx = self.gl.context();
        ctx.delete_framebuffer(Some(&self.frame_buffer));
        ctx.delete_renderbuffer(Some(&self.render_buffer));
        ctx.delete_texture(Some(&self.texture));
    }
}

impl Screen for FrameBuffer {
   fn frame_buffer(&self) -> Option<&WebGlFramebuffer> {
       Some(&self.frame_buffer)
   } 
}
