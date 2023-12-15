const {
    _test_make_keys_claim_request: makeKeysClaimRequest,
    _test_make_keys_query_request: makeKeysQueryRequest,
    _test_make_keys_upload_request: makeKeysUploadRequest,
    RequestType,
    KeysUploadRequest,
    KeysQueryRequest,
    KeysClaimRequest,
    ToDeviceRequest,
    SignatureUploadRequest,
    RoomMessageRequest,
    KeysBackupRequest,
} = require("../pkg/matrix_sdk_crypto_wasm");

describe("RequestType", () => {
    test("has the correct variant values", () => {
        expect(RequestType.KeysUpload).toStrictEqual(0);
        expect(RequestType.KeysQuery).toStrictEqual(1);
        expect(RequestType.KeysClaim).toStrictEqual(2);
        expect(RequestType.ToDevice).toStrictEqual(3);
        expect(RequestType.SignatureUpload).toStrictEqual(4);
        expect(RequestType.RoomMessage).toStrictEqual(5);
        expect(RequestType.KeysBackup).toStrictEqual(6);
    });

    test("Converts request types", () => {
        // test that timeout gets transformed properly into a number
        const keysClaimRequest = makeKeysClaimRequest();
        const keysClaimBody = JSON.parse(keysClaimRequest.body);
        expect(keysClaimBody).toEqual({
            "one_time_keys": {
                "@alice:localhost": {
                    "ABCDEFG": "signed_curve25519",
                },
            },
            "timeout": 10000,
        });

        // test that timeout is omitted when set to None
        const keysQueryRequest = makeKeysQueryRequest();
        const keysQueryBody = JSON.parse(keysQueryRequest.body);
        expect(keysQueryBody).toEqual({
            "device_keys": {},
        })

        // test that device_keys is omitted when set to None
        const keysUploadRequest = makeKeysUploadRequest();
        const keysUploadBody = JSON.parse(keysUploadRequest.body);
        expect(keysUploadBody).toEqual({
            fallback_keys: {},
            one_time_keys: {},
        });
    });
});
