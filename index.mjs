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
 * This is the entrypoint on non-node ESM environments (such as Element Web).
 * `asyncLoad` will load the WASM module using a `fetch` call.
 */

import * as bindings from "./pkg/matrix_sdk_crypto_wasm_bg.js";

const moduleUrl = new URL("./pkg/matrix_sdk_crypto_wasm_bg.wasm", import.meta.url);

// We want to throw an error if the user tries to use the bindings before
// calling `initAsync`.
bindings.__wbg_set_wasm(
    new Proxy(
        {},
        {
            get() {
                throw new Error(
                    "@matrix-org/matrix-sdk-crypto-wasm was used before it was initialized. Call `initAsync` first.",
                );
            },
        },
    ),
);

/**
 * Stores a promise of the `loadModule` call
 * @type {Promise<void> | null}
 */
let modPromise = null;

/**
 * Loads the WASM module asynchronously
 *
 * @returns {Promise<void>}
 */
async function loadModule() {
    let mod;
    if (typeof WebAssembly.compileStreaming === "function") {
        mod = await WebAssembly.compileStreaming(fetch(moduleUrl));
    } else {
        // Fallback to fetch and compile
        const response = await fetch(moduleUrl);
        if (!response.ok) {
            throw new Error(`Failed to fetch wasm module: ${moduleUrl}`);
        }
        const bytes = await response.arrayBuffer();
        mod = await WebAssembly.compile(bytes);
    }

    /** @type {{exports: typeof import("./pkg/matrix_sdk_crypto_wasm_bg.wasm.d.ts")}} */
    // @ts-expect-error: Typescript doesn't know what the instance exports exactly
    const instance = new WebAssembly.Instance(mod, {
        // @ts-expect-error: The bindings don't exactly match the 'ExportValue' type
        "./matrix_sdk_crypto_wasm_bg.js": bindings,
    });

    bindings.__wbg_set_wasm(instance.exports);
    instance.exports.__wbindgen_start();
}

/**
 * Load the WebAssembly module in the background, if it has not already been loaded.
 *
 * Returns a promise which will resolve once the other methods are ready.
 *
 * @returns {Promise<void>}
 */
export async function initAsync() {
    if (!modPromise) modPromise = loadModule();
    await modPromise;
}

// Re-export everything from the generated javascript wrappers
export * from "./pkg/matrix_sdk_crypto_wasm_bg.js";
