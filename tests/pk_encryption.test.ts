const {
    PkEncryption,
    PkDecryption,
    PkMessage,
    Base64EncodedPkMessage,
    Curve25519SecretKey,
} = require("@matrix-org/matrix-sdk-crypto-wasm");

describe(PkDecryption.name, () => {
    test("can create PkDecryption and encrypt/decrypt a message", () => {
        const alice = new PkDecryption();

        const publicKey = alice.publicKey();
        const bob = PkEncryption.fromKey(publicKey);

        const message = bob.encryptString("It's a secret to everybody");
        const decrypted = alice.decryptString(message);

        expect(decrypted).toStrictEqual("It's a secret to everybody");
    });

    test("can Base64-encode the encrypted message", () => {
        const alice = new PkDecryption();

        const publicKey = alice.publicKey();
        const bob = PkEncryption.fromKey(publicKey);

        const message = bob.encryptString("It's a secret to everybody");
        const { ciphertext, mac, ephemeralKey } = message.toBase64();

        const base64Message = new Base64EncodedPkMessage(ciphertext, mac, ephemeralKey);
        const reconstructedMessage = PkMessage.fromBase64(base64Message);

        const decrypted = alice.decryptString(message);

        expect(decrypted).toStrictEqual("It's a secret to everybody");
    });

    test("can restore a PkDecryption object with a secret key", () => {
        const alice = new PkDecryption();

        const secretKey = alice.secretKey();
        const publicKey = alice.publicKey();
        const encodedSecretKey = secretKey.toBase64();

        const bob = PkEncryption.fromKey(publicKey);

        const decodedSecretKey = Curve25519SecretKey.fromBase64(encodedSecretKey);
        const restoredAlice = PkDecryption.fromKey(decodedSecretKey);

        const message = bob.encryptString("It's a secret to everybody");
        const { ciphertext, mac, ephemeralKey } = message.toBase64();

        const decrypted = restoredAlice.decryptString(message);
        expect(decrypted).toStrictEqual("It's a secret to everybody");
    });
});
