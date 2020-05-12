use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use nalgebra::{Point3, Vector3};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<IntersectResult>")]
    pub type IntersectResultArray;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<String>")]
    pub type StringArray;
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
    pub radius: f32,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(mesh_id: String, indices: Vec<u32>, positions: Vec<f32>) -> Mesh {
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut index: u32 = 0;

        if indices.is_empty() {
            panic!("No triangles.");
        }

        for i in (0..indices.len()).step_by(3) {
            let triangle = Triangle::new(
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
                    positions[indices[i + 2] as usize * 3],
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
                //TODO FLOAT COMPARE IS NOT ADVISED
                Mesh::log_ignored_triangle(&triangle);
                continue;
            }

            triangles.push(triangle);

            index += 1;
        }

        if index == 0 {
            panic!("All triangles have zero surface area.");
        }

        let bvh: BVH = BVH::build(&mut triangles);

        let labb = bvh.nodes[0].child_l_aabb();
        let rabb = bvh.nodes[0].child_r_aabb();
        let mut radius: f32 = get_point_magnitude(&labb.min);
        radius = f32::max(radius, get_point_magnitude(&labb.max));
        radius = f32::max(radius, get_point_magnitude(&rabb.min));
        radius = f32::max(radius, get_point_magnitude(&rabb.max));

        Mesh {
            mesh_id,
            radius,
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

pub struct Sphere {
    pub position: Point3<f32>,
    pub radius: f32,
    pub node_index: usize,
    pub id: String,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        let half_size = Vector3::new(self.radius, self.radius, self.radius);
        let min = &self.position - half_size;
        let max = &self.position + half_size;
        AABB::with_bounds(min, max)
    }
}

impl BHShape for Sphere {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }
    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl Clone for Sphere {
    fn clone(&self) -> Self {
        return Sphere {
            position: Point3::new(self.position[0], self.position[1], self.position[2]),
            radius: self.radius,
            node_index: self.node_index,
            id: self.id.clone(),
        };
    }
}

fn get_point_magnitude(p: &Point3<f32>) -> f32 {
    return Vector3::new(p[0], p[1], p[2]).magnitude();
}
