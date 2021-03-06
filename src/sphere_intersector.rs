use crate::model;
use bvh::aabb::Bounded;
use bvh::bvh::BVH;
use bvh::ray::Ray;
use model::Sphere;
use nalgebra::{distance, Point3};
use std::collections::{HashMap, HashSet};

pub struct SphereIntersector {
    spheres: Vec<Sphere>,
    sphere_map: HashMap<String, Sphere>,
    bvh: Option<BVH>,
    changed_sphere_indices: HashSet<usize>,
}

impl SphereIntersector {
    pub fn new() -> SphereIntersector {
        let spheres: Vec<Sphere> = Vec::new();
        SphereIntersector {
            bvh: None,
            spheres,
            sphere_map: HashMap::new(),
            changed_sphere_indices: HashSet::new(),
        }
    }

    pub fn has(&mut self, id: &str) -> bool {
        let mesh_id_string = id.to_string();
        return self.sphere_map.contains_key(&mesh_id_string);
    }

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
            self.sphere_map.remove(&key);
            true
        } else {
            false
        }
    }

    pub fn add(&mut self, id: &str, x: f32, y: f32, z: f32, radius: f32) {
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

        // Store mesh.
        let key = id.to_string();
        if self.sphere_map.contains_key(&key) {
            self.sphere_map.remove(&key);
        }
        self.sphere_map.insert(id.to_string(), sphere);
    }

    /// Builds or rebuilds BVH.
    pub fn build(&mut self) {
        if self.spheres.len() > 0 {
            self.bvh = Some(BVH::build(&mut self.spheres));
        } else {
            self.bvh = None;
        }
        self.changed_sphere_indices.clear();
    }

    pub fn update(&mut self, id: &str, x: f32, y: f32, z: f32, radius: f32) -> bool {
        let id = id.to_string();
        let spheres = &mut self.spheres;
        let sphere_index_result = spheres.iter().position(|r| r.id == id);
        if sphere_index_result.is_some() {
            // Updating existing sphere.
            let sphere_index = sphere_index_result.unwrap();
            let mut existing_sphere = spheres.get_mut(sphere_index).unwrap();
            existing_sphere.position.x = x;
            existing_sphere.position.y = y;
            existing_sphere.position.z = z;
            existing_sphere.radius = radius;
            self.changed_sphere_indices.insert(sphere_index);
            return true;
        } else {
            return false;
        }
    }

    /// Optimizes BVH according to updated spheres.
    pub fn optimize(&mut self) {
        if self.bvh.is_some() {
            self.bvh
                .as_mut()
                .unwrap()
                .optimize(&self.changed_sphere_indices, &self.spheres);
        }
        self.changed_sphere_indices.clear();
    }

    pub fn intersect(&mut self, ray: &Ray, ray_length: f32) -> Vec<String> {
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
    use nalgebra::{Point3, Vector3};

    #[test]
    fn test_ray_intersect() {
        let mut intersector = SphereIntersector::new();

        let id = "1";

        assert_eq!(intersector.has(id), false);
        intersector.add(id, 0.0, 0.0, 0.0, 0.5);
        assert_eq!(intersector.has(id), true);

        intersector.build();

        let intercepting_ids: Vec<String> = intersector.intersect(
            &Ray::new(Point3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0)),
            0.6,
        );

        assert_eq!(intercepting_ids.len(), 1);
        assert_eq!(intercepting_ids.get(0).unwrap(), id);

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

        intersector.update(id, 5.0, 0.0, 0.0, 0.5);

        intersector.build();

        assert_eq!(
            intersector
                .intersect(
                    &Ray::new(Point3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0)),
                    0.6,
                )
                .len(),
            0
        );
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
            0
        );

        assert_eq!(
            intersector
                .intersect(
                    &Ray::new(Point3::new(5.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0)),
                    0.6,
                )
                .len(),
            1
        );
        assert_eq!(
            intersector
                .intersect(
                    &Ray::new(Point3::new(5.0, 0.0, 1.0), Vector3::new(0.0, -1.0, 0.0)),
                    0.6,
                )
                .len(),
            0
        );
        assert_eq!(
            intersector
                .intersect(
                    &Ray::new(Point3::new(5.0, 0.0, 1.0), Vector3::new(0.0, 0.0, -1.0)),
                    0.6,
                )
                .len(),
            1
        );

        assert_eq!(intersector.remove(&id), true);
        intersector.build();

        assert_eq!(intersector.has(&id), false);
    }
}
