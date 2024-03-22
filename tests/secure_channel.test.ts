const {
    SecureChannel,
    SecretsBundle,
    UserId,
    DeviceId,
    OlmMachine,
    RequestType,
} = require("../pkg/matrix_sdk_crypto_wasm");

describe(SecureChannel.name, () => {
    test("can establish a channel and decrypt the initial message", () => {
        const alice = new SecureChannel();
        const bob = new SecureChannel();

        const alice_established = alice.create_outbound_channel(bob.public_key());
        const initial_message = alice_established.encrypt("It's a secret to everybody");

        const { message, channel } = bob.create_inbound_channel(initial_message);
        expect(message).toStrictEqual("It's a secret to everybody");

        const ciphertext = channel.encrypt("Other message");
        const second_plaintext = alice_established.decrypt(ciphertext);

        expect(message).toStrictEqual("It's a secret to everybody");
    });
});

describe(SecretsBundle.name, () => {
    async function bootstrapMachine(machine: OlmMachine): Promise<Object> {
        let bootstrapRequest = await machine.bootstrapCrossSigning(false);
        const crossSigning = JSON.parse(bootstrapRequest.uploadSigningKeysRequest.body);
        const userIdStr = machine.userId.toString();

        const response = {
            device_keys: {
                [userIdStr]: {},
            },
            master_keys: {
                [userIdStr]: crossSigning.master_key,
            },
            self_signing_keys: {
                [userIdStr]: crossSigning.self_signing_key,
            },

            user_signing_keys: {
                [userIdStr]: crossSigning.user_signing_key,
            }
        };

        return response;
    }

    test("can import a secrets bundle", async () => {
        const userId = new UserId("@alice:example.org");
        const firstDevice = new DeviceId("ABCDEF");
        const secondDevice = new DeviceId("DEVICE2");
        const firstMachine = await OlmMachine.initialize(userId, firstDevice);

        const keys_query_response = await bootstrapMachine(firstMachine);

        const secondMachine = await OlmMachine.initialize(userId, secondDevice);
        secondMachine.mark

        await secondMachine.markRequestAsSent(
            "ID",
            RequestType.KeysQuery,
            JSON.stringify(keys_query_response),
        );

        const bundle = await firstMachine.exportSecretsBundle();

        const alice = new SecureChannel();
        const bob = new SecureChannel();

        const json_bundle = bundle.to_json();

        const alice_established = alice.create_outbound_channel(bob.public_key());
        const initial_message = alice_established.encrypt(JSON.stringify(json_bundle));

        const { message, channel } = bob.create_inbound_channel(initial_message);

        const deserialize_message = JSON.parse(message);
        const received_bundle = SecretsBundle.from_json(deserialize_message);

        await secondMachine.importSecretsBundle(received_bundle);

        const crossSigningStatus = await secondMachine.crossSigningStatus();
        expect(crossSigningStatus.hasMaster).toStrictEqual(true);
        expect(crossSigningStatus.hasSelfSigning).toStrictEqual(true);
        expect(crossSigningStatus.hasUserSigning).toStrictEqual(true);

        const exported_bundle = await secondMachine.exportSecretsBundle();

        expect(exported_bundle.masterKey).toStrictEqual(bundle.masterKey);
        expect(exported_bundle.selfSigningKey).toStrictEqual(bundle.selfSigningKey);
        expect(exported_bundle.userSigningKey).toStrictEqual(bundle.userSigningKey);
    });
});

