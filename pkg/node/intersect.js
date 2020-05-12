let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
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

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}
/**
*/
module.exports.init_panic_hook = function() {
    wasm.init_panic_hook();
};

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

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory0;
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

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}
/**
*/
class IntersectResult {

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
module.exports.IntersectResult = IntersectResult;
/**
*/
class MeshIntersector {

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
module.exports.MeshIntersector = MeshIntersector;
/**
*/
class SphereIntersector {

    static __wrap(ptr) {
        const obj = Object.create(SphereIntersector.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_sphereintersector_free(ptr);
    }
    /**
    */
    constructor() {
        var ret = wasm.sphereintersector_new();
        return SphereIntersector.__wrap(ret);
    }
    /**
    * @param {string} id
    * @returns {boolean}
    */
    has(id) {
        var ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.sphereintersector_has(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
    * @param {string} id
    * @returns {boolean}
    */
    remove(id) {
        var ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.sphereintersector_remove(this.ptr, ptr0, len0);
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
        wasm.sphereintersector_set(this.ptr, ptr0, len0, x, y, z, radius);
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
        var ret = wasm.sphereintersector_intersect(this.ptr, origin_x, origin_y, origin_z, direction_x, direction_y, direction_z, ray_length);
        return takeObject(ret);
    }
}
module.exports.SphereIntersector = SphereIntersector;

module.exports.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

module.exports.__wbg_intersectresult_new = function(arg0) {
    var ret = IntersectResult.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

module.exports.__wbg_new_59cb74e423758ede = function() {
    var ret = new Error();
    return addHeapObject(ret);
};

module.exports.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

module.exports.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

module.exports.__wbg_new_0d50725e1ae68303 = function() {
    var ret = new Array();
    return addHeapObject(ret);
};

module.exports.__wbg_push_46274b393147c746 = function(arg0, arg1) {
    var ret = getObject(arg0).push(getObject(arg1));
    return ret;
};

module.exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

const path = require('path').join(__dirname, 'intersect_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

