import {expect} from 'chai';
import {add} from 'rust-ray-intersect';

describe('Test intersect.', () => {

  it('Should test intersect.', async () => {
    expect(add(1,2)).eq(3);

  }).timeout(10000);

});
