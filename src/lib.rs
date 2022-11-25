mod utils;

use wasm_bindgen::prelude::*;
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
            let canvas = document.create_element("canvas").expect("createElement failed.");
            el.append_child(&canvas).expect("appendChild failed.");
        },
        None => {
        }
    }
    ()
}
