//! Store types.

use std::sync::Arc;

use matrix_sdk_crypto::store::{DynCryptoStore, IntoCryptoStore, MemoryStore};
use wasm_bindgen::prelude::*;

use crate::{
    encryption::EncryptionAlgorithm, identifiers::RoomId, impl_from_to_inner,
    vodozemac::Curve25519PublicKey,
};

/// A struct containing an open connection to a CryptoStore.
///
/// Opening the CryptoStore can take some time, due to the PBKDF calculation
/// involved, so if multiple operations are being done on the same store, it is
/// more efficient to open it once.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct StoreHandle {
    pub(crate) store: Arc<DynCryptoStore>,
}

#[wasm_bindgen]
impl StoreHandle {
    /// Open a crypto store.
    ///
    /// The created store will be based on IndexedDB if a `store_name` is
    /// provided; otherwise it will be based on a memory store and once the
    /// objects is dropped, the keys will be lost.
    ///
    /// # Arguments
    ///
    ///
    /// * `store_name` - The name that should be used to open the IndexedDB
    ///   based database. If this isn't provided, a memory-only store will be
    ///   used. *Note* the memory-only store will lose your E2EE keys when the
    ///   `StoreHandle` gets dropped.
    ///
    /// * `store_passphrase` - The passphrase that should be used to encrypt the
    ///   store, for IndexedDB-based stores
    #[wasm_bindgen(js_name = "open")]
    pub async fn open_for_js(
        store_name: Option<String>,
        store_passphrase: Option<String>,
    ) -> Result<JsValue, JsValue> {
        Ok(StoreHandle::open(store_name, store_passphrase)
            .await
            .map_err(|e| JsError::from(&*e))?
            .into())
    }

    pub(crate) async fn open(
        store_name: Option<String>,
        store_passphrase: Option<String>,
    ) -> Result<StoreHandle, anyhow::Error> {
        let store = match store_name {
            Some(store_name) => Self::open_indexeddb(&store_name, store_passphrase).await?,

            None => {
                if store_passphrase.is_some() {
                    return Err(anyhow::Error::msg(
                        "The `store_passphrase` has been set, but it has an effect only if \
                        `store_name` is set, which is not; please provide one",
                    ));
                }

                MemoryStore::new().into_crypto_store()
            }
        };

        Ok(Self { store })
    }

    async fn open_indexeddb(
        store_name: &String,
        store_passphrase: Option<String>,
    ) -> Result<Arc<DynCryptoStore>, matrix_sdk_indexeddb::IndexeddbCryptoStoreError> {
        let store = match store_passphrase {
            Some(mut store_passphrase) => {
                use zeroize::Zeroize;

                let store = matrix_sdk_indexeddb::IndexeddbCryptoStore::open_with_passphrase(
                    &store_name,
                    &store_passphrase,
                )
                .await?;

                store_passphrase.zeroize();
                store
            }

            None => matrix_sdk_indexeddb::IndexeddbCryptoStore::open_with_name(&store_name).await?,
        };

        Ok(store.into_crypto_store())
    }
}

impl IntoCryptoStore for StoreHandle {
    fn into_crypto_store(self) -> Arc<DynCryptoStore> {
        self.store.clone()
    }
}

/// A struct containing private cross signing keys that can be backed
/// up or uploaded to the secret store.
#[wasm_bindgen]
#[derive(Debug)]
pub struct CrossSigningKeyExport {
    pub(crate) inner: matrix_sdk_crypto::store::CrossSigningKeyExport,
}

impl_from_to_inner!(matrix_sdk_crypto::store::CrossSigningKeyExport => CrossSigningKeyExport);

#[wasm_bindgen]
impl CrossSigningKeyExport {
    /// The seed of the master key encoded as unpadded base64.
    #[wasm_bindgen(getter, js_name = "masterKey")]
    pub fn master_key(&self) -> Option<String> {
        self.inner.master_key.clone()
    }

    /// The seed of the self signing key encoded as unpadded base64.
    #[wasm_bindgen(getter, js_name = "self_signing_key")]
    pub fn self_signing_key(&self) -> Option<String> {
        self.inner.self_signing_key.clone()
    }

    /// The seed of the user signing key encoded as unpadded base64.
    #[wasm_bindgen(getter, js_name = "userSigningKey")]
    pub fn user_signing_key(&self) -> Option<String> {
        self.inner.user_signing_key.clone()
    }
}

/// Information on a room key that has been received or imported.
#[wasm_bindgen]
#[derive(Debug)]
pub struct RoomKeyInfo {
    pub(crate) inner: matrix_sdk_crypto::store::RoomKeyInfo,
}

impl_from_to_inner!(matrix_sdk_crypto::store::RoomKeyInfo => RoomKeyInfo);

#[wasm_bindgen]
impl RoomKeyInfo {
    /// The {@link EncryptionAlgorithm} that this key is used for. Will be one
    /// of the `m.megolm.*` algorithms.
    #[wasm_bindgen(getter)]
    pub fn algorithm(&self) -> EncryptionAlgorithm {
        self.inner.algorithm.clone().into()
    }

    /// The room where the key is used.
    #[wasm_bindgen(getter, js_name = "roomId")]
    pub fn room_id(&self) -> RoomId {
        self.inner.room_id.clone().into()
    }

    /// The Curve25519 key of the device which initiated the session originally.
    #[wasm_bindgen(getter, js_name = "senderKey")]
    pub fn sender_key(&self) -> Curve25519PublicKey {
        self.inner.sender_key.into()
    }

    /// The ID of the session that the key is for.
    #[wasm_bindgen(getter, js_name = "sessionId")]
    pub fn session_id(&self) -> String {
        self.inner.session_id.clone()
    }
}
