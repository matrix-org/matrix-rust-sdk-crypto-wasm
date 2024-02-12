import {
    BackupDecryptionKey,
    CrossSigningStatus,
    DecryptedRoomEvent,
    DecryptionErrorCode,
    DeviceId,
    DeviceKeyId,
    DeviceLists,
    EncryptionSettings,
    EventId,
    getVersions,
    InboundGroupSession,
    KeysBackupRequest,
    KeysClaimRequest,
    KeysQueryRequest,
    KeysUploadRequest,
    MaybeSignature,
    MegolmDecryptionError,
    OlmMachine,
    OwnUserIdentity,
    RequestType,
    RoomId,
    RoomMessageRequest,
    ShieldColor,
    SignatureState,
    SignatureUploadRequest,
    StoreHandle,
    ToDeviceRequest,
    UserId,
    UserIdentity,
    VerificationRequest,
    Versions,
} from "../pkg/matrix_sdk_crypto_wasm";
import "fake-indexeddb/auto";

describe("dehydrate devices", () => {
    test("can dehydrate and rehydrate a device", async () => {
        const machine = await OlmMachine.initialize(new UserId("@alice:example.org"), new DeviceId("ABCDEFG"));
        await machine.bootstrapCrossSigning(true);

        const dehydratedDevices = machine.dehydratedDevices();
        const device = await dehydratedDevices.create();
        const key = new Uint8Array(32);
        const req = await device.keysForUpload("Dehydrated device", key);
        const body = JSON.parse(req.body);

        const machine2 = await OlmMachine.initialize(new UserId("@alice:example.org"), new DeviceId("HIJKLMN"));
        const dehydratedDevices2 = machine2.dehydratedDevices();
        const rehydratedDevice = await dehydratedDevices2.rehydrate(
            key,
            new DeviceId(body.device_id),
            JSON.stringify(body.device_data),
        );
    });
});
