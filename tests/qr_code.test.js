const { QrCodeData, QrCodeMode, Curve25519PublicKey } = require("../pkg/matrix_sdk_crypto_wasm");

describe(QrCodeData.name, () => {
    test("can parse the QR code bytes from the MSC", () => {
        const base64Data =
            "TUFUUklYAgPYhmhqshl7eA4wCp1KIUdIBwDXkp85qzG55RQ3AkjtawBHaHR0cHM6Ly9yZW5kZXp2b3VzLmxhYi5lbGVtZW50LmRldi9lOGRhNjM1NS01NTBiLTRhMzItYTE5My0xNjE5ZDk4MzA2Njg";

        const data = QrCodeData.from_base64(base64Data);

        expect(data.public_key.toBase64()).toStrictEqual("2IZoarIZe3gOMAqdSiFHSAcA15KfOasxueUUNwJI7Ws");
        expect(data.rendezvous_url).toStrictEqual(
            "https://rendezvous.lab.element.dev/e8da6355-550b-4a32-a193-1619d9830668",
        );
        expect(data.mode).toStrictEqual(QrCodeMode.Login);

        const encoded = data.to_base64();

        expect(base64Data).toStrictEqual(encoded);
    });

    test("can construct a new QrCodeData class", () => {
        const base64Data =
            "TUFUUklYAgPYhmhqshl7eA4wCp1KIUdIBwDXkp85qzG55RQ3AkjtawBHaHR0cHM6Ly9yZW5kZXp2b3VzLmxhYi5lbGVtZW50LmRldi9lOGRhNjM1NS01NTBiLTRhMzItYTE5My0xNjE5ZDk4MzA2Njg";
        const publicKey = new Curve25519PublicKey("2IZoarIZe3gOMAqdSiFHSAcA15KfOasxueUUNwJI7Ws");
        const rendezvousUrl = "https://rendezvous.lab.element.dev/e8da6355-550b-4a32-a193-1619d9830668";

        const data = new QrCodeData(publicKey, rendezvousUrl);

        expect(data.public_key.toBase64()).toStrictEqual("2IZoarIZe3gOMAqdSiFHSAcA15KfOasxueUUNwJI7Ws");
        expect(data.rendezvous_url).toStrictEqual(rendezvousUrl);
        expect(data.mode).toStrictEqual(QrCodeMode.Login);

        const encoded = data.to_base64();
        expect(base64Data).toStrictEqual(encoded);
    });
});
