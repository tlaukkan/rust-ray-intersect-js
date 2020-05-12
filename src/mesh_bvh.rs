use crate::{model, IntersectResult};
use bvh::bvh::BVH;
use bvh::ray::Ray;
use model::Mesh;
use model::Sphere;
use nalgebra::{distance, Point3, Vector3};
use std::collections::HashMap;
use std::panic;
use std::sync::Mutex;

struct Intersector {
    spheres: Vec<Sphere>,
    meshes: HashMap<String, Mesh>,
    bvh: Option<BVH>,
}

impl Intersector {
    pub fn new() -> Intersector {
        let mut spheres: Vec<Sphere> = Vec::new();
        Intersector {
            bvh: None,
            spheres,
            meshes: HashMap::new(),
        }
    }

    pub fn has_mesh(&mut self, mesh_id: &str) -> bool {
        let mesh_id_string = mesh_id.to_string();
        return self.meshes.contains_key(&mesh_id_string);
    }

    pub fn remove_mesh(&mut self, mesh_id: &str) -> bool {
        let key = mesh_id.to_string();
        if self.meshes.contains_key(&key) {
            let mesh = self.meshes.get(&key).unwrap();
            // Remove mesh bounding sphere from spheres.
            let sphere_index_result = self.spheres.iter().position(|r| r.id == mesh.sphere.id);
            if sphere_index_result.is_some() {
                let sphere_index = sphere_index_result.unwrap();
                self.spheres.remove(sphere_index);
            }
            // Rebuild BVH.
            if (self.spheres.len() > 0) {
                self.bvh = Some(BVH::build(&mut self.spheres));
            } else {
                self.bvh = None;
            }
            self.meshes.remove(&key);
            true
        } else {
            false
        }
    }

    pub fn set_mesh(
        &mut self,
        mesh_id: &str,
        x: f32,
        y: f32,
        z: f32,
        indices: Vec<u32>,
        positions: Vec<f32>,
    ) {
        // Attempt to build mesh.
        let result =
            panic::catch_unwind(|| Mesh::new(mesh_id.to_string(), x, y, z, indices, positions))
                .ok();

        match result {
            Some(mesh) => {
                // Update bounding sphere BVH.
                let sphere = mesh.sphere.clone();
                let spheres = &mut self.spheres;
                let sphere_index_result = spheres.iter().position(|r| r.id == sphere.id);
                if sphere_index_result.is_some() {
                    // Updating existing sphere.
                    let sphere_index = sphere_index_result.unwrap();
                    let mut existing_sphere = spheres.get_mut(sphere_index).unwrap();
                    existing_sphere.position = sphere.position;
                    existing_sphere.radius = sphere.radius;
                } else {
                    // Push new sphere.
                    spheres.push(sphere);
                }
                // Rebuild BVH.
                self.bvh = Some(BVH::build(&mut self.spheres));

                // Store mesh.
                let key = mesh_id.to_string();
                if self.meshes.contains_key(&key) {
                    self.meshes.remove(&key);
                }
                self.meshes.insert(mesh_id.to_string(), mesh);
            }
            None => {
                panic!("Error in mesh triangles. Most likely no valid triangles.");
            }
        }
    }

    pub fn intersect_meshes(
        &mut self,
        ray: &Ray,
        ray_length: f32,
        intercepting_mesh_ids: &mut Vec<String>,
    ) {
        match &self.bvh {
            None => return,
            Some(bvh) => {
                let hits = bvh.traverse(&ray, &self.spheres);
                for sphere in hits {
                    let sphere_radius = sphere.radius;
                    let sphere_origin = &sphere.position;
                    let ray_origin = &ray.origin;
                    let distance = distance(sphere_origin, ray_origin);
                    if distance < ray_length + sphere_radius {
                        intercepting_mesh_ids.push(sphere.id.clone());
                    }
                }
            }
        }
    }

    fn intersect_triangles(
        &mut self,
        local_ray: &Ray,
        mesh_id: &str,
        intercepts: &mut Vec<IntersectResult>,
    ) {
        if self.meshes.contains_key(mesh_id) {
            let mesh: &Mesh = self.meshes.get(mesh_id).unwrap();
            internal_intersect_triangles(local_ray, mesh, intercepts);
        }
        intercepts.sort_by(|a, b| (a.distance).partial_cmp(&b.distance).unwrap());
    }
}

fn internal_intersect_triangles(
    local_ray: &Ray,
    mesh: &Mesh,
    intercepts: &mut Vec<IntersectResult>,
) {
    let hits = mesh.bvh.traverse(&local_ray, &mesh.triangles);

    for triangle in hits {
        let candidate = local_ray.intersects_triangle(&triangle.a, &triangle.b, &triangle.c);
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

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ray_intersect() {
        let mut intersector = Intersector::new();

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

        assert_eq!(intersector.has_mesh(mesh_id), false);
        intersector.set_mesh(mesh_id, 0.0, 0.0, 0.0, indices, positions);
        assert_eq!(intersector.has_mesh(mesh_id), true);

        let origin = Point3::new(0.0, 1.0, 0.0);
        let hittingRay = Ray::new(Point3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0));

        let mut intercepts: Vec<IntersectResult> = vec![];
        let mut intercepting_mesh_ids: Vec<String> = vec![];
        intersector.intersect_meshes(&hittingRay, 1.0, &mut intercepting_mesh_ids);
        assert_eq!(intercepting_mesh_ids.len(), 1);
        assert_eq!(intercepting_mesh_ids.get(0).unwrap(), mesh_id);

        intersector.intersect_triangles(&hittingRay, &mesh_id, &mut intercepts);
        assert_eq!(intercepts.len(), 4);
        assert_eq!(intercepts[0].hit, true);
        assert_eq!(intercepts[0].distance, 0.5);

        assert_eq!(intersector.remove_mesh(&mesh_id), true);
        assert_eq!(intersector.has_mesh(&mesh_id), false);
    }
}
