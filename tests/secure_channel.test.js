const {
    SecureChannel,
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

