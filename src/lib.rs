mod utils;

use wasm_bindgen::prelude::*;
use web_sys::Element;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-wasm-v-js!");
}

#[wasm_bindgen]
pub fn get_element(selector: &str) -> Result<Option<Element>, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    document.query_selector(selector)
}

#[wasm_bindgen]
pub fn transform_array() -> Vec<u32> {
    let vec = vec![2; 500000];

    let vec2 = vec.into_iter().map(|x| x + 1).collect();

    vec2
}