# `matrix-sdk-crypto-wasm`

Welcome to the [WebAssembly] + JavaScript binding for the Rust
[`matrix-sdk-crypto`] library! WebAssembly can run anywhere, but these
bindings are designed to run on a JavaScript host. These bindings are
part of the [`matrix-rust-sdk`] project, which is a library
implementation of a [Matrix] client-server.

`matrix-sdk-crypto` is a no-network-IO implementation of a state
machine, named `OlmMachine`, that handles E2EE ([End-to-End
Encryption](https://en.wikipedia.org/wiki/End-to-end_encryption)) for
[Matrix] clients.

## Usage

1. Install in your project:

    ```
    npm install --save @matrix-org/matrix-sdk-crypto-wasm
    ```

    or:

    ```
    yarn add @matrix-org/matrix-sdk-crypto-wasm
    ```

2. Import the library into your project and initialise it.

    It is recommended that you use a dynamic import, particularly in a Web
    environment, because the WASM artifiact is large:

    ```javascript
    async function loadCrypto(userId, deviceId) {
        const matrixSdkCrypto = await import("@matrix-org/matrix-sdk-crypto-wasm");
        await matrixSdkCrypto.initAsync();

        // Optional: enable tracing in the rust-sdk
        new matrixSdkCrypto.Tracing(matrixSdkCrypto.LoggerLevel.Trace).turnOn();

        // Create a new OlmMachine
        //
        // The following will use an in-memory store. It is recommended to use
        // indexedDB where that is available.
        // See https://matrix-org.github.io/matrix-rust-sdk-crypto-wasm/classes/OlmMachine.html#initialize
        const olmMachine = await matrixSdkCrypto.OlmMachine.initialize(
            new matrixSdkCrypto.UserId(userId),
            new matrixSdkCrypto.DeviceId(deviceId),
        );

        return olmMachine;
    }
    ```

## Building

These WebAssembly bindings are written in [Rust]. To build them, you
need to install the Rust compiler, see [the Install Rust
Page](https://www.rust-lang.org/tools/install). Then, the workflow is
pretty classical by using [yarn](https://yarnpkg.com/), see [the Downloading and installing
Node.js and npm
Page](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) and [installing yarn](https://classic.yarnpkg.com/lang/en/docs/install).

Once the Rust compiler, Node.js and yarn are installed, you can run the
following commands:

```sh
$ yarn install
$ yarn build  # or 'yarn build:dev' to make an unoptimised build
$ yarn test
```

The compiled output should be generated in the `pkg/` directory.

## Local development with matrix-rust-sdk

To build based on a local `matrix-rust-sdk`, add something like this to your
`.cargo/config` file:

```
[patch.'https://github.com/matrix-org/matrix-rust-sdk']
matrix-sdk-base = { path = "../matrix-rust-sdk/crates/matrix-sdk-base" }
matrix-sdk-common = { path = "../matrix-rust-sdk/crates/matrix-sdk-common" }
matrix-sdk-crypto = { path = "../matrix-rust-sdk/crates/matrix-sdk-crypto" }
matrix-sdk-indexeddb = { path = "../matrix-rust-sdk/crates/matrix-sdk-indexeddb" }
matrix-sdk-qrcode = { path = "../matrix-rust-sdk/crates/matrix-sdk-qrcode" }
```

## Documentation

[The documentation can be found
online](https://matrix-org.github.io/matrix-rust-sdk-crypto-wasm/).

To generate the documentation locally, please run the following
command:

```sh
$ yarn doc
```

The documentation is generated in the `./docs` directory.

[WebAssembly]: https://webassembly.org/
[`matrix-sdk-crypto`]: https://github.com/matrix-org/matrix-rust-sdk/tree/main/crates/matrix-sdk-crypto
[`matrix-rust-sdk`]: https://github.com/matrix-org/matrix-rust-sdk
[Matrix]: https://matrix.org/
[Rust]: https://www.rust-lang.org/
[npm]: https://www.npmjs.com/
