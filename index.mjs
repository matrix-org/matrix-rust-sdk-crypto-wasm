export * from "./pkg/matrix_sdk_crypto_wasm_bg.js";
import * as bindings from "./pkg/matrix_sdk_crypto_wasm_bg.js";

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

const moduleUrl = new URL("./pkg/matrix_sdk_crypto_wasm_bg.wasm", import.meta.url);

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
    mod = await WebAssembly.compile(bytes);
    return mod;
}

export async function initAsync() {
    const mod = await loadModule();
    const instance = new WebAssembly.Instance(mod, {
        "./matrix_sdk_crypto_wasm_bg.js": bindings,
    });
    bindings.__wbg_set_wasm(instance.exports);
    instance.exports.__wbindgen_start();
}
