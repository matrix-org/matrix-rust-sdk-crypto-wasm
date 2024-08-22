const { Curve25519PublicKey } = require("../pkg");

describe(Curve25519PublicKey.name, () => {
    test("Can create a Curve25519PublicKey from a base64 string", async () => {
        const key = "itM094954M6rRzAFTwK3OM6xJ2nGtC3YrWZs8wcXg0o";

        const parsedKey = new Curve25519PublicKey(key);

        const serialized = parsedKey.toBase64();
        expect(serialized).toStrictEqual(key);
    });
});
