//! Dehydrated devices
//!
//! WASM wrapper for `matrix_sdk_crypto::dehydrated_devices`.

use js_sys::{Array, JsString, Uint8Array};
use matrix_sdk_crypto::dehydrated_devices;
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
        pickle_key: &Uint8Array,
        device_id: &DeviceId,
        device_data: &str,
    ) -> Result<RehydratedDevice, JsError> {
        let pickle_key: [u8; 32] =
            pickle_key.to_vec().try_into().map_err(|_| JsError::new("Wrong key length"))?;
        Ok(self
            .inner
            .rehydrate(&pickle_key, &device_id.inner, serde_json::from_str(device_data)?)
            .await?
            .into())
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
        pickle_key: Uint8Array,
    ) -> Result<PutDehydratedDeviceRequest, JsError> {
        let pickle_key: [u8; 32] =
            pickle_key.to_vec().try_into().map_err(|_| JsError::new("Wrong key length"))?;
        Ok(self
            .inner
            .keys_for_upload(initial_device_display_name.into(), &pickle_key)
            .await?
            .try_into()?)
    }
}
