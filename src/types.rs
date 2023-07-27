//! Extra types, like `Signatures`.

use js_sys::{JsString, Map};
use matrix_sdk_crypto::backups::{
    SignatureState as InnerSignatureState, SignatureVerification as InnerSignatureVerification,
};
use wasm_bindgen::prelude::*;

use crate::{
    identifiers::{DeviceKeyId, UserId},
    impl_from_to_inner,
    vodozemac::Ed25519Signature,
};

/// A collection of `Signature`.
#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct Signatures {
    inner: matrix_sdk_crypto::types::Signatures,
}

impl_from_to_inner!(matrix_sdk_crypto::types::Signatures => Signatures);

#[wasm_bindgen]
impl Signatures {
    /// Creates a new, empty, signatures collection.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        matrix_sdk_crypto::types::Signatures::new().into()
    }

    /// Add the given signature from the given signer and the given key ID to
    /// the collection.
    #[wasm_bindgen(js_name = "addSignature")]
    pub fn add_signature(
        &mut self,
        signer: &UserId,
        key_id: &DeviceKeyId,
        signature: &Ed25519Signature,
    ) -> Option<MaybeSignature> {
        self.inner
            .add_signature(signer.inner.clone(), key_id.inner.clone(), signature.inner)
            .map(Into::into)
    }

    /// Try to find an Ed25519 signature from the given signer with
    /// the given key ID.
    #[wasm_bindgen(js_name = "getSignature")]
    pub fn get_signature(&self, signer: &UserId, key_id: &DeviceKeyId) -> Option<Ed25519Signature> {
        self.inner.get_signature(signer.inner.as_ref(), key_id.inner.as_ref()).map(Into::into)
    }

    /// Get the map of signatures that belong to the given user.
    pub fn get(&self, signer: &UserId) -> Option<Map> {
        let map = Map::new();

        for (device_key_id, maybe_signature) in
            self.inner.get(signer.inner.as_ref()).map(|map| {
                map.iter().map(|(device_key_id, maybe_signature)| {
                    (
                        device_key_id.as_str().to_owned(),
                        MaybeSignature::from(maybe_signature.clone()),
                    )
                })
            })?
        {
            map.set(&device_key_id.into(), &maybe_signature.into());
        }

        Some(map)
    }

    /// Remove all the signatures we currently hold.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Do we hold any signatures or is our collection completely
    /// empty.
    #[wasm_bindgen(js_name = "isEmpty")]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// How many signatures do we currently hold.
    #[wasm_bindgen(getter)]
    pub fn count(&self) -> usize {
        self.inner.signature_count()
    }

    /// Get the json with all signatures
    #[wasm_bindgen(js_name = "asJSON")]
    pub fn as_json(&self) -> Result<JsString, JsError> {
        Ok(serde_json::to_string(&self.inner)?.into())
    }
}

/// Represents a potentially decoded signature (but not a validated
/// one).
#[wasm_bindgen]
#[derive(Debug)]
pub struct Signature {
    inner: matrix_sdk_crypto::types::Signature,
}

impl_from_to_inner!(matrix_sdk_crypto::types::Signature => Signature);

#[wasm_bindgen]
impl Signature {
    /// Get the Ed25519 signature, if this is one.
    #[wasm_bindgen(getter)]
    pub fn ed25519(&self) -> Option<Ed25519Signature> {
        self.inner.ed25519().map(Into::into)
    }

    /// Convert the signature to a base64 encoded string.
    #[wasm_bindgen(js_name = "toBase64")]
    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }
}

type MaybeSignatureInner =
    Result<matrix_sdk_crypto::types::Signature, matrix_sdk_crypto::types::InvalidSignature>;

/// Represents a signature that is either valid _or_ that could not be
/// decoded.
#[wasm_bindgen]
#[derive(Debug)]
pub struct MaybeSignature {
    inner: MaybeSignatureInner,
}

impl_from_to_inner!(MaybeSignatureInner => MaybeSignature);

#[wasm_bindgen]
impl MaybeSignature {
    /// Check whether the signature has been successfully decoded.
    #[wasm_bindgen(js_name = "isValid")]
    pub fn is_valid(&self) -> bool {
        self.inner.is_ok()
    }

    /// Check whether the signature could not be successfully decoded.
    #[wasm_bindgen(js_name = "isInvalid")]
    pub fn is_invalid(&self) -> bool {
        self.inner.is_err()
    }

    /// The signature, if successfully decoded.
    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> Option<Signature> {
        self.inner.as_ref().cloned().map(Into::into).ok()
    }

    /// The base64 encoded string that is claimed to contain a
    /// signature but could not be decoded, if any.
    #[wasm_bindgen(getter, js_name = "invalidSignatureSource")]
    pub fn invalid_signature_source(&self) -> Option<String> {
        match &self.inner {
            Ok(_) => None,
            Err(signature) => Some(signature.source.clone()),
        }
    }
}

/// The result of a signature verification of a signed JSON object.
#[derive(Debug)]
#[wasm_bindgen]
pub struct SignatureVerification {
    pub(crate) inner: InnerSignatureVerification,
}

/// The result of a signature check.
#[derive(Debug)]
#[wasm_bindgen]
pub enum SignatureState {
    /// The signature is missing.
    Missing = 0,
    /// The signature is invalid.
    Invalid = 1,
    /// The signature is valid but the device or user identity that created the
    /// signature is not trusted.
    ValidButNotTrusted = 2,
    /// The signature is valid and the device or user identity that created the
    /// signature is trusted.
    ValidAndTrusted = 3,
}

impl From<InnerSignatureState> for SignatureState {
    fn from(val: InnerSignatureState) -> Self {
        match val {
            InnerSignatureState::Missing => SignatureState::Missing,
            InnerSignatureState::Invalid => SignatureState::Invalid,
            InnerSignatureState::ValidButNotTrusted => SignatureState::ValidButNotTrusted,
            InnerSignatureState::ValidAndTrusted => SignatureState::ValidAndTrusted,
        }
    }
}

#[wasm_bindgen]
impl SignatureVerification {
    /// Give the backup signature state from the current device.
    /// See SignatureState for values
    #[wasm_bindgen(getter, js_name = "deviceState")]
    pub fn device_state(&self) -> SignatureState {
        self.inner.device_signature.into()
    }

    /// Give the backup signature state from the current user identity.
    /// See SignatureState for values
    #[wasm_bindgen(getter, js_name = "userState")]
    pub fn user_state(&self) -> SignatureState {
        self.inner.user_identity_signature.into()
    }

    /// Is the result considered to be trusted?
    ///
    /// This tells us if the result has a valid signature from any of the
    /// following:
    ///
    /// * Our own device
    /// * Our own user identity, provided the identity is trusted as well
    /// * Any of our own devices, provided the device is trusted as well
    #[wasm_bindgen()]
    pub fn trusted(&self) -> bool { self.inner.trusted() }
}
