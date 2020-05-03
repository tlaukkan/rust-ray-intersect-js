/* tslint:disable */
/* eslint-disable */
/**
* Initialized panic hook.
*/
export function init_panic_hook(): void;
/**
*/
export function greet(): void;
/**
* @param {string} mesh_id 
* @param {Uint32Array} indices 
* @param {Float32Array} positions 
*/
export function save_mesh_triangles(mesh_id: string, indices: Uint32Array, positions: Float32Array): void;
/**
* @param {string} mesh_id 
* @param {number} start_x 
* @param {number} start_y 
* @param {number} start_z 
* @param {number} end_x 
* @param {number} end_y 
* @param {number} end_z 
* @returns {Intersection | undefined} 
*/
export function intersect_vector_with_mesh_triangles(mesh_id: string, start_x: number, start_y: number, start_z: number, end_x: number, end_y: number, end_z: number): Intersection | undefined;
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
export class Intersection {
  free(): void;
/**
* @returns {number} 
*/
  triangle_index: number;
/**
* @returns {number} 
*/
  x: number;
/**
* @returns {number} 
*/
  y: number;
/**
* @returns {number} 
*/
  z: number;
}
