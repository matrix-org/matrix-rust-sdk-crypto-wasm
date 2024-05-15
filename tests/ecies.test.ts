const { Ecies } = require("../pkg/matrix_sdk_crypto_wasm");

describe(Ecies.name, () => {
    test("can establish a channel and decrypt the initial message", () => {
        const alice = new Ecies();
        const bob = new Ecies();

        const { initial_message, channel: alice_established } = alice.establish_outbound_channel(
            bob.public_key(),
            "It's a secret to everybody",
        );

        const { message, channel } = bob.establish_inbound_channel(initial_message);
        expect(message).toStrictEqual("It's a secret to everybody");

        const alice_check = alice_established.check_code();
        const bob_check = channel.check_code();

        expect(alice_check.as_bytes()).toStrictEqual(bob_check.as_bytes());
        expect(alice_check.to_digit()).toStrictEqual(bob_check.to_digit());

        const ciphertext = channel.encrypt("Other message");
        const second_plaintext = alice_established.decrypt(ciphertext);

        expect(message).toStrictEqual("It's a secret to everybody");
        expect(second_plaintext).toStrictEqual("Other message");
    });
});
