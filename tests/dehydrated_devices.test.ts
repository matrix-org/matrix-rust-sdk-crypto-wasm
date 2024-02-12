import { DeviceId, OlmMachine, UserId } from "../pkg/matrix_sdk_crypto_wasm";
import "fake-indexeddb/auto";

afterEach(() => {
    // reset fake-indexeddb after each test, to make sure we don't leak data
    // cf https://github.com/dumbmatter/fakeIndexedDB#wipingresetting-the-indexeddb-for-a-fresh-state
    // eslint-disable-next-line no-global-assign
    indexedDB = new IDBFactory();
});

describe("dehydrated devices", () => {
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
