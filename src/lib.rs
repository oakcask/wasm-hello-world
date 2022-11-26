mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;

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

fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context.create_shader(shader_type)
        .ok_or_else(|| String::from("createShader failed."))?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("failed to compile shader")))
    }
}

fn link_program(context: &WebGl2RenderingContext, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = context.create_program()
        .ok_or_else(|| String::from("createProgram failed."))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(context.get_program_info_log(&program).unwrap_or_else(|| String::from("failed to link program")))
    }
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

    let vert_shader = compile_shader(&context, WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
        in vec4 position;
        void main() {
            gl_Position = position;
        }
        "##,
    )?;
    log("vertex shader compiled.");

    let frag_shader = compile_shader(&context, WebGl2RenderingContext::FRAGMENT_SHADER, 
        r##"#version 300 es
        precision highp float;
        out vec4 outColor;
        void main() {
            outColor = vec4(1, 0, 0.5, 0.7);
        }
        "##,
    )?;
    log("fragment shader compiled.");

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    
    log("program created.");

    context.use_program(Some(&program));
    let position_attribute_location = context.get_attrib_location(&program, "position");
    
    log("program enabled.");

    let buffer = context.create_buffer().expect("createBuffer failed.");

    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    let verts: [f32; 3*3] = [
        1.0,  1.0, 0.0,
        1.0, -1.0, 0.0,
        -1.0, 1.0, 0.0,
    ];
    unsafe {
        let view = js_sys::Float32Array::view(&verts);
        context.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &view, WebGl2RenderingContext::STATIC_DRAW);
    }

    let vert_array = context.create_vertex_array().unwrap();
    context.bind_vertex_array(Some(&vert_array));
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false, // not normalized
        0, // stride
        0 // offset
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, (verts.len() / 3) as i32);

    Ok(())
}
