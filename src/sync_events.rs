//! `GET /_matrix/client/*/sync`

use matrix_sdk_common::ruma;
use wasm_bindgen::prelude::*;

use crate::identifiers;

/// Information on E2E device updates.
#[wasm_bindgen]
#[derive(Debug)]
pub struct DeviceLists {
    pub(crate) inner: ruma::api::client::sync::sync_events::DeviceLists,
}

#[wasm_bindgen]
impl DeviceLists {
    /// Create an empty `DeviceLists`.
    ///
    /// `changed` and `left` must be an array of `UserId`.
    ///
    /// Items inside `changed` and `left` will be invalidated by this method. Be
    /// careful not to use the `UserId`s after this method has been called.
    #[wasm_bindgen(constructor)]
    pub fn new(
        changed: Option<Vec<identifiers::UserId>>,
        left: Option<Vec<identifiers::UserId>>,
    ) -> Result<DeviceLists, JsError> {
        let mut inner = ruma::api::client::sync::sync_events::DeviceLists::default();

        inner.changed = changed.unwrap_or_default().iter().map(|user| user.inner.clone()).collect();
        inner.left = left.unwrap_or_default().iter().map(|user| user.inner.clone()).collect();

        Ok(Self { inner })
    }

    /// Returns true if there are no device list updates.
    #[wasm_bindgen(js_name = "isEmpty")]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// List of users who have updated their device identity keys or
    /// who now share an encrypted room with the client since the
    /// previous sync
    #[wasm_bindgen(getter)]
    pub fn changed(&self) -> Vec<identifiers::UserId> {
        self.inner.changed.iter().map(|user| identifiers::UserId::from(user.clone())).collect()
    }

    /// List of users who no longer share encrypted rooms since the
    /// previous sync response.
    #[wasm_bindgen(getter)]
    pub fn left(&self) -> Vec<identifiers::UserId> {
        self.inner.left.iter().map(|user| identifiers::UserId::from(user.clone())).collect()
    }
}
