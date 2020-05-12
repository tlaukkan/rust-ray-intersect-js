import {expect} from 'chai';
import {
  init_panic_hook,
  ray_intersect,
  set_mesh,
  has_mesh,
  remove_mesh,
  IntersectResult,
  init
} from 'rust-ray-intersect';

describe('Test ray intersect.', () => {

  it('Should test ray intersect.', async () => {
    await init();

      init_panic_hook();

      const indices = new Uint32Array([
          0,  1,  2,
          0,  2,  3,
          4,  5,  6,
          4,  6,  7,
          8,  9, 10,
          8, 10, 11,
          12, 13, 14,
          12, 14, 15,
          16, 17, 18,
          16, 18, 19,
          20, 21, 22,
          20, 22, 23
      ]);

      const positions = new Float32Array([
          0.5, -0.5,  0.5,
          -0.5, -0.5,  0.5,
          -0.5,  0.5,  0.5,
          0.5,  0.5,  0.5,
          0.5,  0.5, -0.5,
          -0.5,  0.5, -0.5,
          -0.5, -0.5, -0.5,
          0.5, -0.5, -0.5,
          0.5,  0.5, -0.5,
          0.5, -0.5, -0.5,
          0.5, -0.5,  0.5,
          0.5,  0.5,  0.5,
          -0.5,  0.5,  0.5,
          -0.5, -0.5,  0.5,
          -0.5, -0.5, -0.5,
          -0.5,  0.5, -0.5,
          -0.5,  0.5,  0.5,
          -0.5,  0.5, -0.5,
          0.5,  0.5, -0.5,
          0.5,  0.5,  0.5,
          0.5, -0.5,  0.5,
          0.5, -0.5, -0.5,
          -0.5, -0.5, -0.5,
          -0.5, -0.5,  0.5
      ]);

      const meshId = 'test-mesh';

      expect(has_mesh(meshId)).eq(false);

      set_mesh(meshId, indices, positions);

      expect(has_mesh(meshId)).eq(true);

      const result: IntersectResult[] = ray_intersect(meshId, 0, 1, 0, 0, -1, 0);
      expect(result.length).eq(4);
      expect(result[0].hit).eq(true);
      expect(result[0].distance).eq(0.5);
      result[0].free();
      result[1].free();

      expect(remove_mesh(meshId)).eq(true);
      expect(has_mesh(meshId)).eq(false);
  });

});
