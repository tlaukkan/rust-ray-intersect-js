use crate::model;
use bvh::ray::Ray;
use js_sys::Array;
use model::IntersectResult;
use model::IntersectResultArray;
use model::Mesh;
use nalgebra::{Point3, Vector3};
use std::collections::HashMap;
use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct MeshIntersector {
    meshes: HashMap<String, Mesh>,
}

#[wasm_bindgen]
impl MeshIntersector {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MeshIntersector {
        MeshIntersector {
            meshes: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn has(&mut self, mesh_id: &str) -> bool {
        let mesh_id_string = mesh_id.to_string();
        return self.meshes.contains_key(&mesh_id_string);
    }

    #[wasm_bindgen]
    pub fn remove(&mut self, mesh_id: &str) -> bool {
        let key = mesh_id.to_string();
        if self.meshes.contains_key(&key) {
            self.meshes.remove(&key);
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn set(&mut self, mesh_id: &str, indices: Vec<u32>, positions: Vec<f32>) -> f32 {
        // Attempt to build mesh.
        let result =
            panic::catch_unwind(|| Mesh::new(mesh_id.to_string(), indices, positions)).ok();

        match result {
            Some(mesh) => {
                // Store mesh.
                let key = mesh_id.to_string();
                if self.meshes.contains_key(&key) {
                    self.meshes.remove(&key);
                }
                let radius = mesh.radius;
                self.meshes.insert(mesh_id.to_string(), mesh);
                return radius;
            }
            None => {
                panic!("Error in mesh triangles. Most likely no valid triangles.");
            }
        }
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
        let intersects = self.internal_intersect(&ray, mesh_id);
        intersects
            .into_iter()
            .map(JsValue::from)
            .collect::<Array>()
            .unchecked_into::<IntersectResultArray>()
    }

    fn internal_intersect(&mut self, local_ray: &Ray, mesh_id: &str) -> Vec<IntersectResult> {
        let mut intercepts: Vec<IntersectResult> = vec![];
        if self.meshes.contains_key(mesh_id) {
            let mesh: &Mesh = self.meshes.get(mesh_id).unwrap();
            let hits = mesh.bvh.traverse(&local_ray, &mesh.triangles);

            for triangle in hits {
                let candidate =
                    local_ray.intersects_triangle(&triangle.a, &triangle.b, &triangle.c);
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
                        local_ray.intersects_triangle(&triangle.c, &triangle.b, &triangle.a);
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
        return intercepts;
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ray_intersect() {
        let mut intersector = MeshIntersector::new();

        let mesh_id = "1";
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

        assert_eq!(intersector.has(mesh_id), false);
        assert_eq!(intersector.set(mesh_id, indices, positions), 0.8660254);
        assert_eq!(intersector.has(mesh_id), true);

        let intercepts = intersector.internal_intersect(
            &Ray::new(Point3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0)),
            &mesh_id,
        );
        assert_eq!(intercepts.len(), 4);
        assert_eq!(intercepts[0].hit, true);
        assert_eq!(intercepts[0].distance, 0.5);

        assert_eq!(intersector.remove(&mesh_id), true);
        assert_eq!(intersector.has(&mesh_id), false);
    }
}
