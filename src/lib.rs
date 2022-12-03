mod utils;
mod gl;

use gl::primitive::TriangleStrip;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::WebGl2RenderingContext;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn main(id: &str) -> Result<(), JsValue> {
    let window = web_sys::window().expect("global window not found.");
    let document = window.document().expect("window.document not found.");
    let el = document.get_element_by_id(id).expect("getElementById failed.");
    let o = document.create_element("canvas").expect("createElement failed.");

    el.append_child(&o).expect("appendChild failed.");

    let canvas = o.dyn_into::<web_sys::HtmlCanvasElement>().expect("failed to cast into HtmlCanvasElement");
    let context = canvas.get_context("webgl2")
        .expect("getContext('webgl2') failed.")
        .expect("getContext('webgl2') retuned null.")
        .dyn_into::<WebGl2RenderingContext>().
        expect("failed to cast into WebGl2RenderingContext");

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let vert_shader_source = r##"#version 300 es
        in vec4 position;
        void main() {
            gl_Position = position;
        }
        "##;
    let frag_shader_source = r##"#version 300 es
        precision highp float;
        out vec4 outColor;
        void main() {
            outColor = vec4(1, 0, 0.5, 1);
        }
        "##;
    let shader = gl::shader::create_shader(&context, vert_shader_source, frag_shader_source)?;
    let vertices = TriangleStrip{
        vertices: vec![
            0.5,  0.5, 0.0,
            0.5, -0.5, 0.0,
            -0.5, 0.5, 0.0,
        ]
    };

    let triangle = gl::primitive::create_primitive(&context, vertices)?;

    triangle.draw(&shader);

    Ok(())
}