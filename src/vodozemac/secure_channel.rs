use std::sync::{Arc, Mutex};

use matrix_sdk_crypto::vodozemac::secure_channel;
use wasm_bindgen::prelude::*;

use super::Curve25519PublicKey;

#[wasm_bindgen(getter_with_clone)]
pub struct ChannelCreationResult {
    pub channel: EstablishedSecureChannel,
    pub message: String,
}

#[wasm_bindgen]
pub struct SecureChannel {
    inner: Option<secure_channel::SecureChannel>,
    public_key: Curve25519PublicKey,
}

#[wasm_bindgen]
impl SecureChannel {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let inner = secure_channel::SecureChannel::new();
        let public_key = inner.public_key().into();

        Self { inner: Some(inner), public_key }
    }

    pub fn public_key(&self) -> Curve25519PublicKey {
        self.public_key.clone()
    }

    fn used_up_error() -> JsError {
        JsError::new("The channel was already established and used up.")
    }

    pub fn create_inbound_channel(
        &mut self,
        initial_message: &str,
    ) -> Result<ChannelCreationResult, JsError> {
        let message = secure_channel::InitialMessage::decode(initial_message)?;
        let result =
            self.inner.take().ok_or_else(Self::used_up_error)?.create_inbound_channel(&message)?;

        let message = String::from_utf8_lossy(&result.message).to_string();

        Ok(ChannelCreationResult { message, channel: result.secure_channel.into() })
    }

    pub fn create_outbound_channel(
        &mut self,
        public_key: &Curve25519PublicKey,
    ) -> Result<EstablishedSecureChannel, JsError> {
        let channel = self
            .inner
            .take()
            .ok_or_else(Self::used_up_error)?
            .create_outbound_channel(public_key.inner)?;

        Ok(channel.into())
    }
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct EstablishedSecureChannel {
    inner: Arc<Mutex<secure_channel::EstablishedSecureChannel>>,
}

#[wasm_bindgen]
impl EstablishedSecureChannel {
    pub fn public_key(&self) -> Curve25519PublicKey {
        self.inner.lock().unwrap().public_key().into()
    }

    pub fn encrypt(&mut self, message: &str) -> String {
        self.inner.lock().unwrap().encrypt(message.as_bytes()).encode()
    }

    pub fn decrypt(&mut self, message: &str) -> Result<String, JsError> {
        let message = secure_channel::Message::decode(message)?;
        let result = self.inner.lock().unwrap().decrypt(&message)?;

        Ok(String::from_utf8_lossy(&result).to_string())
    }
}

impl From<secure_channel::EstablishedSecureChannel> for EstablishedSecureChannel {
    fn from(value: secure_channel::EstablishedSecureChannel) -> Self {
        Self { inner: Mutex::new(value).into() }
    }
}
