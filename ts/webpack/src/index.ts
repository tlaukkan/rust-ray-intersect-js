import {init, has_mesh, init_panic_hook, IntersectResult, ray_intersect, set_mesh} from "../../../pkg";

init().then(
    () => {
        console.log('hello world.');

        init_panic_hook();

        const indices = new Uint32Array(3);
        indices[0] = 0;
        indices[1] = 1;
        indices[2] = 2;

        const positions = new Float32Array(9);
        positions[0] = 0;
        positions[1] = 0;
        positions[2] = 0;

        positions[3] = 2;
        positions[4] = 0;
        positions[5] = 0;

        positions[6] = 0;
        positions[7] = 2;
        positions[8] = 0;

        const meshId = 'test-mesh';

        set_mesh(meshId, indices, positions);
        const result: IntersectResult = new IntersectResult();
        ray_intersect(meshId, 0.5, 0.5, 0.5, 0, 0, -1, result);
        console.log("hit: " + result.hit);
        result.free();
    }
)


