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
* @param {IntersectResult} result 
* @returns {boolean} 
*/
export function ray_intersect(mesh_id: string, origin_x: number, origin_y: number, origin_z: number, direction_x: number, direction_y: number, direction_z: number, result: IntersectResult): boolean;
/**
*/
export function init_panic_hook(): void;
/**
*/
export function greet(): void;
/**
* @param {number} a 
* @param {number} b 
* @returns {number} 
*/
export function add(a: number, b: number): number;
/**
* @param {number} a 
* @param {number} b 
* @returns {number} 
*/
export function bad_add(a: number, b: number): number;
/**
* @param {any} array 
* @returns {number} 
*/
export function test_number_array(array: any): number;
/**
* @param {Float32Array} array 
* @returns {number} 
*/
export function test_float_32_array(array: Float32Array): number;
/**
* @param {Float64Array} array 
* @returns {number} 
*/
export function test_float_64_array(array: Float64Array): number;
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
