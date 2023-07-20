//! Megolm backup types

use js_sys::JsString;
use matrix_sdk_crypto::{backups::MegolmV1BackupKey as InnerMegolmV1BackupKey, store};
use wasm_bindgen::prelude::*;

/// The private part of the backup key, the one used for recovery.
#[derive(Debug)]
#[wasm_bindgen]
pub struct BackupDecryptionKey {
    pub(crate) inner: store::BackupDecryptionKey,
    pub(crate) passphrase_info: Option<PassphraseInfo>,
}

/// Struct containing info about the way the backup key got derived from a
/// passphrase.
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct PassphraseInfo {
    /// The salt that was used during key derivation.
    #[wasm_bindgen(getter_with_clone)]
    pub private_key_salt: JsString,
    /// The number of PBKDF rounds that were used for key derivation.
    pub private_key_iterations: i32,
}

/// The public part of the backup key.
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct MegolmV1BackupKey {
    inner: InnerMegolmV1BackupKey,
    passphrase_info: Option<PassphraseInfo>,
}

#[wasm_bindgen]
impl MegolmV1BackupKey {
    /// The actual base64 encoded public key.
    #[wasm_bindgen(getter, js_name = "publicKeyBase64")]
    pub fn public_key(&self) -> JsString {
        self.inner.to_base64().into()
    }

    /// The passphrase info, if the key was derived from one.
    #[wasm_bindgen(getter, js_name = "passphraseInfo")]
    pub fn passphrase_info(&self) -> Option<PassphraseInfo> {
        self.passphrase_info.clone()
    }

    /// Get the full name of the backup algorithm this backup key supports.
    #[wasm_bindgen(getter, js_name = "algorithm")]
    pub fn backup_algorithm(&self) -> JsString {
        self.inner.backup_algorithm().into()
    }
}

#[wasm_bindgen]
impl BackupDecryptionKey {
    /// Create a new random [`BackupDecryptionKey`].
    #[wasm_bindgen(js_name = "createRandomKey")]
    pub fn create_random_key() -> BackupDecryptionKey {
        BackupDecryptionKey {
            inner: store::BackupDecryptionKey::new()
                .expect("Can't gather enough randomness to create a recovery key"),
            passphrase_info: None,
        }
    }

    /// Try to create a [`BackupDecryptionKey`] from a base 64 encoded string.
    #[wasm_bindgen(js_name = "fromBase64")]
    pub fn from_base64(key: String) -> Result<BackupDecryptionKey, JsError> {
        Ok(Self { inner: store::BackupDecryptionKey::from_base64(&key)?, passphrase_info: None })
    }

    /// Convert the recovery key to a base 64 encoded string.
    #[wasm_bindgen(js_name = "toBase64")]
    pub fn to_base64(&self) -> JsString {
        self.inner.to_base64().into()
    }

    /// Get the public part of the backup key.
    #[wasm_bindgen(getter, js_name = "megolmV1PublicKey")]
    pub fn megolm_v1_public_key(&self) -> MegolmV1BackupKey {
        let public_key = self.inner.megolm_v1_public_key();

        MegolmV1BackupKey { inner: public_key, passphrase_info: self.passphrase_info.clone() }
    }

    /// Try to decrypt a message that was encrypted using the public part of the
    /// backup key.
    #[wasm_bindgen(js_name = "decryptV1")]
    pub fn decrypt_v1(
        &self,
        ephemeral_key: String,
        mac: String,
        ciphertext: String,
    ) -> Result<String, JsError> {
        self.inner.decrypt_v1(&ephemeral_key, &mac, &ciphertext).map_err(|e| e.into())
    }
}
