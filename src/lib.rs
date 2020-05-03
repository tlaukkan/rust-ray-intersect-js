use js_sys;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Initialized panic hook.
#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-ray-intersect-js!");
}

#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[wasm_bindgen]
pub fn bad_add(a: f64, b: f64) -> f64 {
    a - b
}

#[wasm_bindgen]
pub fn test_number_array(array: JsValue) -> usize {
    let elements: Vec<u32> = array.into_serde().unwrap();
    return elements.len();
}

#[wasm_bindgen]
pub fn test_float_32_array(array: js_sys::Float32Array) -> usize {
    let elements: Vec<f32> = array.to_vec();
    return elements.len();
}

#[wasm_bindgen]
pub fn test_float_64_array(array: js_sys::Float64Array) -> usize {
    let elements: Vec<f64> = array.to_vec();
    return elements.len();
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1.0, 2.0), 3.0);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1.0, 2.0), 3.0);
    }
}
