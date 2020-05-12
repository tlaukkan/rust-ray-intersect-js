#[macro_use]
extern crate lazy_static;

mod mesh_bvh;
pub mod model;
use model::Mesh;

use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::ray::Ray;
use js_sys::Array;
use nalgebra::{Point3, Vector3};
use std::collections::HashMap;
use std::panic;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, Mesh>> = Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub fn has_mesh(mesh_id: &str) -> bool {
    HASHMAP.lock().unwrap().contains_key(&mesh_id.to_string())
}

#[wasm_bindgen]
pub fn set_mesh(mesh_id: &str, indices: js_sys::Uint32Array, positions: js_sys::Float32Array) {
    _set_mesh(mesh_id, indices.to_vec(), positions.to_vec())
}

pub fn _set_mesh(mesh_id: &str, indices: Vec<u32>, positions: Vec<f32>) {
    let result =
        panic::catch_unwind(|| Mesh::new(mesh_id.to_string(), 0.0, 0.0, 0.0, indices, positions))
            .ok();

    match result {
        Some(value) => {
            let mut map = HASHMAP.lock().unwrap();
            let key = mesh_id.to_string();
            if map.contains_key(&key) {
                map.remove(&key);
            }
            map.insert(mesh_id.to_string(), value);
        }
        None => {
            panic!("Error in mesh triangles. Most likely no valid triangles.");
        }
    }
}

#[wasm_bindgen]
pub fn remove_mesh(mesh_id: &str) -> bool {
    let mut map = HASHMAP.lock().unwrap();
    let key = mesh_id.to_string();
    if map.contains_key(&key) {
        map.remove(&key);
        true
    } else {
        false
    }
}

pub fn _ray_intersect(
    ray: Ray,
    mesh_id: &str,
    mut intercepts: Vec<IntersectResult>,
) -> Vec<IntersectResult> {
    let map = HASHMAP.lock().unwrap();
    let key = mesh_id.to_string();
    if map.contains_key(&key) {
        let mesh: &Mesh = map.get(&key).unwrap();

        let hits = mesh.bvh.traverse(&ray, &mesh.triangles);

        for triangle in hits {
            let candidate = ray.intersects_triangle(&triangle.a, &triangle.b, &triangle.c);
            if candidate.distance != core::f32::INFINITY {
                let mut result: IntersectResult = IntersectResult::new();
                result.hit = true;
                result.distance = candidate.distance;
                result.u = candidate.u;
                result.v = candidate.v;
                result.triangle_index = triangle.index;

                intercepts.push(result);
            } else {
                let inverse_candidate =
                    ray.intersects_triangle(&triangle.c, &triangle.b, &triangle.a);
                if inverse_candidate.distance != core::f32::INFINITY {
                    let mut result: IntersectResult = IntersectResult::new();
                    result.hit = true;
                    result.distance = inverse_candidate.distance;
                    result.u = inverse_candidate.u;
                    result.v = inverse_candidate.v;
                    result.triangle_index = triangle.index;

                    intercepts.push(result);
                }
            }
        }
    }

    intercepts.sort_by(|a, b| (a.distance).partial_cmp(&b.distance).unwrap());
    intercepts
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<IntersectResult>")]
    pub type IntersectResultArray;
}

#[wasm_bindgen]
pub fn ray_intersect(
    mesh_id: &str,
    origin_x: f32,
    origin_y: f32,
    origin_z: f32,
    direction_x: f32,
    direction_y: f32,
    direction_z: f32,
) -> IntersectResultArray {
    let mut intercepts: Vec<IntersectResult> = vec![];

    let origin = Point3::new(origin_x, origin_y, origin_z);
    let direction = Vector3::new(direction_x, direction_y, direction_z);
    let ray = Ray::new(origin, direction);
    intercepts = _ray_intersect(ray, mesh_id, intercepts);

    intercepts
        .into_iter()
        .map(JsValue::from)
        .collect::<Array>()
        .unchecked_into::<IntersectResultArray>()
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct IntersectResult {
    pub hit: bool,
    pub triangle_index: u32,
    pub u: f32,
    pub v: f32,
    pub distance: f32,
}

#[wasm_bindgen]
impl IntersectResult {
    #[wasm_bindgen(constructor)]
    pub fn new() -> IntersectResult {
        IntersectResult {
            hit: false,
            triangle_index: 0,
            u: 0.0,
            v: 0.0,
            distance: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ray_intersect() {
        let mesh_id = "test";
        let indices: Vec<u32> = vec![
            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16,
            17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
        ];
        let positions: Vec<f32> = vec![
            0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, -0.5,
            0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5,
            -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5,
            -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5,
            0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, 0.5,
        ];

        assert_eq!(has_mesh(mesh_id), false);

        _set_mesh(mesh_id, indices, positions);

        assert_eq!(has_mesh(mesh_id), true);

        let mut intercepts: Vec<IntersectResult> = vec![];

        let origin = Point3::new(0.0, 1.0, 0.0);
        let direction = Vector3::new(0.0, -1.0, 0.0);
        let ray = Ray::new(origin, direction);

        intercepts = _ray_intersect(ray, mesh_id, intercepts);

        assert_eq!(intercepts.len(), 4);
        assert_eq!(intercepts[0].hit, true);
        assert_eq!(intercepts[0].distance, 0.5);
        assert_eq!(remove_mesh(mesh_id), true);
        assert_eq!(has_mesh(mesh_id), false);
    }
}
