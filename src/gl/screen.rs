use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::WebGl2RenderingContext;
use web_sys::{Document, Element};

use super::Color4;

pub struct Screen {
    _element: HtmlCanvasElement,
    context: WebGl2RenderingContext
}

fn create_element(document: &Document, tag: &str) -> Result<Element, String> {
    document.create_element(tag).map_err(|_| String::from("createElement failed."))
}

pub fn create_screen(element_id: &str, width: u32, height: u32) -> Result<Screen, String> {
    let window = web_sys::window().ok_or_else(|| String::from("global window object not found."))?;
    let document = window.document().ok_or_else(|| String::from("window document object not found."))?;

    let el = document.get_element_by_id(element_id).ok_or_else(|| String::from(format!("element with id={} not found.", element_id)))?;

    let canvas_ = create_element(&document, "canvas")?;
    let canvas = canvas_.dyn_into::<HtmlCanvasElement>().map_err(|_| String::from("failed to cast into HtmlCanvasElement"))?;
    canvas.set_width(width);
    canvas.set_height(height);

    el.append_child(&canvas).map_err(|_| String::from("appendChild failed."))?;

    let context = canvas
        .get_context("webgl2")
        .map_err(|_| String::from("getContext('webgl2') failed."))?
        .ok_or_else(|| String::from("getContext('webgl2') returned null."))?
        .dyn_into::<WebGl2RenderingContext>()
        .map_err(|_| String::from("failed to cast into WebGl2RenderingContext"))?;
    
    Ok(Screen{
        _element: canvas,
        context,
    })
}

impl Screen {
    pub fn clear(&self, color: Color4) {
        match color {
            (r, g, b, a) => {
                self.context.clear_color(r, g, b, a)
            }
        }
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT)
    }

    pub fn context(&self) -> &WebGl2RenderingContext {
        &self.context
    }
}