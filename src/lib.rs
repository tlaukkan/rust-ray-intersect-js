#[macro_use]
extern crate lazy_static;
extern crate js_sys;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::ray::Ray;
use js_sys;
use nalgebra::{magnitude, Point3, Vector, Vector3};
use std::collections::HashMap;
use std::panic;
use std::sync::Mutex;

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
    return HASHMAP.lock().unwrap().contains_key(&mesh_id.to_string());
}

#[wasm_bindgen]
pub fn set_mesh(mesh_id: &str, indices: js_sys::Uint32Array, positions: js_sys::Float32Array) {
    _set_mesh(mesh_id, indices.to_vec(), positions.to_vec())
}

pub fn _set_mesh(mesh_id: &str, indices: Vec<u32>, positions: Vec<f32>) {
    let result = panic::catch_unwind(|| {
        return Mesh::new(mesh_id.to_string(), indices, positions);
    })
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

fn _ray_intersect(
    ray: Ray,
    mesh: &Mesh,
    mut intercepts: Vec<IntersectResult>,
) -> Vec<IntersectResult> {
    let hits = mesh.bvh.traverse(&ray, &mesh.triangles);

    for triangle in hits {
        println!("trying dis");
        let candidate = ray.intersects_triangle(&triangle.a, &triangle.b, &triangle.c);
        if candidate.distance != core::f32::INFINITY {
            let mut result: IntersectResult = IntersectResult::new();

            result.hit = true;
            result.distance = candidate.distance;
            result.triangle_index = triangle.index;

            intercepts.push(result);
        }
    }
    intercepts.sort_by(|a, b| (a.distance).partial_cmp(&b.distance).unwrap());
    intercepts
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
) -> js_sys::Array {
    let mut intercepts: Vec<IntersectResult> = vec![];
    let map = HASHMAP.lock().unwrap();
    let key = mesh_id.to_string();
    if map.contains_key(&key) {
        let mesh: &Mesh = map.get(&key).unwrap();
        let origin = Point3::new(origin_x, origin_y, origin_z);
        let direction = Vector3::new(direction_x, direction_y, direction_z);
        let ray = Ray::new(origin, direction);
        intercepts = _ray_intersect(ray, mesh, intercepts);
    }

    let mesh: &Mesh = map.get(&key).unwrap();

    let origin = Point3::new(origin_x, origin_y, origin_z);
    let direction = Vector3::new(direction_x, direction_y, direction_z);
    let ray = Ray::new(origin, direction);

    let hits = &mesh.bvh.traverse(&ray, &mesh.triangles);

    result.distance = f32::INFINITY;
    for triangle in hits {
        let candidate = ray.intersects_triangle(&triangle.a, &triangle.b, &triangle.c);
        /*println!(
            "candidate triangle {} at {}",
            triangle.index, candidate.distance
        );*/
        if candidate.distance < result.distance {
            result.hit = true;
            result.distance = candidate.distance;
            result.u = candidate.u;
            result.v = candidate.v;
            result.triangle_index = triangle.index;
        } else {
            let inverseCandidate = ray.intersects_triangle(&triangle.c, &triangle.b, &triangle.a);
            if inverseCandidate.distance < result.distance {
                result.hit = true;
                result.distance = inverseCandidate.distance;
                result.u = inverseCandidate.u;
                result.v = inverseCandidate.v;
                result.triangle_index = triangle.index;
            }
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
    pub distance: f32,
}

#[wasm_bindgen]
impl IntersectResult {
    #[wasm_bindgen(constructor)]
    pub fn new() -> IntersectResult {
        IntersectResult {
            hit: false,
            triangle_index: 0,
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

        if indices.len() == 0 {
            panic!("No triangles.");
        }

        for i in (0..indices.len()).step_by(3) {
            let triangle = Triangle::new(
                tri_index as u32,
                Point3::new(
                    positions[indices[i + 0] as usize * 3 + 0],
                    positions[indices[i + 0] as usize * 3 + 1],
                    positions[indices[i + 0] as usize * 3 + 2],
                ),
                Point3::new(
                    positions[indices[i + 1] as usize * 3 + 0],
                    positions[indices[i + 1] as usize * 3 + 1],
                    positions[indices[i + 1] as usize * 3 + 2],
                ),
                Point3::new(
                    positions[indices[i + 2] as usize * 3 + 0],
                    positions[indices[i + 2] as usize * 3 + 1],
                    positions[indices[i + 2] as usize * 3 + 2],
                ),
            );

            // Check for vectors with zero surface area.
            let ab = Vector3::new(
                triangle.b[0] - triangle.a[0],
                triangle.b[1] - triangle.a[1],
                triangle.b[2] - triangle.a[2],
            );
            let ac = Vector3::new(
                triangle.c[0] - triangle.a[0],
                triangle.c[1] - triangle.a[1],
                triangle.c[2] - triangle.a[2],
            );
            if ab.magnitude() == 0.0 {
                Mesh::log_ignored_triangle(&triangle);
                continue;
            }
            if ac.magnitude() == 0.0 {
                Mesh::log_ignored_triangle(&triangle);
                continue;
            }
            let abn = ab.normalize();
            let acn = ac.normalize();
            let dot = abn.dot(&acn);

            if dot == 1.0 || dot == -1.0 {
                Mesh::log_ignored_triangle(&triangle);
                continue;
            }

            triangles.push(triangle);
        }

        if index == 0 {
            panic!("All triangles have zero surface area.");
        }

        let bvh: BVH = BVH::build(&mut triangles);

        Mesh {
            mesh_id,
            bvh,
            triangles,
        }
    }

    fn log_ignored_triangle(triangle: &Triangle) {
        println!(
            "Ignored triangle with zero surface area: ({},{},{}) ({},{},{}) ({},{},{}) ",
            triangle.a[0],
            triangle.a[1],
            triangle.a[2],
            triangle.b[0],
            triangle.b[1],
            triangle.b[2],
            triangle.c[0],
            triangle.c[1],
            triangle.c[2],
        );
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
        {
            // scoping brackets to allow map to go out of scope
            let map = HASHMAP.lock().unwrap();
            let key = mesh_id.to_string();
            let mesh: &Mesh = map.get(&key).unwrap();
            let origin = Point3::new(0.0, 1.0, 0.0);
            let direction = Vector3::new(0.0, -1.0, 0.0);
            let ray = Ray::new(origin, direction);

            intercepts = _ray_intersect(ray, mesh, intercepts);
        }

        assert_eq!(intercepts.len(), 2);
        assert_eq!(intercepts[0].hit, true);
        assert_eq!(intercepts[0].distance, 0.5);
        assert_eq!(remove_mesh(mesh_id), true);
        assert_eq!(has_mesh(mesh_id), false);
    }
}
