use std::rc::Rc;

use log::trace;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wasm_bindgen::JsCast;
use crate::{error::Error, math::{Size, Vector4}, size, vec4};

#[derive(Clone)]
pub struct GL(Rc<Inner>);

struct Inner {
    element: HtmlCanvasElement,
    context: WebGl2RenderingContext
}

impl GL {
    pub fn init(element_id: &str) -> Result<GL, Error> {
        let window = web_sys::window().ok_or_else(|| "global window object not found.")?;
        let document = window.document().ok_or_else(|| "window document object not found.")?;

        let el = document.get_element_by_id(element_id).ok_or_else(|| format!("element with id={} not found.", element_id))?;
        let canvas = el.dyn_into::<HtmlCanvasElement>()
            .map_err(|_| format!("element with id={} is not HTMLCanvasElement", element_id))?;

        let context = canvas
            .get_context("webgl2")
            .map_err(|_| "getContext('webgl2') failed.")?
            .ok_or_else(|| "getContext('webgl2') returned null.")?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|_| "failed to cast into WebGl2RenderingContext")?;
    
        // https://github.com/mrdoob/three.js/issues/15493#issuecomment-450820195
        Self::ensure_extension(&context, "EXT_color_buffer_float")?;

        Ok(GL(
            Rc::new(Inner { element: canvas, context })
        ))
    }

    fn ensure_extension(ctx: &WebGl2RenderingContext, name: &str) -> Result<(), Error> {
        ctx.get_extension(name)?.ok_or_else(|| 
            format!("{} is disabled", name)
        )?;
        Ok(())
    }
    
    pub fn clear<Color>(&self, color: Color)
        where Color: Into<Vector4> {
        match color.into() {
            vec4!(r, g, b, a) => {
                self.0.context.clear_color(r, g, b, a)
            }
        }
        self.0.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT)
    }

    pub fn context(&self) -> &WebGl2RenderingContext {
        &self.0.context
    }

    pub fn screen_size(&self) -> Size { 
        let r = size!(self.0.element.width() as i32, self.0.element.height() as i32);
        trace!("GL::screen_size = {:?}", r);
        r
    }

    pub fn screen_aspect_ratio(&self) -> f32 {
        let size!(x, y) = self.screen_size();
        x as f32 / y as f32
    }
}
