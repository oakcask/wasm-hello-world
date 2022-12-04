mod gl;
mod utils;

use gl::TriangleStrip;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn main(id: &str) -> Result<(), JsValue> {

    let screen = gl::create_screen(id, 640, 480)?;
    screen.clear((0.0, 0.0, 0.0, 1.0));

    let context = screen.context();
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
    let shader = gl::create_shader(&context, vert_shader_source, frag_shader_source)?;

    #[rustfmt::skip]
    let vertices = TriangleStrip {
        vertices: vec![
            0.5, 0.5, 0.0,
            0.5, -0.5, 0.0,
            -0.5, 0.5, 0.0
        ],
    };

    let triangle = gl::create_primitive(&context, vertices)?;

    triangle.draw(&shader);

    Ok(())
}
