use crate::{model, IntersectResult};
use bvh::bvh::BVH;
use bvh::ray::Ray;
use model::Mesh;
use model::Sphere;
use nalgebra::{distance, Point3, Vector3};
use std::collections::HashMap;
use std::panic;
use std::sync::Mutex;

struct Container {
    pub spheres: Vec<Sphere>,
    pub meshes: HashMap<String, Mesh>,
    pub bvh: Option<BVH>,
}

impl Container {
    pub fn new() -> Container {
        let mut spheres: Vec<Sphere> = Vec::new();
        Container {
            bvh: None,
            spheres,
            meshes: HashMap::new(),
        }
    }
}

lazy_static! {
    static ref DYNAMIC: Mutex<Container> = Mutex::new(Container::new());
    static ref STATIC: Mutex<Container> = Mutex::new(Container::new());
}

pub fn internal_has_mesh(mesh_id: &str, dynamic: bool) -> bool {
    let mesh_id_string = mesh_id.to_string();
    let mut container = if dynamic {
        DYNAMIC.lock().unwrap()
    } else {
        STATIC.lock().unwrap()
    };
    return container.meshes.contains_key(&mesh_id_string);
}

pub fn internal_remove_mesh(mesh_id: &str, dynamic: bool) -> bool {
    let mut container = if dynamic {
        DYNAMIC.lock().unwrap()
    } else {
        STATIC.lock().unwrap()
    };

    let key = mesh_id.to_string();
    if container.meshes.contains_key(&key) {
        let mesh = container.meshes.get(&key).unwrap();
        // Remove mesh bounding sphere from spheres.
        let sphere_index_result = container
            .spheres
            .iter()
            .position(|r| r.id == mesh.sphere.id);
        if sphere_index_result.is_some() {
            let sphere_index = sphere_index_result.unwrap();
            container.spheres.remove(sphere_index);
        }
        // Rebuild BVH.
        if (container.spheres.len() > 0) {
            container.bvh = Some(BVH::build(&mut container.spheres));
        } else {
            container.bvh = None;
        }
        container.meshes.remove(&key);
        true
    } else {
        false
    }
}

pub fn internal_set_mesh(
    mesh_id: &str,
    x: f32,
    y: f32,
    z: f32,
    indices: Vec<u32>,
    positions: Vec<f32>,
    dynamic: bool,
) {
    // Attempt to build mesh.
    let result =
        panic::catch_unwind(|| Mesh::new(mesh_id.to_string(), x, y, z, indices, positions)).ok();

    match result {
        Some(mesh) => {
            let mut container = if dynamic {
                DYNAMIC.lock().unwrap()
            } else {
                STATIC.lock().unwrap()
            };

            // Update bounding sphere BVH.
            let sphere = mesh.sphere.clone();
            let spheres = &mut container.spheres;
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
            container.bvh = Some(BVH::build(&mut container.spheres));

            // Store mesh.
            let key = mesh_id.to_string();
            if container.meshes.contains_key(&key) {
                container.meshes.remove(&key);
            }
            container.meshes.insert(mesh_id.to_string(), mesh);
        }
        None => {
            panic!("Error in mesh triangles. Most likely no valid triangles.");
        }
    }
}

pub fn internal_mesh_ray_intersect(
    ray: &Ray,
    ray_length: f32,
    intercepting_mesh_ids: &mut Vec<String>,
) {
    mesh_ray_intersect(ray, ray_length, intercepting_mesh_ids, false);
    mesh_ray_intersect(ray, ray_length, intercepting_mesh_ids, true);
}

fn mesh_ray_intersect(
    ray: &Ray,
    ray_length: f32,
    intercepting_mesh_ids: &mut Vec<String>,
    dynamic: bool,
) {
    let container = if dynamic {
        DYNAMIC.lock().unwrap()
    } else {
        STATIC.lock().unwrap()
    };
    match &container.bvh {
        None => return,
        Some(bvh) => {
            let hits = bvh.traverse(&ray, &container.spheres);
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

fn internal_triangle_ray_intersect(
    local_ray: &Ray,
    mesh_id: &str,
    intercepts: &mut Vec<IntersectResult>,
) {
    let dynamic_container = DYNAMIC.lock().unwrap();
    if dynamic_container.meshes.contains_key(mesh_id) {
        let mesh: &Mesh = dynamic_container.meshes.get(mesh_id).unwrap();
        triangle_ray_intersect(local_ray, mesh, intercepts);
    }
    let static_container = STATIC.lock().unwrap();
    if static_container.meshes.contains_key(mesh_id) {
        let mesh: &Mesh = static_container.meshes.get(mesh_id).unwrap();
        triangle_ray_intersect(local_ray, mesh, intercepts);
    }
    intercepts.sort_by(|a, b| (a.distance).partial_cmp(&b.distance).unwrap());
}

fn triangle_ray_intersect(local_ray: &Ray, mesh: &Mesh, intercepts: &mut Vec<IntersectResult>) {
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
        /*let mut spheres: Vec<Sphere> = Vec::new();
        Container {
            bvh: BVH::build(&mut spheres),
            spheres,
            meshes: HashMap::new(),
        };*/

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

        assert_eq!(internal_has_mesh(mesh_id, true), false);

        internal_set_mesh(mesh_id, 0.0, 0.0, 0.0, indices, positions, true);

        assert_eq!(internal_has_mesh(mesh_id, true), true);

        let mut intercepts: Vec<IntersectResult> = vec![];

        let origin = Point3::new(0.0, 1.0, 0.0);
        let direction = Vector3::new(0.0, -1.0, 0.0);
        let ray = Ray::new(origin, direction);

        internal_triangle_ray_intersect(&ray, &mesh_id, &mut intercepts);

        assert_eq!(intercepts.len(), 4);
        assert_eq!(intercepts[0].hit, true);
        assert_eq!(intercepts[0].distance, 0.5);
        assert_eq!(internal_remove_mesh(&mesh_id, true), true);
        assert_eq!(internal_has_mesh(&mesh_id, true), false);
    }
}
