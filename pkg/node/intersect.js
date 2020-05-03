let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
let wasm;
const { TextDecoder } = require(String.raw`util`);

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

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
* Initialized panic hook.
*/
module.exports.init_panic_hook = function() {
    wasm.init_panic_hook();
};

/**
*/
module.exports.greet = function() {
    wasm.greet();
};

/**
* @param {string} mesh_id
* @param {Uint32Array} indices
* @param {Float32Array} positions
*/
module.exports.save_mesh_triangles = function(mesh_id, indices, positions) {
    var ptr0 = passStringToWasm0(mesh_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.save_mesh_triangles(ptr0, len0, addHeapObject(indices), addHeapObject(positions));
};

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
module.exports.intersect_vector_with_mesh_triangles = function(mesh_id, start_x, start_y, start_z, end_x, end_y, end_z) {
    var ptr0 = passStringToWasm0(mesh_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.intersect_vector_with_mesh_triangles(ptr0, len0, start_x, start_y, start_z, end_x, end_y, end_z);
    return ret === 0 ? undefined : Intersection.__wrap(ret);
};

/**
* @param {number} a
* @param {number} b
* @returns {number}
*/
module.exports.add = function(a, b) {
    var ret = wasm.add(a, b);
    return ret;
};

/**
* @param {number} a
* @param {number} b
* @returns {number}
*/
module.exports.bad_add = function(a, b) {
    var ret = wasm.bad_add(a, b);
    return ret;
};

/**
* @param {any} array
* @returns {number}
*/
module.exports.test_number_array = function(array) {
    var ret = wasm.test_number_array(addHeapObject(array));
    return ret >>> 0;
};

/**
* @param {Float32Array} array
* @returns {number}
*/
module.exports.test_float_32_array = function(array) {
    var ret = wasm.test_float_32_array(addHeapObject(array));
    return ret >>> 0;
};

/**
* @param {Float64Array} array
* @returns {number}
*/
module.exports.test_float_64_array = function(array) {
    var ret = wasm.test_float_64_array(addHeapObject(array));
    return ret >>> 0;
};

/**
*/
class Intersection {

    static __wrap(ptr) {
        const obj = Object.create(Intersection.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_intersection_free(ptr);
    }
    /**
    * @returns {number}
    */
    get triangle_index() {
        var ret = wasm.__wbg_get_intersection_triangle_index(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set triangle_index(arg0) {
        wasm.__wbg_set_intersection_triangle_index(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get x() {
        var ret = wasm.__wbg_get_intersection_x(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_intersection_x(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        var ret = wasm.__wbg_get_intersection_y(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_intersection_y(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get z() {
        var ret = wasm.__wbg_get_intersection_z(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set z(arg0) {
        wasm.__wbg_set_intersection_z(this.ptr, arg0);
    }
}
module.exports.Intersection = Intersection;

module.exports.__wbindgen_json_serialize = function(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = JSON.stringify(obj === undefined ? null : obj);
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

module.exports.__wbg_alert_182cc974c2b0c640 = function(arg0, arg1) {
    alert(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
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

module.exports.__wbg_buffer_eb5185aa4a8e9c62 = function(arg0) {
    var ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};

module.exports.__wbg_new_470473004db6a289 = function(arg0) {
    var ret = new Float32Array(getObject(arg0));
    return addHeapObject(ret);
};

module.exports.__wbg_set_47b2beca3d5c9e3f = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};

module.exports.__wbg_length_2f682a6b8ac0fb07 = function(arg0) {
    var ret = getObject(arg0).length;
    return ret;
};

module.exports.__wbg_new_dc67e7a478517b2a = function(arg0) {
    var ret = new Float64Array(getObject(arg0));
    return addHeapObject(ret);
};

module.exports.__wbg_set_140fcae3e39e261c = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};

module.exports.__wbg_length_27825c6c3610b331 = function(arg0) {
    var ret = getObject(arg0).length;
    return ret;
};

module.exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_memory = function() {
    var ret = wasm.memory;
    return addHeapObject(ret);
};

const path = require('path').join(__dirname, 'intersect_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

