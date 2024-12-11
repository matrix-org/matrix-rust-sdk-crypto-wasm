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
 * This is the entry point for ESM environments which support the ES Module Integration Proposal for WebAssembly [1].
 *
 * [1]: https://github.com/webassembly/esm-integration
 */

import * as bindings from "./pkg/matrix_sdk_crypto_wasm_bg.js";

// Although we could simply instantiate the WASM at import time with a top-level `await`,
// we avoid that, to make it easier for callers to delay loading the WASM (and instead
// wait until `initAsync` is called). (Also, Safari 14 doesn't support top-level `await`.)
//
// However, having done so, there is no way to synchronously load the WASM if the user ends
// up using the bindings before calling `initAsync` (unlike under Node.js), so we just throw
// an error.
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
 * Stores a promise of the `loadModuleAsync` call
 * @type {Promise<void> | null}
 */
let modPromise = null;

/**
 * Loads and instantiates the WASM module asynchronously
 *
 * @returns {Promise<void>}
 */
async function loadModuleAsync() {
    /** @type {typeof import("./pkg/matrix_sdk_crypto_wasm_bg.wasm.d.ts")} */
    // @ts-expect-error TSC can't find the definitions file, for some reason.
    const wasm = await import("./pkg/matrix_sdk_crypto_wasm_bg.wasm");
    bindings.__wbg_set_wasm(wasm);
    wasm.__wbindgen_start();
}

/**
 * Load the WebAssembly module in the background, if it has not already been loaded.
 *
 * Returns a promise which will resolve once the other methods are ready.
 *
 * @returns {Promise<void>}
 */
export async function initAsync() {
    if (!modPromise) modPromise = loadModuleAsync();
    await modPromise;
}

// Re-export everything from the generated javascript wrappers
export * from "./pkg/matrix_sdk_crypto_wasm_bg.js";
