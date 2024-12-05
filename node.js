// Copyright 2024 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// @ts-check

/**
 * This is the entrypoint on node-compatible CommonJS environments.
 * `asyncLoad` will use `fs.readFile` to load the WASM module.
 */

const { readFileSync } = require("node:fs");
const { readFile } = require("node:fs/promises");
const path = require("node:path");
const bindings = require("./pkg/matrix_sdk_crypto_wasm_bg.cjs");

const filename = path.join(__dirname, "pkg/matrix_sdk_crypto_wasm_bg.wasm");

// In node environments, we want to automatically load the WASM module
// synchronously if the consumer did not call `initAsync`. To do so, we install
// a `Proxy` that will intercept calls to the WASM module.
bindings.__wbg_set_wasm(
    new Proxy(
        {},
        {
            get(_target, prop) {
                const mod = loadModuleSync();
                // @ts-expect-error: This results to an `any` type, which is fine
                return initInstance(mod)[prop];
            },
        },
    ),
);

/**
 * Stores a promise which resolves to the WebAssembly module
 * @type {Promise<WebAssembly.Module> | null}
 */
let modPromise = null;

/**
 * Tracks whether the module has been instantiated or not
 * @type {boolean}
 */
let initialised = false;

/**
 * Loads the WASM module synchronously
 *
 * It will throw if there is an attempt to laod the module asynchronously running
 *
 * @returns {WebAssembly.Module}
 */
function loadModuleSync() {
    if (modPromise) throw new Error("The WASM module is being loadded asynchronously but hasn't finished");
    const bytes = readFileSync(filename);
    return new WebAssembly.Module(bytes);
}

/**
 * Loads the WASM module asynchronously
 *
 * @returns {Promise<WebAssembly.Module>}
 */
async function loadModule() {
    const bytes = await readFile(filename);
    return await WebAssembly.compile(bytes);
}

/**
 * Initializes the WASM module and returns the exports from the WASM module.
 *
 * @param {WebAssembly.Module} mod
 * @returns {typeof import("./pkg/matrix_sdk_crypto_wasm_bg.wasm.d")}
 */
function initInstance(mod) {
    if (initialised) throw new Error("initInstance called twice");

    /** @type {{exports: typeof import("./pkg/matrix_sdk_crypto_wasm_bg.wasm.d.ts")}} */
    // @ts-expect-error: Typescript doesn't know what the instance exports exactly
    const instance = new WebAssembly.Instance(mod, {
        // @ts-expect-error: The bindings don't exactly match the 'ExportValue' type
        "./matrix_sdk_crypto_wasm_bg.js": bindings,
    });

    bindings.__wbg_set_wasm(instance.exports);
    instance.exports.__wbindgen_start();
    initialised = true;
    return instance.exports;
}

/**
 * Load the WebAssembly module in the background, if it has not already been loaded.
 *
 * Returns a promise which will resolve once the other methods are ready.
 *
 * @returns {Promise<void>}
 */
async function initAsync() {
    if (initialised) return;
    if (!modPromise) modPromise = loadModule().then(initInstance);
    await modPromise;
}

module.exports = {
    // Re-export everything from the generated javascript wrappers
    ...bindings,
    initAsync,
};
