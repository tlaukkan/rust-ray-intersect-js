use intersect;
use intersect::{_set_mesh, has_mesh, ray_intersect, remove_mesh, IntersectResult, Triangle};
use nalgebra::{Point3, Vector3};

#[test]
#[should_panic]
fn test_ray_intersect_with_zero_surface_area_triangles_only() {
    let mesh_id = "test";
    let indices: Vec<u32> = vec![0, 0, 0, 0, 0, 1];
    let positions: Vec<f32> = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0];

    for i in (0..indices.len()).step_by(3) {
        let triangle = Triangle::new(
            0,
            Point3::new(
                positions[indices[i + 0] as usize * 3 + 2],
                positions[indices[i + 0] as usize * 3 + 1],
                positions[indices[i + 0] as usize * 3 + 0],
            ),
            Point3::new(
                positions[indices[i + 1] as usize * 3 + 2],
                positions[indices[i + 1] as usize * 3 + 1],
                positions[indices[i + 1] as usize * 3 + 0],
            ),
            Point3::new(
                positions[indices[i + 2] as usize * 3 + 2],
                positions[indices[i + 2] as usize * 3 + 1],
                positions[indices[i + 2] as usize * 3 + 0],
            ),
        );
        println!(
            "triangle #{} ({},{},{}) ({},{},{}) ({},{},{}) ",
            triangle.index,
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
        ray_intersect(mesh_id, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, &mut result),
        true
    );

    assert_eq!(result.hit, true);
    assert_eq!(result.distance, 0.5);

    assert_eq!(remove_mesh(mesh_id), true);

    assert_eq!(has_mesh(mesh_id), false);
}

#[test]
fn test_ray_intersect_with_zero_surface_area_triangles() {
    let mesh_id = "test";
    let indices: Vec<u32> = vec![
        0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 17,
        18, 16, 18, 19, 20, 21, 22, 20, 22, 23, 24, 24, 24, 24, 24, 25,
    ];
    let positions: Vec<f32> = vec![
        0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, -0.5, 0.5,
        -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5,
        0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5,
        0.5, 0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5, -0.5, -0.5, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    ];

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
        ray_intersect(mesh_id, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, &mut result),
        true
    );

    assert_eq!(result.hit, true);
    assert_eq!(result.distance, 0.5);

    assert_eq!(remove_mesh(mesh_id), true);

    assert_eq!(has_mesh(mesh_id), false);
}
