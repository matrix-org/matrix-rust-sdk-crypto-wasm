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
 * This is the entrypoint on node-compatible ESM environments.
 * `asyncLoad` will use `fs.readFile` to load the WASM module.
 */

import { fileURLToPath } from "node:url";
import { readFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import * as bindings from "./pkg/matrix_sdk_crypto_wasm_bg.js";

const filename = fileURLToPath(new URL("./pkg/matrix_sdk_crypto_wasm_bg.wasm", import.meta.url));

// In node environments, we want to automatically load the WASM module
// synchronously if the consumer did not call `initAsync`. To do so, we install
// a `Proxy` that will intercept calls to the WASM module.
bindings.__wbg_set_wasm(
    new Proxy(
        {},
        {
            get(_target, prop) {
                const instance = loadModuleSync();
                // @ts-expect-error: This results in an `any` type, which is fine
                return instance[prop];
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
 * Loads and instantiates the WASM module synchronously
 *
 * It will throw if there is an attempt to load the module asynchronously running
 *
 * @returns {typeof import("./pkg/matrix_sdk_crypto_wasm_bg.wasm.d.ts")}
 */
function loadModuleSync() {
    if (modPromise) throw new Error("The WASM module is being loaded asynchronously but hasn't finished");
    const bytes = readFileSync(filename);
    const mod = new WebAssembly.Module(bytes);

    const instance = new WebAssembly.Instance(mod, {
        // @ts-expect-error: The bindings don't exactly match the 'ExportValue' type
        "./matrix_sdk_crypto_wasm_bg.js": bindings,
    });

    initInstance(instance);

    // @ts-expect-error: Typescript doesn't know what the module exports are
    return instance.exports;
}

/**
 * Loads and instantiates the WASM module asynchronously
 *
 * @returns {Promise<typeof import("./pkg/matrix_sdk_crypto_wasm_bg.wasm.d.ts")>}
 */
async function loadModuleAsync() {
    const bytes = await readFile(filename);
    const { instance } = await WebAssembly.instantiate(bytes, {
        // @ts-expect-error: The bindings don't exactly match the 'ExportValue' type
        "./matrix_sdk_crypto_wasm_bg.js": bindings,
    });

    initInstance(instance);

    // @ts-expect-error: Typescript doesn't know what the module exports are
    return instance.exports;
}

/**
 * Initializes the WASM module and returns the exports from the WASM module.
 *
 * @param {WebAssembly.Instance} instance
 */
function initInstance(instance) {
    if (initialised) throw new Error("initInstance called twice");
    bindings.__wbg_set_wasm(instance.exports);
    // @ts-expect-error: Typescript doesn't know what the module exports are
    instance.exports.__wbindgen_start();
    initialised = true;
}

/**
 * Load the WebAssembly module in the background, if it has not already been loaded.
 *
 * Returns a promise which will resolve once the other methods are ready.
 *
 * @returns {Promise<void>}
 */
export async function initAsync() {
    if (initialised) return;
    if (!modPromise) modPromise = loadModuleAsync();
    await modPromise;
}

// Re-export everything from the generated javascript wrappers
export * from "./pkg/matrix_sdk_crypto_wasm_bg.js";
