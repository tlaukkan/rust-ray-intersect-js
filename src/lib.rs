#[macro_use]
extern crate lazy_static;

use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::{BHShape, BoundingHierarchy};
use bvh::bvh::BVH;
use bvh::ray::Ray;
use js_sys;
use nalgebra::{Point3, Vector3};
use std::collections::HashMap;
use std::sync::Mutex;
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

#[wasm_bindgen]
pub struct Intersection {
    pub triangle_index: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[wasm_bindgen]
pub fn save_mesh_triangles(
    mesh_id: &str,
    indices: js_sys::Uint32Array,
    positions: js_sys::Float32Array,
) {
}

#[wasm_bindgen]
pub fn intersect_vector_with_mesh_triangles(
    mesh_id: &str,
    start_x: f32,
    start_y: f32,
    start_z: f32,
    end_x: f32,
    end_y: f32,
    end_z: f32,
) -> Option<Intersection> {
    return None;
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub struct Triangle {
    pub a: Point3<f32>,
    pub b: Point3<f32>,
    pub c: Point3<f32>,
    aabb: AABB,
    node_index: usize,
}

impl Triangle {
    pub fn new(a: Point3<f32>, b: Point3<f32>, c: Point3<f32>) -> Triangle {
        Triangle {
            a: a,
            b: b,
            c: c,
            aabb: AABB::empty().grow(&a).grow(&b).grow(&c),
            node_index: 0,
        }
    }
}

impl Bounded for Triangle {
    fn aabb(&self) -> AABB {
        self.aabb
    }
}

impl BHShape for Triangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, BVH>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

fn add_bvh_from_triangles(key: String, triangles: &mut Vec<Triangle>) {
    let mut map = HASHMAP.lock().unwrap();
    map.insert(key, BVH::build(triangles));
}

// -------------------------------------------------------------------------------------------------
// Interface test code starts from here ------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

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

// -------------------------------------------------------------------------------------------------
// Unit tests --------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

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
