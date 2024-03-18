const { BackupDecryptionKey } = require("../pkg");

const aMegolmKey = {
    algorithm: "m.megolm.v1.aes-sha2",
    sender_key: "wREG/hBdSspoqM9xPCEXd/4YwjpBFXlsobRkyDTo/Q8",
    session_key:
        "AQAAAABwCEYsl5BrvPW0N8HTYP11phC7LOzItQLS3Zen6j1j9qMydUHVDeuMLxwo5i3GYfLWGjJEjsCj0Q99TZMABnJBCFg9MheV8cNSBfj7mHSZr6NP8aUAAAOhsY+cJwPDHxcnU181nAEs0fovHnonZGXs6iB/K6sKfuRWUNvX50ORohgDT3TGl0gQFed1FQEtn2Q1qT35iTRfe81SGOnFJrOM",
    sender_claimed_keys: { ed25519: "MnNLGwn4j9ArCvtgU6o1jG8TgJaEXQpDTxz7QU0h7GM" },
    forwarding_curve25519_key_chain: [],
};

const encryptedMegolm = {
    first_message_index: 0,
    forwarded_count: 0,
    is_verified: false,
    session_data: {
        ephemeral: "HlLi76oV6wxHz3PCqE/bxJi6yF1HnYz5Dq3T+d/KpRw",
        ciphertext:
            "MuM8E3Yc6TSAvhVGb77rQ++jE6p9dRepx63/3YPD2wACKAppkZHeFrnTH6wJ/HSyrmzo7HfwqVl6tKNpfooSTHqUf6x1LHz+h4B/Id5ITO1WYt16AaI40LOnZqTkJZCfSPuE2oxalwEHnCS3biWybutcnrBFPR3LMtaeHvvkb+k3ny9l5ZpsU9G7vCm3XoeYkWfLekWXvDhbqWrylXD0+CNUuaQJ/S527TzLd4XKctqVjjO/cCH7q+9utt9WJAfK8LGaWT/mZ3AeWjf5kiqOpKKf5Cn4n5SSil5p/pvGYmjnURvZSEeQIzHgvunIBEPtzK/MYEPOXe/P5achNGlCx+5N19Ftyp9TFaTFlTWCTi0mpD7ePfCNISrwpozAz9HZc0OhA8+1aSc7rhYFIeAYXFU326NuFIFHI5pvpSxjzPQlOA+mavIKmiRAtjlLw11IVKTxgrdT4N8lXeMr4ndCSmvIkAzFMo1uZA4fzjiAdQJE4/2WeXFNNpvdfoYmX8Zl9CAYjpSO5HvpwkAbk4/iLEH3hDfCVUwDfMh05PdGLnxeRpiEFWSMSsJNp+OWAA+5JsF41BoRGrxoXXT+VKqlUDONd+O296Psu8Q+d8/S618",
        mac: "GtMrurhDTwo",
    },
};

describe("BackupDecryptionKey", () => {
    test("create from base64 string", () => {
        const backupkey = BackupDecryptionKey.fromBase64("Ha9cklU/9NqFo9WKdVfGzmqUL/9wlkdxfEitbSIPVXw");

        const decrypted = JSON.parse(
            backupkey.decryptV1(
                encryptedMegolm.session_data.ephemeral,
                encryptedMegolm.session_data.mac,
                encryptedMegolm.session_data.ciphertext,
            ),
        );

        expect(decrypted.algorithm).toStrictEqual(aMegolmKey.algorithm);
        expect(decrypted.sender_key).toStrictEqual(aMegolmKey.sender_key);
        expect(decrypted.session_key).toStrictEqual(aMegolmKey.session_key);
    });

    test("errors", () => {
        expect(() => {
            BackupDecryptionKey.fromBase64("notBase64");
        }).toThrow();
    });
});
