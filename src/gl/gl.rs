use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wasm_bindgen::JsCast;
use crate::{math::Vector4, vec4};

pub struct GL {
    element: HtmlCanvasElement,
    context: WebGl2RenderingContext
}

impl GL {
    pub fn init(element_id: &str) -> Result<GL, String> {
        let window = web_sys::window().ok_or_else(|| String::from("global window object not found."))?;
        let document = window.document().ok_or_else(|| String::from("window document object not found."))?;

        let el = document.get_element_by_id(element_id).ok_or_else(|| format!("element with id={} not found.", element_id))?;
        let canvas = el.dyn_into::<HtmlCanvasElement>()
            .map_err(|_| format!("element with id={} is not HTMLCanvasElement", element_id))?;

        let context = canvas
            .get_context("webgl2")
            .map_err(|_| String::from("getContext('webgl2') failed."))?
            .ok_or_else(|| String::from("getContext('webgl2') returned null."))?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|_| String::from("failed to cast into WebGl2RenderingContext"))?;

        Ok(GL{
            element: canvas,
            context,
        })
    }
    
    pub fn clear<Color>(&self, color: Color)
        where Color: Into<Vector4> {
        match color.into() {
            vec4!(r, g, b, a) => {
                self.context.clear_color(r, g, b, a)
            }
        }
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT)
    }

    pub fn context(&self) -> &WebGl2RenderingContext {
        &self.context
    }

    pub fn size(&self) -> (u32, u32) { 
        (self.element.width(), self.element.height())
    }

    pub fn aspect_ratio(&self) -> f32 {
        let (x, y) = self.size();
        x as f32 / y as f32
    }
}