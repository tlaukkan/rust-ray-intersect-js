use crate::model;
use bvh::aabb::Bounded;
use bvh::bvh::BVH;
use bvh::ray::Ray;
use model::Sphere;
use nalgebra::{distance, Point3, Vector3};
use std::collections::HashMap;

struct SphereInterceptor {
    spheres: Vec<Sphere>,
    sphereMap: HashMap<String, Sphere>,
    bvh: Option<BVH>,
}

impl SphereInterceptor {
    pub fn new() -> SphereInterceptor {
        let mut spheres: Vec<Sphere> = Vec::new();
        SphereInterceptor {
            bvh: None,
            spheres,
            sphereMap: HashMap::new(),
        }
    }

    pub fn contains(&mut self, id: &str) -> bool {
        let mesh_id_string = id.to_string();
        return self.sphereMap.contains_key(&mesh_id_string);
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let key = id.to_string();
        if self.sphereMap.contains_key(&key) {
            let sphere = self.sphereMap.get(&key).unwrap();
            // Remove mesh bounding sphere from spheres.
            let sphere_index_result = self.spheres.iter().position(|r| r.id == sphere.id);
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
            self.sphereMap.remove(&key);
            true
        } else {
            false
        }
    }

    pub fn set(&mut self, id: &str, x: f32, y: f32, z: f32, radius: f32) {
        let sphere: Sphere = Sphere {
            id: id.to_string(),
            position: Point3::new(x, y, z),
            radius: radius,
            node_index: 0,
        };

        let spheres = &mut self.spheres;
        let sphere_index_result = spheres.iter().position(|r| r.id == sphere.id);
        if sphere_index_result.is_some() {
            // Updating existing sphere.
            let sphere_index = sphere_index_result.unwrap();
            let mut existing_sphere = spheres.get_mut(sphere_index).unwrap();
            existing_sphere.position = sphere.position.clone();
            existing_sphere.radius = sphere.radius;
        } else {
            // Push new sphere.
            spheres.push(sphere.clone());
        }
        // Rebuild BVH.
        self.bvh = Some(BVH::build(&mut self.spheres));

        // Store mesh.
        let key = id.to_string();
        if self.sphereMap.contains_key(&key) {
            self.sphereMap.remove(&key);
        }
        self.sphereMap.insert(id.to_string(), sphere);
    }

    pub fn intersect(&mut self, ray: &Ray, ray_length: f32) -> Vec<String> {
        let mut intercepting_mesh_ids: Vec<String> = Vec::new();
        match &self.bvh {
            None => return intercepting_mesh_ids,
            Some(bvh) => {
                let hits = bvh.traverse(&ray, &self.spheres);
                for sphere in hits {
                    let sphere_radius = sphere.radius;
                    let sphere_origin = &sphere.position;
                    let ray_origin = &ray.origin;
                    let distance = distance(sphere_origin, ray_origin);

                    // AABB check close enough
                    let aabb = sphere.aabb();
                    if ray.intersects_aabb(&aabb) {
                        intercepting_mesh_ids.push(sphere.id.clone());
                    }
                    /*if distance < ray_length + sphere_radius {
                    }*/
                }
            }
        }
        return intercepting_mesh_ids;
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ray_intersect() {
        let mut intersector = SphereInterceptor::new();

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

        assert_eq!(intersector.contains(mesh_id), false);
        intersector.set(mesh_id, 0.0, 0.0, 0.0, 0.5);
        assert_eq!(intersector.contains(mesh_id), true);

        let mut intercepting_mesh_ids: Vec<String> = intersector.intersect(
            &Ray::new(Point3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0)),
            0.6,
        );

        assert_eq!(intercepting_mesh_ids.len(), 1);
        assert_eq!(intercepting_mesh_ids.get(0).unwrap(), mesh_id);

        assert_eq!(
            intersector
                .intersect(
                    &Ray::new(Point3::new(0.0, 0.0, 1.0), Vector3::new(0.0, -1.0, 0.0)),
                    0.6,
                )
                .len(),
            0
        );

        assert_eq!(
            intersector
                .intersect(
                    &Ray::new(Point3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, -1.0)),
                    0.6,
                )
                .len(),
            1
        );

        assert_eq!(intersector.remove(&mesh_id), true);
        assert_eq!(intersector.contains(&mesh_id), false);
    }
}
