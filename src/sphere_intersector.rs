use crate::model;
use bvh::aabb::Bounded;
use bvh::bvh::BVH;
use bvh::ray::Ray;
use js_sys::Array;
use model::Sphere;
use model::StringArray;
use nalgebra::{distance, Point3, Vector3};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct SphereIntersector {
    spheres: Vec<Sphere>,
    sphere_map: HashMap<String, Sphere>,
    bvh: Option<BVH>,
}

#[wasm_bindgen]
impl SphereIntersector {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SphereIntersector {
        let spheres: Vec<Sphere> = Vec::new();
        SphereIntersector {
            bvh: None,
            spheres,
            sphere_map: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn has(&mut self, id: &str) -> bool {
        let mesh_id_string = id.to_string();
        return self.sphere_map.contains_key(&mesh_id_string);
    }

    #[wasm_bindgen]
    pub fn remove(&mut self, id: &str) -> bool {
        let key = id.to_string();
        if self.sphere_map.contains_key(&key) {
            let sphere = self.sphere_map.get(&key).unwrap();
            // Remove mesh bounding sphere from spheres.
            let sphere_index_result = self.spheres.iter().position(|r| r.id == sphere.id);
            if sphere_index_result.is_some() {
                let sphere_index = sphere_index_result.unwrap();
                self.spheres.remove(sphere_index);
            }
            // Rebuild BVH.
            if self.spheres.len() > 0 {
                self.bvh = Some(BVH::build(&mut self.spheres));
            } else {
                self.bvh = None;
            }
            self.sphere_map.remove(&key);
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn set(&mut self, id: &str, x: f32, y: f32, z: f32, radius: f32) {
        let sphere: Sphere = Sphere {
            id: id.to_string(),
            position: Point3::new(x, y, z),
            radius,
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
        if self.sphere_map.contains_key(&key) {
            self.sphere_map.remove(&key);
        }
        self.sphere_map.insert(id.to_string(), sphere);
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
        let intersecting_mesh_ids = self.internal_intersect(&ray, ray_length);
        intersecting_mesh_ids
            .into_iter()
            .map(JsValue::from)
            .collect::<Array>()
            .unchecked_into::<StringArray>()
    }

    fn internal_intersect(&mut self, ray: &Ray, ray_length: f32) -> Vec<String> {
        let mut intercepting_mesh_ids: Vec<String> = Vec::new();
        match &self.bvh {
            None => return intercepting_mesh_ids,
            Some(bvh) => {
                let hits = bvh.traverse(&ray, &self.spheres);
                for sphere in hits {
                    let sphere_origin = &sphere.position;
                    let sphere_radius = sphere.radius;
                    let ray_origin = &ray.origin;
                    let distance = distance(sphere_origin, ray_origin);

                    // AABB check close enough
                    let aabb = sphere.aabb();
                    if ray.intersects_aabb(&aabb) && distance < ray_length + sphere_radius {
                        intercepting_mesh_ids.push(sphere.id.clone());
                    }
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
        let mut intersector = SphereIntersector::new();

        let id = "1";

        assert_eq!(intersector.has(id), false);
        intersector.set(id, 0.0, 0.0, 0.0, 0.5);
        assert_eq!(intersector.has(id), true);

        let intercepting_ids: Vec<String> = intersector.internal_intersect(
            &Ray::new(Point3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0)),
            0.6,
        );

        assert_eq!(intercepting_ids.len(), 1);
        assert_eq!(intercepting_ids.get(0).unwrap(), id);

        assert_eq!(
            intersector
                .internal_intersect(
                    &Ray::new(Point3::new(0.0, 0.0, 1.0), Vector3::new(0.0, -1.0, 0.0)),
                    0.6,
                )
                .len(),
            0
        );

        assert_eq!(
            intersector
                .internal_intersect(
                    &Ray::new(Point3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, -1.0)),
                    0.6,
                )
                .len(),
            1
        );

        assert_eq!(intersector.remove(&id), true);
        assert_eq!(intersector.has(&id), false);
    }
}
