use crate::model;
use bvh::ray::Ray;
use model::IntersectResult;
use model::Mesh;
use std::collections::HashMap;
use std::panic;

pub struct MeshIntersector {
    meshes: HashMap<String, Mesh>,
}

impl MeshIntersector {
    pub fn new() -> MeshIntersector {
        MeshIntersector {
            meshes: HashMap::new(),
        }
    }

    pub fn has(&mut self, mesh_id: &str) -> bool {
        let mesh_id_string = mesh_id.to_string();
        return self.meshes.contains_key(&mesh_id_string);
    }

    pub fn remove(&mut self, mesh_id: &str) -> bool {
        let key = mesh_id.to_string();
        if self.meshes.contains_key(&key) {
            self.meshes.remove(&key);
            true
        } else {
            false
        }
    }

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

    pub fn intersect(
        &mut self,
        mesh_id: &str,
        local_ray: &Ray,
        ray_length: f32,
    ) -> Vec<IntersectResult> {
        let mut intercepts: Vec<IntersectResult> = vec![];
        if self.meshes.contains_key(mesh_id) {
            let mesh: &Mesh = self.meshes.get(mesh_id).unwrap();
            let hits = mesh.bvh.traverse(&local_ray, &mesh.triangles);

            for triangle in hits {
                let candidate =
                    local_ray.intersects_triangle(&triangle.a, &triangle.b, &triangle.c);
                if candidate.distance != core::f32::INFINITY && candidate.distance < ray_length {
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
                    if inverse_candidate.distance != core::f32::INFINITY
                        && inverse_candidate.distance < ray_length
                    {
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
    use nalgebra::{Point3, Vector3};

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

        let intercepts = intersector.intersect(
            &mesh_id,
            &Ray::new(Point3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0)),
            1.0,
        );
        assert_eq!(intercepts.len(), 2);
        assert_eq!(intercepts[0].hit, true);
        assert_eq!(intercepts[0].distance, 0.5);

        assert_eq!(
            intersector
                .intersect(
                    &mesh_id,
                    &Ray::new(Point3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0)),
                    2.0,
                )
                .len(),
            4
        );

        assert_eq!(intersector.remove(&mesh_id), true);
        assert_eq!(intersector.has(&mesh_id), false);
    }
}
