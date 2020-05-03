import {expect} from 'chai';
import {add, test_number_array, test_float_32_array, test_float_64_array, init_panic_hook} from 'rust-ray-intersect';

describe('Test rust binding.', () => {

  it('Should test add.', async () => {
    expect(add(1,2)).eq(3);

  }).timeout(10000);

  it('Should test number array.', async () => {
    expect(test_number_array([1,2,3,4])).eq(4);
  }).timeout(10000);

  it('Should test Float32Array.', async () => {
    init_panic_hook();
    const float32Array = new Float32Array(5);
    float32Array[0] = 42;
    expect(test_float_32_array(float32Array)).eq(5);
  }).timeout(10000);

  it('Should test Float64Array.', async () => {
    init_panic_hook();
    const float64Array = new Float64Array(5);
    float64Array[0] = 42;
    expect(test_float_64_array(float64Array)).eq(5);
  }).timeout(10000);

  it('Should test number array with test_float_64_array.', async () => {
    expect(test_float_64_array([1,2,3,4, 5] as any)).eq(5);
  }).timeout(10000);

});
