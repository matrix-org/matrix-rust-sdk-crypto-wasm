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

// This is the entrypoint on node-compatible ESM environments.
// `asyncLoad` will use `fs.readFile` to load the WASM module.
import { fileURLToPath } from "node:url";
import { readFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import * as bindings from "./pkg/matrix_sdk_crypto_wasm_bg.js";
export * from "./pkg/matrix_sdk_crypto_wasm_bg.js";

const filename = fileURLToPath(new URL("./pkg/matrix_sdk_crypto_wasm_bg.wasm", import.meta.url));

// We want to automatically load the WASM module in Node environments
// synchronously if the consumer did not call `initAsync`. To do so, we install
// a `Proxy` that will intercept calls to the WASM module
bindings.__wbg_set_wasm(
    new Proxy(
        {},
        {
            get(_target, prop) {
                loadModuleSync();
                return initInstance(mod)[prop];
            },
        },
    ),
);

let mod;
function loadModuleSync() {
    if (mod) return mod;
    const bytes = readFileSync(filename);
    mod = new WebAssembly.Module(bytes);
}

async function loadModule() {
    if (mod) return mod;
    const bytes = await readFile(filename);
    mod = await WebAssembly.compile(bytes);
}

function initInstance() {
    const instance = new WebAssembly.Instance(mod, {
        "./matrix_sdk_crypto_wasm_bg.js": bindings,
    });
    bindings.__wbg_set_wasm(instance.exports);
    instance.exports.__wbindgen_start();
    return instance.exports;
}

export async function initAsync() {
    await loadModule();
    initInstance();
}
