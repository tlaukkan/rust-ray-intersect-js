#[macro_use]
extern crate lazy_static;

use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
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

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, Mesh>> = { Mutex::new(HashMap::new()) };
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
    result: &mut IntersectResult,
) -> bool {
    let map = HASHMAP.lock().unwrap();
    let key = mesh_id.to_string();
    if !map.contains_key(&key) {
        return false;
    }

    let mesh: &Mesh = map.get(&key).unwrap();

    let origin = Point3::new(origin_x, origin_y, origin_z);
    let direction = Vector3::new(direction_x, direction_y, direction_z);
    let ray = Ray::new(origin, direction);

    let hits = &mesh.bvh.traverse(&ray, &mesh.triangles);

    result.distance = f32::INFINITY;
    for triangle in hits {
        let candidate = ray.intersects_triangle(&triangle.a, &triangle.b, &triangle.c);
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

pub struct Mesh {
    pub mesh_id: String,
    pub bvh: BVH,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(mesh_id: String, indices: Vec<u32>, positions: Vec<f32>) -> Mesh {
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut index: u32 = 0;

        for i in (0..indices.len()).step_by(3) {
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

        Mesh {
            mesh_id,
            bvh,
            triangles,
        }
    }
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
            index,
            a,
            b,
            c,
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

        let mut result = IntersectResult {
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
