/* tslint:disable */
/* eslint-disable */
/**
* @param {string} mesh_id 
* @returns {boolean} 
*/
export function has_mesh(mesh_id: string): boolean;
/**
* @param {string} mesh_id 
* @param {Uint32Array} indices 
* @param {Float32Array} positions 
*/
export function set_mesh(mesh_id: string, indices: Uint32Array, positions: Float32Array): void;
/**
* @param {string} mesh_id 
* @returns {boolean} 
*/
export function remove_mesh(mesh_id: string): boolean;
/**
* @param {string} mesh_id 
* @param {number} origin_x 
* @param {number} origin_y 
* @param {number} origin_z 
* @param {number} direction_x 
* @param {number} direction_y 
* @param {number} direction_z 
* @returns {Array<IntersectResult>} 
*/
export function ray_intersect(mesh_id: string, origin_x: number, origin_y: number, origin_z: number, direction_x: number, direction_y: number, direction_z: number): Array<IntersectResult>;
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
