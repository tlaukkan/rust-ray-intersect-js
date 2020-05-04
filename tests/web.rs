//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

#[path = "../src/math/calc.rs"]
use intersect;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/*
#[wasm_bindgen_test]
fn pass() {
    assert_eq!(intersect::add(1.0, 1.0), 2.0);
}
*/
