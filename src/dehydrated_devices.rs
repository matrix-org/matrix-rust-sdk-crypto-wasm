//! Dehydrated devices
//!
//! WASM wrapper for `matrix_sdk_crypto::dehydrated_devices`.

use js_sys::{Array, JsString, Uint8Array};
use matrix_sdk_crypto::{dehydrated_devices, store::DehydratedDeviceKey as SdkDehydratedDeviceKey};
use wasm_bindgen::prelude::*;

use crate::{identifiers::DeviceId, requests::PutDehydratedDeviceRequest, store::RoomKeyInfo};

#[wasm_bindgen]
#[derive(Debug)]
/// Struct collecting methods to create and rehydrate dehydrated devices.
pub struct DehydratedDevices {
    inner: dehydrated_devices::DehydratedDevices,
}

impl From<dehydrated_devices::DehydratedDevices> for DehydratedDevices {
    fn from(value: dehydrated_devices::DehydratedDevices) -> Self {
        Self { inner: value }
    }
}

/// Dehydrated device key
#[wasm_bindgen]
#[derive(Debug)]
pub struct DehydratedDeviceKey {
    pub(crate) inner: Uint8Array,
}

#[wasm_bindgen]
impl DehydratedDeviceKey {
    /// Generates a new random pickle key.
    #[wasm_bindgen(js_name = "createRandomKey")]
    pub fn create_random_key() -> Result<DehydratedDeviceKey, JsError> {
        Ok(SdkDehydratedDeviceKey::new()?.into())
    }

    /// Generates a new random pickle key.
    #[wasm_bindgen(js_name = "createKeyFromArray")]
    pub fn create_key_from_array(array: Uint8Array) -> Result<DehydratedDeviceKey, JsError> {
        Ok(SdkDehydratedDeviceKey::from_slice(array.to_vec().as_slice())?.into())
    }

    /// Convert the pickle key to a base 64 encoded string.
    #[wasm_bindgen(js_name = "toBase64")]
    pub fn to_base64(&self) -> JsString {
        let binding = self.inner.to_vec();
        let inner: &[u8; 32] = binding.as_slice().try_into().expect("Expected 32 byte array");

        SdkDehydratedDeviceKey::from(inner).to_base64().into()
    }
}

// Zero out on drop
impl Drop for DehydratedDeviceKey {
    fn drop(&mut self) {
        self.inner.fill(0, 0, 32);
    }
}

impl From<matrix_sdk_crypto::store::DehydratedDeviceKey> for DehydratedDeviceKey {
    fn from(pickle_key: matrix_sdk_crypto::store::DehydratedDeviceKey) -> Self {
        let vec: Vec<u8> = pickle_key.into();
        DehydratedDeviceKey { inner: vec.as_slice().into() }
    }
}

#[wasm_bindgen]
impl DehydratedDevices {
    /// Create a new [`DehydratedDevice`] which can be uploaded to the server.
    #[wasm_bindgen]
    pub async fn create(&self) -> Result<DehydratedDevice, JsError> {
        Ok(self.inner.create().await?.into())
    }

    /// Rehydrate a dehydrated device.
    #[wasm_bindgen]
    pub async fn rehydrate(
        &self,
        pickle_key: &DehydratedDeviceKey,
        device_id: &DeviceId,
        device_data: &str,
    ) -> Result<RehydratedDevice, JsError> {
        let sdk_pickle_key =
            SdkDehydratedDeviceKey::from_slice(pickle_key.inner.to_vec().as_slice())?;

        Ok(self
            .inner
            .rehydrate(&sdk_pickle_key, &device_id.inner, serde_json::from_str(device_data)?)
            .await?
            .into())
    }

    /// Get the cached dehydrated device pickle key if any.
    ///
    /// None if the key was not previously cached (via
    /// [`Self::save_dehydrated_device_pickle_key`]).
    ///
    /// Should be used to periodically rotate the dehydrated device to avoid
    /// OTK exhaustion and accumulation of to_device messages.
    #[wasm_bindgen(js_name = "getDehydratedDeviceKey")]
    pub async fn get_dehydrated_device_key(&self) -> Result<Option<DehydratedDeviceKey>, JsError> {
        let key = self.inner.get_dehydrated_device_pickle_key().await?;
        Ok(key.map(DehydratedDeviceKey::from))
    }

    /// Store the dehydrated device pickle key in the crypto store.
    ///
    /// This is useful if the client wants to periodically rotate dehydrated
    /// devices to avoid OTK exhaustion and accumulated to_device problems.
    #[wasm_bindgen(js_name = "saveDehydratedDeviceKey")]
    pub async fn save_dehydrated_device_key(
        &self,
        pickle_key: &DehydratedDeviceKey,
    ) -> Result<(), JsError> {
        let sdk_pickle_key =
            SdkDehydratedDeviceKey::from_slice(pickle_key.inner.to_vec().as_slice())?;
        self.inner.save_dehydrated_device_pickle_key(&sdk_pickle_key).await?;
        Ok(())
    }

    /// Clear the dehydrated device pickle key saved in the crypto store.
    #[wasm_bindgen(js_name = "deleteDehydratedDeviceKey")]
    pub async fn delete_dehydrated_device_key(&self) -> Result<(), JsError> {
        self.inner.delete_dehydrated_device_pickle_key().await?;
        Ok(())
    }
}

#[wasm_bindgen]
#[derive(Debug)]
/// A rehydrated device
///
/// This device can receive to-device events to get room keys that were send to
/// it.
pub struct RehydratedDevice {
    inner: dehydrated_devices::RehydratedDevice,
}

impl From<dehydrated_devices::RehydratedDevice> for RehydratedDevice {
    fn from(value: dehydrated_devices::RehydratedDevice) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl RehydratedDevice {
    #[wasm_bindgen(js_name = "receiveEvents")]
    /// Receive the to-device events that sent to the dehydrated device
    ///
    /// The rehydrated device will decrypt the events and pass the room keys
    /// into the `OlmMachine`.
    ///
    /// `to_device_events` is a JSON-encoded result of the `events` array from
    /// `/dehydrated_device/{device_id}/events`.
    ///
    /// Returns an array of `RoomKeyInfo`, indicating the room keys that were
    /// received.
    pub async fn receive_events(&self, to_device_events: &str) -> Result<Array, JsError> {
        let to_device_events = serde_json::from_str(to_device_events)?;

        let room_key_info = self.inner.receive_events(to_device_events).await?;
        Ok(room_key_info.into_iter().map(RoomKeyInfo::from).map(JsValue::from).collect())
    }
}

#[wasm_bindgen]
#[derive(Debug)]
/// A dehydrated device that can be uploaded to the server
pub struct DehydratedDevice {
    inner: dehydrated_devices::DehydratedDevice,
}

impl From<dehydrated_devices::DehydratedDevice> for DehydratedDevice {
    fn from(value: dehydrated_devices::DehydratedDevice) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl DehydratedDevice {
    #[wasm_bindgen(js_name = "keysForUpload")]
    /// Create the request to upload the dehydrated device
    pub async fn keys_for_upload(
        &self,
        initial_device_display_name: JsString,
        pickle_key: &DehydratedDeviceKey,
    ) -> Result<PutDehydratedDeviceRequest, JsError> {
        let pickle_key = SdkDehydratedDeviceKey::from_slice(pickle_key.inner.to_vec().as_slice())?;

        Ok(self
            .inner
            .keys_for_upload(initial_device_display_name.into(), &pickle_key)
            .await?
            .try_into()?)
    }
}
