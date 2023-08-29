//! Errors related to room event decryption.

use js_sys::JsString;
use matrix_sdk_crypto::{vodozemac, MegolmError};
use wasm_bindgen::prelude::wasm_bindgen;

/// Decryption error codes
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum DecryptionErrorCode {
    /// The room key is not known
    MissingRoomKey,
    /// The room key is known but ratcheted
    UnknownMessageIndex,
    /// Decryption failed because of a mismatch between the identity keys of the
    /// device we received the room key from and the identity keys recorded in
    /// the plaintext of the room key to-device message.
    MismatchedIdentityKeys,
    /// Other failuer
    UnableToDecrypt,
}

/// Js Decryption error with code.
#[derive(Debug)]
#[wasm_bindgen(getter_with_clone)]
pub struct MegolmDecryptionError {
    /// Description code for the error. See `DecryptionErrorCode`
    #[wasm_bindgen(readonly)]
    pub code: DecryptionErrorCode,
    /// detailed description
    #[wasm_bindgen(readonly)]
    pub description: JsString,
    /// Witheld code if any. Only for `UnknownMessageIndex` error code
    #[wasm_bindgen(readonly)]
    pub maybe_withheld: Option<JsString>,
}

impl MegolmDecryptionError {
    /// Creates generic error with description
    pub fn unable_to_decrypt(desc: String) -> Self {
        Self {
            code: DecryptionErrorCode::UnableToDecrypt,
            description: desc.into(),
            maybe_withheld: None,
        }
    }
}

impl From<MegolmError> for MegolmDecryptionError {
    fn from(value: MegolmError) -> Self {
        match &value {
            MegolmError::MissingRoomKey(withheld_code) => MegolmDecryptionError {
                code: DecryptionErrorCode::MissingRoomKey,
                description: value.to_string().into(),
                maybe_withheld: withheld_code
                    .as_ref()
                    .map(|code| code.to_string().to_owned().into()),
            },
            MegolmError::Decryption(vodozemac::megolm::DecryptionError::UnknownMessageIndex(
                ..,
            )) => MegolmDecryptionError {
                code: DecryptionErrorCode::UnknownMessageIndex,
                description: value.to_string().into(),
                maybe_withheld: None,
            },
            MegolmError::MismatchedIdentityKeys { .. } => MegolmDecryptionError {
                code: DecryptionErrorCode::UnknownMessageIndex,
                description: value.to_string().into(),
                maybe_withheld: None,
            },
            _ => MegolmDecryptionError {
                code: DecryptionErrorCode::UnableToDecrypt,
                description: value.to_string().into(),
                maybe_withheld: None,
            },
        }
    }
}
