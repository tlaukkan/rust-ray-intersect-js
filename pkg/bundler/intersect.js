import * as wasmBase64Bytes from "./intersect_bg.wasm";

// CUSTOM INITIALIZATION START
function _base64ToArrayBuffer(base64) {
    var binary_string = window.atob(base64);
    var len = binary_string.length;
    var bytes = new Uint8Array(len);
    for (var i = 0; i < len; i++) {
        bytes[i] = binary_string.charCodeAt(i);
    }
    return bytes.buffer;
}
let imports = {};
const bytes = _base64ToArrayBuffer(wasmBase64Bytes);
export async function init() {
    const wasmInstanceSource = await WebAssembly.instantiate(bytes , imports);
    const wasmInstance = wasmInstanceSource.instance;
    wasm = wasmInstance.exports;
}
// CUSTOM INITIALIZATION END

// UTILITY METHODS FROM BUNDLER AND NODE START
let wasm;
const { TextDecoder } = require(String.raw`util`);

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let cachedTextDecoder = new window.TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}


let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory0;
}


function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

let cachegetNodeBufferMemory0 = null;
function getNodeBufferMemory0() {
    if (cachegetNodeBufferMemory0 === null || cachegetNodeBufferMemory0.buffer !== wasm.memory.buffer) {
        cachegetNodeBufferMemory0 = Buffer.from(wasm.memory.buffer);
    }
    return cachegetNodeBufferMemory0;
}

function passStringToWasm0(arg, malloc) {

    const len = Buffer.byteLength(arg);
    const ptr = malloc(len);
    getNodeBufferMemory0().write(arg, ptr, len);
    WASM_VECTOR_LEN = len;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}


function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4);
    getUint32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachegetFloat32Memory0 = null;
function getFloat32Memory0() {
    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory0;
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4);
    getFloat32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

/*export const __wbg_alert_f58994d3249a7d5b = function(arg0, arg1) {
    alert(getStringFromWasm0(arg0, arg1));
};*/

/*export const __wbg_new_59cb74e423758ede = function() {
    var ret = new Error();
    return addHeapObject(ret);
};*/

/*export const __wbg_stack_558ba5917b466edd = function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};*/

/*export const __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};*/

/*export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};*/
// UTILITY METHODS FROM BUNDLER AND NODE START

// IMPORTS FROM WEB BEGIN
imports.wbg = {};
imports.wbg.__wbg_intersectresult_new = function(arg0) {
    var ret = IntersectResult.__wrap(arg0);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};
imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};
imports.wbg.__wbg_new_59cb74e423758ede = function() {
    var ret = new Error();
    return addHeapObject(ret);
};
imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};
imports.wbg.__wbg_new_0d50725e1ae68303 = function() {
    var ret = new Array();
    return addHeapObject(ret);
};
imports.wbg.__wbg_push_46274b393147c746 = function(arg0, arg1) {
    var ret = getObject(arg0).push(getObject(arg1));
    return ret;
};
imports.wbg.__wbg_buffer_eb5185aa4a8e9c62 = function(arg0) {
    var ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};
imports.wbg.__wbg_new_425c4bc0e35ec22f = function(arg0) {
    var ret = new Uint32Array(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_set_c6918ca5977a5b66 = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};
imports.wbg.__wbg_length_81dcf4356abac381 = function(arg0) {
    var ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_new_470473004db6a289 = function(arg0) {
    var ret = new Float32Array(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_set_47b2beca3d5c9e3f = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};
imports.wbg.__wbg_length_2f682a6b8ac0fb07 = function(arg0) {
    var ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_memory = function() {
    var ret = wasm.memory;
    return addHeapObject(ret);
};
// IMPORTS FROM WEB END

// INTERFACE FROM WEB
/**
 * @param {string} mesh_id
 * @returns {boolean}
 */
export function has_mesh(mesh_id) {
    var ptr0 = passStringToWasm0(mesh_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.has_mesh(ptr0, len0);
    return ret !== 0;
}

/**
 * @param {string} mesh_id
 * @param {Uint32Array} indices
 * @param {Float32Array} positions
 */
export function set_mesh(mesh_id, indices, positions) {
    var ptr0 = passStringToWasm0(mesh_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.set_mesh(ptr0, len0, addHeapObject(indices), addHeapObject(positions));
}

/**
 * @param {string} mesh_id
 * @returns {boolean}
 */
export function remove_mesh(mesh_id) {
    var ptr0 = passStringToWasm0(mesh_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.remove_mesh(ptr0, len0);
    return ret !== 0;
}

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
export function ray_intersect(mesh_id, origin_x, origin_y, origin_z, direction_x, direction_y, direction_z) {
    var ptr0 = passStringToWasm0(mesh_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.ray_intersect(ptr0, len0, origin_x, origin_y, origin_z, direction_x, direction_y, direction_z);
    return takeObject(ret);
}

/**
 */
export function init_panic_hook() {
    wasm.init_panic_hook();
}

/**
 */
export class IntersectResult {

    static __wrap(ptr) {
        const obj = Object.create(IntersectResult.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_intersectresult_free(ptr);
    }
    /**
     * @returns {boolean}
     */
    get hit() {
        var ret = wasm.__wbg_get_intersectresult_hit(this.ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set hit(arg0) {
        wasm.__wbg_set_intersectresult_hit(this.ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get triangle_index() {
        var ret = wasm.__wbg_get_intersectresult_triangle_index(this.ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set triangle_index(arg0) {
        wasm.__wbg_set_intersectresult_triangle_index(this.ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get u() {
        var ret = wasm.__wbg_get_intersectresult_u(this.ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set u(arg0) {
        wasm.__wbg_set_intersectresult_u(this.ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get v() {
        var ret = wasm.__wbg_get_intersectresult_v(this.ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set v(arg0) {
        wasm.__wbg_set_intersectresult_v(this.ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get distance() {
        var ret = wasm.__wbg_get_intersectresult_distance(this.ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set distance(arg0) {
        wasm.__wbg_set_intersectresult_distance(this.ptr, arg0);
    }
    /**
     */
    constructor() {
        var ret = wasm.intersectresult_new();
        return IntersectResult.__wrap(ret);
    }
}
/**
 */
export class MeshIntersector {

    static __wrap(ptr) {
        const obj = Object.create(MeshIntersector.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_meshintersector_free(ptr);
    }
    /**
     */
    constructor() {
        var ret = wasm.meshintersector_new();
        return MeshIntersector.__wrap(ret);
    }
    /**
     * @param {string} mesh_id
     * @returns {boolean}
     */
    has(mesh_id) {
        var ptr0 = passStringToWasm0(mesh_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.meshintersector_has(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * @param {string} mesh_id
     * @returns {boolean}
     */
    remove(mesh_id) {
        var ptr0 = passStringToWasm0(mesh_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.meshintersector_remove(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * @param {string} mesh_id
     * @param {Uint32Array} indices
     * @param {Float32Array} positions
     * @returns {number}
     */
    set(mesh_id, indices, positions) {
        var ptr0 = passStringToWasm0(mesh_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray32ToWasm0(indices, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = passArrayF32ToWasm0(positions, wasm.__wbindgen_malloc);
        var len2 = WASM_VECTOR_LEN;
        var ret = wasm.meshintersector_set(this.ptr, ptr0, len0, ptr1, len1, ptr2, len2);
        return ret;
    }
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
    intersect(origin_x, origin_y, origin_z, direction_x, direction_y, direction_z, mesh_id) {
        var ptr0 = passStringToWasm0(mesh_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.meshintersector_intersect(this.ptr, origin_x, origin_y, origin_z, direction_x, direction_y, direction_z, ptr0, len0);
        return takeObject(ret);
    }
}
/**
 */
export class SphereInterceptor {

    static __wrap(ptr) {
        const obj = Object.create(SphereInterceptor.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_sphereinterceptor_free(ptr);
    }
    /**
     */
    constructor() {
        var ret = wasm.sphereinterceptor_new();
        return SphereInterceptor.__wrap(ret);
    }
    /**
     * @param {string} id
     * @returns {boolean}
     */
    contains(id) {
        var ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.sphereinterceptor_contains(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * @param {string} id
     * @returns {boolean}
     */
    remove(id) {
        var ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.sphereinterceptor_remove(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * @param {string} id
     * @param {number} x
     * @param {number} y
     * @param {number} z
     * @param {number} radius
     */
    set(id, x, y, z, radius) {
        var ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.sphereinterceptor_set(this.ptr, ptr0, len0, x, y, z, radius);
    }
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
    intersect(origin_x, origin_y, origin_z, direction_x, direction_y, direction_z, ray_length) {
        var ret = wasm.sphereinterceptor_intersect(this.ptr, origin_x, origin_y, origin_z, direction_x, direction_y, direction_z, ray_length);
        return takeObject(ret);
    }
}
// INTERFACE FROM WEB