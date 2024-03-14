const {
    SecureChannel,
} = require("../pkg/matrix_sdk_crypto_wasm");

describe(SecureChannel.name, () => {
    const alice = new SecureChannel();
    const bob = new SecureChannel();

    const alice_established = alice.create_outbound_channel(bob.public_key());
    const initial_message = alice_established.encrypt("It's a secret to everybody");
    const { message, channel } = bob.create_inbound_channel(initial_message);

    test("can establish a channel and decrypt the initial message", () => {
        expect(message).toStrictEqual("It's a secret to everybody");
    });
});

