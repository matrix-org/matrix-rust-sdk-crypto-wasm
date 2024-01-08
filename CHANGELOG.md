# UNRELEASED

-   Add a `Migration` class, supporting importing account and session data from
    libolm.
    ([#77](https://github.com/matrix-org/matrix-rust-sdk-crypto-wasm/pull/77))

-   Add a `StoreHandle` class which can be used to hold a connection to a
    crypto store, and thus improve performance when doing multiple operations
    on the store.
    ([#76](https://github.com/matrix-org/matrix-rust-sdk-crypto-wasm/pull/76))

-   Update `matrix-rust-sdk` version, with changes including:
    -   Fix for an issue which caused the same keys to be repeatedly backed up.
        ([matrix-rust-sdk#2937](https://github.com/matrix-org/matrix-rust-sdk/pull/2957)
    -   Performance improvement in `markRequestAsSent`.
        ([matrix-rust-sdk#2977](https://github.com/matrix-org/matrix-rust-sdk/pull/2977))
    -   Logging for the open sequence for indexeddb store.
        ([matrix-rust-sdk#2983](https://github.com/matrix-org/matrix-rust-sdk/pull/2983))

# matrix-sdk-crypto-wasm v3.5.0

-   Update matrix-rust-sdk version, providing several changes including a fix
    for occasional freezes (https://github.com/element-hq/element-web/issues/26488).

-   New API `OlmMachine.requestMissingSecretsIfNeeded` that creates an
    outgoing secret request to other sessions.

-   Verification cancel codes for `cancelWithCode` and `cancelInfo.cancelCode`
    are now passed as strings rather than an enum.

# matrix-sdk-crypto-wasm v3.4.0

-   Include Rust symbol names in the generated wasm output.
    ([#65](https://github.com/matrix-org/matrix-rust-sdk-crypto-wasm/pull/65))

# matrix-sdk-crypto-wasm v3.3.0

-   Add new properties `roomKeyRequestsEnabled` and `roomKeyForwardingEnabled`
    to OlmMachine.
    ([#60](https://github.com/matrix-org/matrix-rust-sdk-crypto-wasm/pull/60),
    ([matrix-rust-sdk#2902](https://github.com/matrix-org/matrix-rust-sdk/pull/2902))

# matrix-sdk-crypto-wasm v3.2.0

-   Add `timeout_secs` parameters to `OlmMachine.get_user_devices` and
    `OlmMachine.get_device`. ([#60](https://github.com/matrix-org/matrix-rust-sdk-crypto-wasm/pull/60))

-   Improve efficiency of IndexedDB storage of `inbound_group_sessions`.
    ([matrix-rust-sdk#2885](https://github.com/matrix-org/matrix-rust-sdk/pull/2885))

# matrix-sdk-crypto-wasm v3.1.0

-   Improve performance of `OlmMachine.shareRoomKey`.
    ([matrix-rust-sdk#2862](https://github.com/matrix-org/matrix-rust-sdk/pull/2862))

-   `OlmMachine.getMissingSessions`: Don't block waiting for `/keys/query`
    requests on blacklisted servers, and improve performance.
    ([matrix-rust-sdk#2845](https://github.com/matrix-org/matrix-rust-sdk/pull/2845))

-   Various clarifications to the log messages written during encryption operations.
    ([matrix-rust-sdk#2859](https://github.com/matrix-org/matrix-rust-sdk/pull/2859))

-   `OlmMachine.importRoomKeys` is now deprecated in favour of separate
    methods for importing room keys from backup and export,
    `OlmMachine.importBackedUpRoomKeys` and
    `OlmMachine.importExportedRoomKeys`.

-   Minor improvements to the formatting of messages logged to the console.

# matrix-sdk-crypto-wasm v3.0.1

**BREAKING CHANGES**

-   `OlmMachine.bootstrapCrossSigning` no longer returns an array of request
    objects. Rather, it returns a new class (`CrossSigningBootstrapRequests`)
    which contains the request objects within it.

    As part of this work, `SigningKeysUploadRequest` (which was one of the
    types formerly returned by `bootstrapCrossSigning`) has been renamed to
    `UploadSigningKeysRequest` for consistency with the underlying SDK.

## Other changes

-   Devices which have exhausted their one-time-keys will now be correctly
    handled in `/keys/claim` responses (we will register them as "failed" and
    stop attempting to send to them for a while.)

-   Olm decryption operations will no longer log large quantities of data about
    the data `Store`.

# matrix-sdk-crypto-wasm v3.0.0

Do not use this release. It has a [critical bug](https://github.com/matrix-org/matrix-rust-sdk/issues/2802).

# matrix-sdk-crypto-wasm v2.2.0

-   Added bindings versions details to `getVersions()`. Two new fields `git_sha` and
    `git_description` have been included in the returned `Versions` struct.

# matrix-sdk-crypto-wasm v2.1.1

## Changes in the underlying Rust crate

-   Clean up the logging of to-device messages in `share_room_key`. Also fixes
    some `panic` errors which were introduced in v2.1.0.

-   Remove spurious "Unknown outgoing secret request" warning which was logged
    for every outgoing secret request.

-   Various other changes.

# matrix-sdk-crypto-wasm v2.1.0

-   Attach message IDs to outgoing to-device messages, and log the IDs on
    incoming messages.

-   Improve logging output to include more information, including data that is
    attached to tracing spans. Remove the `tracing` feature: tracing support is
    now always included.

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
