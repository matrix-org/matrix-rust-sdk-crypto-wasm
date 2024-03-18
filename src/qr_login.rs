//! Types for QR code login

use matrix_sdk_crypto::qr_login;
use url::Url;
use wasm_bindgen::prelude::*;

use crate::vodozemac::Curve25519PublicKey;

#[wasm_bindgen]
pub enum QrCodeMode {
    Login,
    Reciprocate,
}

impl From<qr_login::QrCodeMode> for QrCodeMode {
    fn from(value: qr_login::QrCodeMode) -> Self {
        match value {
            qr_login::QrCodeMode::Login => Self::Login,
            qr_login::QrCodeMode::Reciprocate => Self::Reciprocate,
        }
    }
}

#[wasm_bindgen]
pub struct QrCodeData {
    inner: qr_login::QrCodeData,
}

#[wasm_bindgen]
impl QrCodeData {
    /// Create new [`QrCodeData`] from a given public key, a rendezvous URL and,
    /// optionally, a homeserver ULR.
    ///
    /// If a homeserver URL is given, then the [`QrCodeData`] mode will be
    /// [`QrCodeMode::Reciprocate`], i.e. the QR code will contain data for the
    /// existing device to display the QR code. If no homeserver is given,
    /// the [`QrCodeData`] mode will be [`QrCodeMode::Login`], i.e. the QR
    /// code will contain data for the new device to display the QR code.
    #[wasm_bindgen(constructor)]
    pub fn new(
        public_key: Curve25519PublicKey,
        rendezvous_url: &str,
        homeserver_url: Option<String>,
    ) -> Result<QrCodeData, JsError> {
        let public_key = public_key.inner;
        let rendezvous_url = Url::parse(rendezvous_url)?;

        let mode = if let Some(homeserver_url) = homeserver_url {
            qr_login::QrCodeModeData::Reciprocate { homeserver_url: Url::parse(&homeserver_url)? }
        } else {
            qr_login::QrCodeModeData::Login
        };

        let inner = qr_login::QrCodeData { public_key, rendezvous_url, mode };

        Ok(QrCodeData { inner })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<QrCodeData, JsError> {
        Ok(Self { inner: qr_login::QrCodeData::from_bytes(bytes)? })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.to_bytes()
    }

    pub fn from_base64(data: &str) -> Result<QrCodeData, JsError> {
        Ok(Self { inner: qr_login::QrCodeData::from_base64(data)? })
    }

    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }

    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Curve25519PublicKey {
        self.inner.public_key.into()
    }

    #[wasm_bindgen(getter)]
    pub fn rendezvous_url(&self) -> String {
        self.inner.rendezvous_url.as_str().to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn homeserver_url(&self) -> Option<String> {
        if let qr_login::QrCodeModeData::Reciprocate { homeserver_url } = &self.inner.mode {
            Some(homeserver_url.as_str().to_owned())
        } else {
            None
        }
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> QrCodeMode {
        self.inner.mode.mode_identifier().into()
    }
}
