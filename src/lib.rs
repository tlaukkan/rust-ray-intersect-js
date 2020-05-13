use std::panic;

use bvh::ray::Ray;
use js_sys::Array;
use nalgebra::{Point3, Vector3};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::mesh_intersector::MeshIntersector;
use crate::sphere_intersector::SphereIntersector;

pub mod mesh_intersector;
pub mod model;
pub mod sphere_intersector;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<IntersectResult>")]
    pub type IntersectResultArray;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<String>")]
    pub type StringArray;
}

#[wasm_bindgen]
pub struct MeshIntersectorJS {
    intersector: MeshIntersector,
}

#[wasm_bindgen]
impl MeshIntersectorJS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MeshIntersectorJS {
        MeshIntersectorJS {
            intersector: MeshIntersector::new(),
        }
    }

    #[wasm_bindgen]
    pub fn has(&mut self, mesh_id: &str) -> bool {
        return self.intersector.has(mesh_id);
    }

    #[wasm_bindgen]
    pub fn remove(&mut self, mesh_id: &str) -> bool {
        return self.intersector.remove(mesh_id);
    }

    #[wasm_bindgen]
    pub fn set(&mut self, mesh_id: &str, indices: Vec<u32>, positions: Vec<f32>) -> f32 {
        return self.intersector.set(mesh_id, indices, positions);
    }

    #[wasm_bindgen]
    pub fn intersect(
        &mut self,
        origin_x: f32,
        origin_y: f32,
        origin_z: f32,
        direction_x: f32,
        direction_y: f32,
        direction_z: f32,
        mesh_id: &str,
    ) -> IntersectResultArray {
        let origin = Point3::new(origin_x, origin_y, origin_z);
        let direction = Vector3::new(direction_x, direction_y, direction_z);
        let ray = Ray::new(origin, direction);
        self.intersector
            .intersect(&ray, mesh_id)
            .into_iter()
            .map(JsValue::from)
            .collect::<Array>()
            .unchecked_into::<IntersectResultArray>()
    }
}

#[wasm_bindgen]
pub struct SphereIntersectorJS {
    intersector: SphereIntersector,
}

#[wasm_bindgen]
impl SphereIntersectorJS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SphereIntersectorJS {
        SphereIntersectorJS {
            intersector: SphereIntersector::new(),
        }
    }

    #[wasm_bindgen]
    pub fn has(&mut self, id: &str) -> bool {
        return self.intersector.has(id);
    }

    #[wasm_bindgen]
    pub fn remove(&mut self, id: &str) -> bool {
        return self.intersector.remove(id);
    }

    #[wasm_bindgen]
    pub fn set(&mut self, id: &str, x: f32, y: f32, z: f32, radius: f32) {
        self.intersector.set(id, x, y, z, radius);
    }

    #[wasm_bindgen]
    pub fn intersect(
        &mut self,
        origin_x: f32,
        origin_y: f32,
        origin_z: f32,
        direction_x: f32,
        direction_y: f32,
        direction_z: f32,
        ray_length: f32,
    ) -> StringArray {
        let origin = Point3::new(origin_x, origin_y, origin_z);
        let direction = Vector3::new(direction_x, direction_y, direction_z);
        let ray = Ray::new(origin, direction);
        return self
            .intersector
            .intersect(&ray, ray_length)
            .into_iter()
            .map(JsValue::from)
            .collect::<Array>()
            .unchecked_into::<StringArray>();
    }
}
