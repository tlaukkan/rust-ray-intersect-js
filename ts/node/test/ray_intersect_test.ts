import {expect} from 'chai';
import {
    init_panic_hook,
    IntersectResult,
    MeshIntersectorJS,
    SphereIntersectorJS
} from 'rust-ray-intersect';

import * as BABYLON from 'babylonjs';
// Force loading loaders.
import * as BABYLON_LOADERS from 'babylonjs-loaders';
import GLTFLoaderCoordinateSystemMode = BABYLON_LOADERS.GLTFLoaderCoordinateSystemMode;
import VertexBuffer = BABYLON.VertexBuffer;
import Mesh = BABYLON.Mesh;
import Vector3 = BABYLON.Vector3;

const gltfLoaderCoordinateSystemMode = GLTFLoaderCoordinateSystemMode;

const get_3d_position = (intersector: MeshIntersectorJS, origin: Vector3, direction: Vector3, distance: number, meshId: string): Vector3[]|null => {
    let result = intersector.intersect(origin.x, origin.y, origin.z, direction.x, direction.y, direction.z, meshId);
    if (result.length === 0){
        return null;
    }
    let Intercepts = [];
    for (const intersect_result of result){
        const intersect_position = origin.clone().add(direction.scale(intersect_result.distance));
        Intercepts.push(intersect_position);
        intersect_result.free();
    }
    return Intercepts;

}

describe('Test ray intersect.', () => {
    it('Test Babylon Headless.', (done) => {
        (global as any).XMLHttpRequest = require('xhr2').XMLHttpRequest;

        const intersector = new MeshIntersectorJS();

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
                intersector.set(meshId, new Uint32Array(indices), position as Float32Array);

                let position_intersects = get_3d_position(intersector, Origin, Direction, MaxDistance, meshId);
                const t0 = Date.now()
                for (let i = 0; i < 10000; i++) {
                    position_intersects = get_3d_position(intersector, Origin, Direction, MaxDistance, meshId);
                }
                const t1 = Date.now()
                if(position_intersects){
                    console.log(`Rusty      intersect took ${t1 - t0} milliseconds. Intersected at${position_intersects}`)
                }

                //
                expect(intersector.remove(meshId)).eq(true);
                expect(intersector.has(meshId)).eq(false);


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

    it('Should test ray mesh intersect.', async () => {
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

        const intersector = new MeshIntersectorJS();


        expect(intersector.has(meshId)).eq(false);

        expect(intersector.set(meshId, indices, positions)).eq(0.8660253882408142);

        expect(intersector.has(meshId)).eq(true);

        const result: IntersectResult[] = intersector.intersect( 0, 1, 0, 0, -1, 0, meshId);
        expect(result.length).eq(4);
        expect(result[0].hit).eq(true);
        expect(result[0].distance).eq(0.5);
        result[0].free();
        result[1].free();

        expect(intersector.remove(meshId)).eq(true);
        expect(intersector.has(meshId)).eq(false);

    }).timeout(10000);

    it('Should test ray sphere intersect.', async () => {
        init_panic_hook();

        const id = 'test-mesh';

        const intersector = new SphereIntersectorJS();

        expect(intersector.has(id)).eq(false);

        expect(intersector.add(id, 0, 0, 0, 1));
        expect(intersector.build());

        expect(intersector.has(id)).eq(true);

        const result: String[] = intersector.intersect( 0, 1, 0, 0, -1, 0, 1);
        expect(result.length).eq(1);
        expect(result[0]).eq(id);

        expect(intersector.remove(id)).eq(true);
        expect(intersector.build());
        expect(intersector.has(id)).eq(false);

    }).timeout(10000);
});
