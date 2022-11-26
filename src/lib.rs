mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

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
pub fn main(id: &str) {
    let window = web_sys::window().expect("global window not found.");
    let document = window.document().expect("window.document not found.");

    match document.get_element_by_id(id) {
        Some(el) => {
            let o = document.create_element("canvas").expect("createElement failed.");
            el.append_child(&o).expect("appendChild failed.");
            let canvas = o.dyn_into::<web_sys::HtmlCanvasElement>().expect("failed to cast into HtmlCanvasElement");
            let context = canvas.get_context("webgl2")
                .expect("getContext('webgl2') failed.")
                .expect("getContext('webgl2') retuned null.")
                .dyn_into::<web_sys::WebGl2RenderingContext>().
                expect("failed to cast into WebGl2RenderingContext");
            context.clear_color(0.0, 0.0, 0.0, 1.0);
            context.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);
        },
        None => {
        }
    }
    ()
}
