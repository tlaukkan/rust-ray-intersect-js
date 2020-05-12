/* tslint:disable */
/* eslint-disable */
export function init(): Promise<void>;
/**
 */
export function init_panic_hook(): void;
/**
 */
export class IntersectResult {
  free(): void;
  /**
   */
  constructor();
  /**
   * @returns {number}
   */
  distance: number;
  /**
   * @returns {boolean}
   */
  hit: boolean;
  /**
   * @returns {number}
   */
  triangle_index: number;
  /**
   * @returns {number}
   */
  u: number;
  /**
   * @returns {number}
   */
  v: number;
}
/**
 */
export class MeshIntersector {
  free(): void;
  /**
   */
  constructor();
  /**
   * @param {string} mesh_id
   * @returns {boolean}
   */
  has(mesh_id: string): boolean;
  /**
   * @param {string} mesh_id
   * @returns {boolean}
   */
  remove(mesh_id: string): boolean;
  /**
   * @param {string} mesh_id
   * @param {Uint32Array} indices
   * @param {Float32Array} positions
   * @returns {number}
   */
  set(mesh_id: string, indices: Uint32Array, positions: Float32Array): number;
  /**
   * @param {number} origin_x
   * @param {number} origin_y
   * @param {number} origin_z
   * @param {number} direction_x
   * @param {number} direction_y
   * @param {number} direction_z
   * @param {string} mesh_id
   * @returns {Array<IntersectResult>}
   */
  intersect(origin_x: number, origin_y: number, origin_z: number, direction_x: number, direction_y: number, direction_z: number, mesh_id: string): Array<IntersectResult>;
}
/**
 */
export class SphereIntersector {
  free(): void;
  /**
   */
  constructor();
  /**
   * @param {string} id
   * @returns {boolean}
   */
  has(id: string): boolean;
  /**
   * @param {string} id
   * @returns {boolean}
   */
  remove(id: string): boolean;
  /**
   * @param {string} id
   * @param {number} x
   * @param {number} y
   * @param {number} z
   * @param {number} radius
   */
  set(id: string, x: number, y: number, z: number, radius: number): void;
  /**
   * @param {number} origin_x
   * @param {number} origin_y
   * @param {number} origin_z
   * @param {number} direction_x
   * @param {number} direction_y
   * @param {number} direction_z
   * @param {number} ray_length
   * @returns {Array<String>}
   */
  intersect(origin_x: number, origin_y: number, origin_z: number, direction_x: number, direction_y: number, direction_z: number, ray_length: number): Array<String>;
}
