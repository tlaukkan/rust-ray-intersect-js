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
pub struct Result {
    pub hit: bool,
    pub triangle_index: u32,
    pub u: f32,
    pub v: f32,
    pub distance: f32,
}

pub struct Mesh {
    pub bvh: BVH,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(mesh_id: String, indices: Vec<u32>, positions: Vec<f32>) -> Mesh {
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut index: u32 = 0;
        for i in (0..indices.len()).step_by(3) {
            let ai = indices[i];
            let bi = indices[i + 1];
            let ci = indices[i + 2];
            triangles.push(Triangle::new(
                index,
                Point3::new(
                    positions[indices[i] as usize * 3],
                    positions[indices[i] as usize * 3 + 1],
                    positions[indices[i] as usize * 3 + 2],
                ),
                Point3::new(
                    positions[indices[i + 1] as usize * 3],
                    positions[indices[i + 1] as usize * 3 + 1],
                    positions[indices[i + 1] as usize * 3 + 2],
                ),
                Point3::new(
                    positions[indices[i + 2] as usize],
                    positions[indices[i + 2] as usize * 3 + 1],
                    positions[indices[i + 2] as usize * 3 + 2],
                ),
            ));
            index = index + 1;
        }

        let bvh: BVH = BVH::build(&mut triangles);

        Mesh { bvh, triangles }
    }
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, Mesh>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

#[wasm_bindgen]
pub fn has_mesh(mesh_id: &str) -> bool {
    return HASHMAP.lock().unwrap().contains_key(&mesh_id.to_string());
}

#[wasm_bindgen]
pub fn set_mesh(mesh_id: &str, indices: js_sys::Uint32Array, positions: js_sys::Float32Array) {
    _set_mesh(mesh_id, indices.to_vec(), positions.to_vec())
}

fn _set_mesh(mesh_id: &str, indices: Vec<u32>, positions: Vec<f32>) {
    let mut map = HASHMAP.lock().unwrap();
    let key = mesh_id.to_string();
    if map.contains_key(&key) {
        map.remove(&key);
    }
    map.insert(
        mesh_id.to_string(),
        Mesh::new(mesh_id.to_string(), indices, positions),
    );
}

#[wasm_bindgen]
pub fn remove_mesh(mesh_id: &str) -> bool {
    let mut map = HASHMAP.lock().unwrap();
    let key = mesh_id.to_string();
    return if map.contains_key(&key) {
        map.remove(&key);
        true
    } else {
        false
    };
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
    result: &mut Result,
) -> bool {
    let map = HASHMAP.lock().unwrap();
    let key = mesh_id.to_string();
    if !map.contains_key(&key) {
        return true;
    }

    let mesh: &Mesh = map.get(&key).unwrap();

    let origin = Point3::new(origin_x, origin_y, origin_z);
    let direction = Vector3::new(direction_x, direction_y, direction_z);
    let ray = Ray::new(origin, direction);

    println!("tris: {}", mesh.triangles.len());

    let hits = &mesh.bvh.traverse(&ray, &mesh.triangles);
    println!("hits: {}", hits.len());

    result.distance = f32::INFINITY;
    for triangle in hits {
        println!("candidate tri a x: {}", &triangle.a.coords[0]);
        println!("candidate tri a y: {}", &triangle.a.coords[1]);
        println!("candidate tri a z: {}", &triangle.a.coords[2]);

        println!("candidate tri b x: {}", &triangle.b.coords[0]);
        println!("candidate tri b y: {}", &triangle.b.coords[1]);
        println!("candidate tri b z: {}", &triangle.b.coords[2]);

        println!("candidate tri c x: {}", &triangle.c.coords[0]);
        println!("candidate tri c y: {}", &triangle.c.coords[1]);
        println!("candidate tri c z: {}", &triangle.c.coords[2]);

        let candidate = ray.intersects_triangle(&triangle.a, &triangle.b, &triangle.c);
        println!("candidate distance: {}", candidate.distance);
        if candidate.distance < result.distance {
            result.hit = true;
            result.distance = candidate.distance;
            result.u = candidate.u;
            result.v = candidate.v;
            result.triangle_index = triangle.index;
        }
    }

    return result.hit;
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub struct Triangle {
    pub index: u32,
    pub a: Point3<f32>,
    pub b: Point3<f32>,
    pub c: Point3<f32>,
    aabb: AABB,
    node_index: usize,
}

impl Triangle {
    pub fn new(index: u32, a: Point3<f32>, b: Point3<f32>, c: Point3<f32>) -> Triangle {
        Triangle {
            index: index,
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
    fn test_ray_intersect() {
        let mesh_id = "test";
        let indices: Vec<u32> = vec![0, 1, 2];
        let positions: Vec<f32> = vec![0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 4.0, 0.0];

        assert_eq!(has_mesh(mesh_id), false);

        _set_mesh(mesh_id, indices, positions);

        assert_eq!(has_mesh(mesh_id), true);

        let mut result = Result {
            hit: false,
            triangle_index: 0,
            u: 0.0,
            v: 0.0,
            distance: 0.0,
        };
        assert_eq!(
            ray_intersect(mesh_id, 0.5, 0.5, 0.5, 0.0, 0.0, -1.0, &mut result),
            true
        );

        assert_eq!(result.hit, true);
        assert_eq!(result.triangle_index, 0);
        assert_eq!(result.distance, 0.5);

        assert_eq!(remove_mesh(mesh_id), true);

        assert_eq!(has_mesh(mesh_id), false);
    }
}
