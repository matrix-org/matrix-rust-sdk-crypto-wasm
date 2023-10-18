# matrix-sdk-crypto-wasm vx.x.x

-   BugFix: `ToDeviceRequest` returned by `shareRoomKey(..)` always had an `undefined` `id` field.

# matrix-sdk-crypto-wasm v2.0.0

-   Updated rust sdk version to revision [c2bb76029ae6d99c741727e0f87abcd734377016](https://github.com/matrix-org/matrix-rust-sdk/commit/c2bb76029ae6d99c741727e0f87abcd734377016), including:
    -   [Remove dashmap crate usage from matrix-sdk-crypto](https://github.com/matrix-org/matrix-rust-sdk/pull/2669)
    -   [Bugfix for invalidated private cross signing keys not being persisted](https://github.com/matrix-org/matrix-rust-sdk/pull/2676)
-   API Break: `RoomId.localpart` and `RoomId.serverName` have been removed.
-   Add new secrets API `OlmMachine.registerReceiveSecretCallback`,
    `OlmMachine.getSecretsFromInbox`, `OlmMachine.deleteSecretsFromInbox`.

# matrix-sdk-crypto-wasm v1.3.0

-   Add `OlmMachine.registerUserIdentityUpdatedCallback`.
-   Expose new method `OlmMachine.getRoomEventEncryptionInfo`.
-   Update `IndexeddbCryptoStore` to use a single store for outgoing secret
    requests.

# matrix-sdk-crypto-wasm v1.2.2

## Changes in the WASM bindings

-   `OlmMachine.decrypt_room_event()` now throws a
    typed `MegolmDecryptionError` instead of generic `Error`.

# matrix-sdk-crypto-wasm v1.2.1

## Changes in the WASM bindings

-   Expose `version` field of `KeysBackupRequest`.

# matrix-sdk-crypto-wasm v1.2.0

## Changes in the WASM bindings

-   The `BackupKeys` structure returned by `OlmMachine.getBackupKeys` now
    contains a `decryptionKey` property which is is a `BackupDecryptionKey`
    instance.

-   Expose `SignatureVerification.trusted` method.

# matrix-sdk-crypto-wasm v1.1.0

## Changes in the WASM bindings

-   Expose bindings for secure key backup.
-   Expose `OwnUserIdentity.isVerified`.

## Changes in the underlying Rust crate

-   Mark our `OwnUserIdentity` as verified if we successfully import the matching private keys.

# matrix-sdk-crypto-wasm v1.0.1

No functional changes. Fixes for the release process which prevented v1.0.0
being released.

# matrix-sdk-crypto-wasm v1.0.0

Project renamed to `matrix-sdk-crypto-wasm`. No functional changes.

# matrix-sdk-crypto-js v0.1.4

-   Add method `OlmMachine.queryKeysForUsers` to build an out-of-band key
    request.

# matrix-sdk-crypto-js v0.1.3

## Changes in the Javascript bindings

-   Fix bug introduced in v0.1.2 which caused an undocumented change to the results of `OlmMachine.receiveSyncChanges`.

## Changes in the underlying Rust crate

-   Fix a bug which could cause generated one-time-keys not to be persisted.

# matrix-sdk-crypto-js v0.1.2

**WARNING**: this version had a breaking change in the result type of `OlmMachine.receiveSyncChanges`.
This is corrected in v0.1.3.

## Changes in the Javascript bindings

-   Add `Qr.state()` method to inspect the current state of QR code
    verifications.

## Changes in the underlying Rust crate

-   Fix handling of SAS verification start events once we have shown a QR code.

# matrix-sdk-crypto-js v0.1.1

-   Add `verify` method to `Device`.

# matrix-sdk-crypto-js v0.1.0

## Changes in the Javascript bindings

-   In `OlmMachine.getIdentity`, wait a limited time for any in-flight
    device-list updates to complete.

-   Add `VerificationRequest.timeRemainingMillis()`.

## Changes in the underlying Rust crate

-   When rejecting a key-verification request over to-device messages, send the
    `m.key.verification.cancel` to the device that made the request, rather
    than broadcasting to all devices.

# matrix-sdk-crypto-js v0.1.0-alpha.11

## Changes in the Javascript bindings

-   Simplify the response type of `Sas.confirm()`.
-   Add `VerificationRequest.registerChangesCallback()`,
    `Sas.registerChangesCallback()`, and `Qr.registerChangesCallback()`.
-   Add `VerificationRequest.phase()` and `VerificationRequest.getVerification()`.

## Changes in the underlying Rust crate

-   Add support for the `hkdf-hmac-sha256.v2` SAS message authentication code.

-   Ensure that the correct short authentication strings are used when accepting a
    SAS verification with the `Sas::accept()` method.

# matrix-sdk-crypto-js v0.1.0-alpha.10

-   Add `masterKey`, `userSigningKey`, `selfSigningKey` to `UserIdentity` and `OwnUserIdentity`

# matrix-sdk-crypto-js v0.1.0-alpha.9

-   Extend `OlmDevice.markRequestAsSent` to accept responses to
    `SigningKeysUploadRequest`s.
-   Add a missing `const` for compatibility with ECMAScript Module compatibility
    mode.
-   Fix the body of `SignatureUploadRequest`s to match the spec.
-   Add a constructor for `SigningKeysUploadRequest`.

# matrix-sdk-crypto-js v0.1.0-alpha.8

-   `importCrossSigningKeys`: change the parameters to be individual keys
    rather than a `CrossSigningKeyExport` object.
-   Make `unused_fallback_keys` optional in `Machine.receive_sync_changes`

# matrix-sdk-crypto-js v0.1.0-alpha.7

-   Add new accessors `Device.algorithms` and `Device.isSignedByOwner`
-   In `OlmMachine.getUserDevices`, wait a limited time for any in-flight
    device-list updates to complete.

# matrix-sdk-crypto-js v0.1.0-alpha.6

-   Add new accessor `InboundGroupSession.senderKey`.
-   Add a new API, `OlmMachine.registerRoomKeyUpdatedCallback`, which
    applications can use to listen for received room keys.
