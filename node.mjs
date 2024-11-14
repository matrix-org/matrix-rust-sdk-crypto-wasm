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
