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

// This is the entrypoint on non-node CommonJS environments.
// `asyncLoad` will load the WASM module using a `fetch` call.
const bindings = require("./pkg/matrix_sdk_crypto_wasm_bg.cjs");
const moduleUrl = require.resolve("./pkg/matrix_sdk_crypto_wasm_bg.wasm");

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

let mod;
async function loadModule() {
    if (mod) return mod;

    if (typeof WebAssembly.compileStreaming === "function") {
        mod = await WebAssembly.compileStreaming(fetch(moduleUrl));
        return mod;
    }

    // Fallback to fetch and compile
    const response = await fetch(moduleUrl);
    if (!response.ok) {
        throw new Error(`Failed to fetch wasm module: ${moduleUrl}`);
    }
    const bytes = await response.arrayBuffer();
    mod = await WebAssembly.compile(response);
    return mod;
}

async function initAsync() {
    const module = await loadModule();
    const instance = new WebAssembly.Instance(module, {
        "./matrix_sdk_crypto_wasm_bg.js": bindings,
    });
    bindings.__wbg_set_wasm(instance.exports);
    instance.exports.__wbindgen_start();
}

module.exports = {
    // Re-export everything from the generated javascript wrappers
    ...bindings,
    initAsync,
};
