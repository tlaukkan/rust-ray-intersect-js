import {expect} from 'chai';
import {
    init_panic_hook,
    IntersectResult,
    init, MeshIntersectorJS, SphereIntersectorJS
} from 'rust-ray-intersect';

describe('Test ray intersect.', () => {

    it('Should test ray intersect.', async () => {
        await init();

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

        const result: IntersectResult[] = intersector.intersect(0, 1, 0, 0, -1, 0, meshId);
        expect(result.length).eq(4);
        expect(result[0].hit).eq(true);
        expect(result[0].distance).eq(0.5);
        result[0].free();
        result[1].free();

        expect(intersector.remove(meshId)).eq(true);
        expect(intersector.has(meshId)).eq(false);
    });

    it('Should test ray sphere intersect.', async () => {
        await init();

        init_panic_hook();

        const id = 'test-mesh';

        const intersector = new SphereIntersectorJS();

        expect(intersector.has(id)).eq(false);

        expect(intersector.set(id, 0, 0, 0, 1));

        expect(intersector.has(id)).eq(true);

        const result: String[] = intersector.intersect( 0, 1, 0, 0, -1, 0, 1);
        expect(result.length).eq(1);
        expect(result[0]).eq(id);

        expect(intersector.remove(id)).eq(true);
        expect(intersector.has(id)).eq(false);

    }).timeout(10000);
});
