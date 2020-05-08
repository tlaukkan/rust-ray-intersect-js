import {expect} from 'chai';
import {has_mesh, init_panic_hook, IntersectResult, ray_intersect, remove_mesh, set_mesh} from 'rust-ray-intersect';

import * as BABYLON from 'babylonjs';
// Force loading loaders.
import * as BABYLON_LOADERS from 'babylonjs-loaders';
import GLTFLoaderCoordinateSystemMode = BABYLON_LOADERS.GLTFLoaderCoordinateSystemMode;
import VertexBuffer = BABYLON.VertexBuffer;
import Mesh = BABYLON.Mesh;
import Vector3 = BABYLON.Vector3;

const gltfLoaderCoordinateSystemMode = GLTFLoaderCoordinateSystemMode;

const get_3d_position = (origin: Vector3, direction: Vector3, distance: number, meshId: string): Vector3[]|null => {
    let result = ray_intersect(meshId, origin.x, origin.y, origin.z, direction.x, direction.y, direction.z);
    if (result.length === 0){
        return null;
    }
    let Intercepts = [];
    for (const intersect_result of result){
        const intersect_position = origin.clone().add(direction.scale((intersect_result as IntersectResult).distance));
        Intercepts.push(intersect_position);
        (intersect_result as IntersectResult).free();
    }
    return Intercepts;

}

describe('Test ray intersect.', () => {
    it('Test Babylon Headless.', (done) => {
        (global as any).XMLHttpRequest = require('xhr2').XMLHttpRequest;

        const engine = new BABYLON.NullEngine();
        const scene = new BABYLON.Scene(engine);

        const light = new BABYLON.PointLight("Omni", new BABYLON.Vector3(20, 20, 100), scene);

        const camera = new BABYLON.ArcRotateCamera("Camera", 0, 0.8, 100, BABYLON.Vector3.Zero(), scene);


        BABYLON.SceneLoader.ImportMesh("", "https://www.babylonjs.com/Assets/DamagedHelmet/glTF/", "DamagedHelmet.gltf", scene, function (meshes) {
            console.log("Meshes loaded from gltf file: " + meshes.length);
            for (let index = 0; index < meshes.length; index++) {
                const indices = meshes[index].getIndices();
                if (!indices) {
                    continue;
                }
                const position = (meshes[index] as Mesh).getVerticesData(VertexBuffer.PositionKind);
                if (!position) {
                    continue
                }

                const Origin = new BABYLON.Vector3(0, 0, 0);
                const Direction = new BABYLON.Vector3(0, -1, 0);
                const MaxDistance = Infinity;

                //time test rusty
                const meshId = 'test2-mesh';
                set_mesh(meshId, new Uint32Array(indices), position as Float32Array);

                let position_intersects = get_3d_position(Origin, Direction, MaxDistance, meshId);
                const t0 = Date.now()
                for (let i = 0; i < 10000; i++) {
                    position_intersects = get_3d_position(Origin, Direction, MaxDistance, meshId);
                }
                const t1 = Date.now()
                if(position_intersects){
                    console.log(`Rusty      intersect took ${t1 - t0} milliseconds. Intersected at${position_intersects}`)
                }

                //
                expect(remove_mesh(meshId)).eq(true);
                expect(has_mesh(meshId)).eq(false);


                // Time Babylonjs raycast.
                (meshes[index] as Mesh).useOctreeForPicking = true;
                (meshes[index] as Mesh).useOctreeForCollisions = true;
                (meshes[index] as Mesh).useOctreeForRenderingSelection = true;
                const ray = new BABYLON.Ray(Origin,Direction,MaxDistance);
                let hit = (meshes[index] as Mesh).intersects(ray, false);

                const t2 = Date.now();
                for (let i = 0; i < 10000; i++) {
                    hit = (meshes[index] as Mesh).intersects(ray, false);
                }
                const t3 = Date.now()
                console.log(`Babylon intersect took ${t3 - t2} milliseconds. Intersected at ${hit.pickedPoint}`)
            }

            console.log("render started")
            engine.runRenderLoop(function () {
                scene.render();
                engine.dispose();
                done();
            })

        });

    }).timeout(20000);

    // it('Should test ray intersect.', async () => {
    //     init_panic_hook();
    //
    //     const indices = new Uint32Array([
    //         0, 1, 2,
    //         0, 2, 3,
    //         4, 5, 6,
    //         4, 6, 7,
    //         8, 9, 10,
    //         8, 10, 11,
    //         12, 13, 14,
    //         12, 14, 15,
    //         16, 17, 18,
    //         16, 18, 19,
    //         20, 21, 22,
    //         20, 22, 23
    //     ]);
    //
    //     const positions = new Float32Array([
    //         0.5, -0.5, 0.5,
    //         -0.5, -0.5, 0.5,
    //         -0.5, 0.5, 0.5,
    //         0.5, 0.5, 0.5,
    //         0.5, 0.5, -0.5,
    //         -0.5, 0.5, -0.5,
    //         -0.5, -0.5, -0.5,
    //         0.5, -0.5, -0.5,
    //         0.5, 0.5, -0.5,
    //         0.5, -0.5, -0.5,
    //         0.5, -0.5, 0.5,
    //         0.5, 0.5, 0.5,
    //         -0.5, 0.5, 0.5,
    //         -0.5, -0.5, 0.5,
    //         -0.5, -0.5, -0.5,
    //         -0.5, 0.5, -0.5,
    //         -0.5, 0.5, 0.5,
    //         -0.5, 0.5, -0.5,
    //         0.5, 0.5, -0.5,
    //         0.5, 0.5, 0.5,
    //         0.5, -0.5, 0.5,
    //         0.5, -0.5, -0.5,
    //         -0.5, -0.5, -0.5,
    //         -0.5, -0.5, 0.5
    //     ]);
    //
    //     const meshId = 'test-mesh';
    //
    //     expect(has_mesh(meshId)).eq(false);
    //
    //     set_mesh(meshId, indices, positions);
    //
    //     expect(has_mesh(meshId)).eq(true);
    //
    //     const result: IntersectResult = new IntersectResult();
    //     expect(ray_intersect(meshId, 0, 1, 0, 0, -1, 0).length).eq(true);
    //     expect(result.hit).eq(true);
    //     expect(result.distance).eq(0.5);
    //     result.free();
    //
    //     expect(remove_mesh(meshId)).eq(true);
    //     expect(has_mesh(meshId)).eq(false);
    //
    // }).timeout(10000);

});
