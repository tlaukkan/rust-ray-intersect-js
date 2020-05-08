import {expect} from 'chai';
import {init_panic_hook, ray_intersect, set_mesh, has_mesh, remove_mesh, IntersectResult} from 'rust-ray-intersect';

import * as BABYLON from 'babylonjs';
// Force loading loaders.
import * as BABYLON_LOADERS from 'babylonjs-loaders';
import GLTFLoaderCoordinateSystemMode = BABYLON_LOADERS.GLTFLoaderCoordinateSystemMode;
import VertexBuffer = BABYLON.VertexBuffer;
import Mesh = BABYLON.Mesh;

const gltfLoaderCoordinateSystemMode = GLTFLoaderCoordinateSystemMode;

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
                  throw new Error("No indices in mesh.");
                }
                const position = (meshes[index] as Mesh).getVerticesData(VertexBuffer.PositionKind);
                if (!position) {
                    continue
                  //throw new Error("No positions in mesh.");
                }

                const meshId = 'test2-mesh';
                set_mesh(meshId, new Uint32Array(indices), position as Float32Array);
                const result: IntersectResult = new IntersectResult();
                //
                const t0 = Date.now()
                for (let i = 0; i<1000;i++){
                    ray_intersect(meshId, 0, 1, 0, 0, -1, 0, result);
                }
                const t1 = Date.now()

                console.log("Rusty intersect took " + (t1 - t0) + " milliseconds.")
                result.free();
                expect(remove_mesh(meshId)).eq(true);
                expect(has_mesh(meshId)).eq(false);
                //
                const t2 = Date.now()
                for (let i = 0; i<1000;i++){
                    const ray = new BABYLON.Ray(new BABYLON.Vector3(0,0,0), new BABYLON.Vector3(0,-1,0), Infinity);
                    const hit = (meshes[index] as Mesh).intersects(ray, false);
                }
                const t3 = Date.now()
                console.log("Babylon intersect took " + (t3 - t2) + " milliseconds.")


            }

            console.log("render started")
            engine.runRenderLoop(function () {
                scene.render();
                engine.dispose();
                done();
            })

        });

    }).timeout(20000);

    it('Should test ray intersect.', async () => {
        init_panic_hook();

        const indices = new Uint32Array([
            0, 1, 2,
            0, 2, 3,
            4, 5, 6,
            4, 6, 7,
            8, 9, 10,
            8, 10, 11,
            12, 13, 14,
            12, 14, 15,
            16, 17, 18,
            16, 18, 19,
            20, 21, 22,
            20, 22, 23
        ]);

        const positions = new Float32Array([
            0.5, -0.5, 0.5,
            -0.5, -0.5, 0.5,
            -0.5, 0.5, 0.5,
            0.5, 0.5, 0.5,
            0.5, 0.5, -0.5,
            -0.5, 0.5, -0.5,
            -0.5, -0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, 0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, -0.5, 0.5,
            0.5, 0.5, 0.5,
            -0.5, 0.5, 0.5,
            -0.5, -0.5, 0.5,
            -0.5, -0.5, -0.5,
            -0.5, 0.5, -0.5,
            -0.5, 0.5, 0.5,
            -0.5, 0.5, -0.5,
            0.5, 0.5, -0.5,
            0.5, 0.5, 0.5,
            0.5, -0.5, 0.5,
            0.5, -0.5, -0.5,
            -0.5, -0.5, -0.5,
            -0.5, -0.5, 0.5
        ]);

        const meshId = 'test-mesh';

        expect(has_mesh(meshId)).eq(false);

        set_mesh(meshId, indices, positions);

        expect(has_mesh(meshId)).eq(true);

        const result: IntersectResult = new IntersectResult();
        expect(ray_intersect(meshId, 0, 1, 0, 0, -1, 0, result)).eq(true);
        expect(result.hit).eq(true);
        expect(result.distance).eq(0.5);
        result.free();

        expect(remove_mesh(meshId)).eq(true);
        expect(has_mesh(meshId)).eq(false);

    }).timeout(10000);

});
