import {expect} from 'chai';
import {init_panic_hook, ray_intersect, set_mesh, has_mesh, remove_mesh, IntersectResult} from 'rust-ray-intersect';

describe('Test ray intersect.', () => {

  it('Should test ray intersect.', async () => {
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

    expect(has_mesh(meshId)).eq(false);

    set_mesh(meshId, indices, positions);

    expect(has_mesh(meshId)).eq(true);

    const result: IntersectResult = new IntersectResult();
    expect(ray_intersect(meshId, 0.5, 0.5, 0.5, 0, 0, -1, result)).eq(true);
    expect(result.hit).eq(true);
    expect(result.distance).eq(0.5);
    result.free();

    expect(remove_mesh(meshId)).eq(true);

    expect(has_mesh(meshId)).eq(false);

  }).timeout(10000);

});
