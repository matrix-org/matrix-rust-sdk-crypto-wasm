//! Different verification types.

use js_sys::{Array, JsString};
use ruma::events::key::verification::{
    cancel::CancelCode as RumaCancelCode, VerificationMethod as RumaVerificationMethod,
};
use wasm_bindgen::prelude::*;

use crate::{
    identifiers::{DeviceId, RoomId, UserId},
    requests,
};

/// List of available verification methods.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum VerificationMethod {
    /// The `m.sas.v1` verification method.
    ///
    /// SAS means Short Authentication String.
    SasV1 = 0,

    /// The `m.qr_code.scan.v1` verification method.
    QrCodeScanV1 = 1,

    /// The `m.qr_code.show.v1` verification method.
    QrCodeShowV1 = 2,

    /// The `m.reciprocate.v1` verification method.
    ReciprocateV1 = 3,
}

impl From<VerificationMethod> for JsValue {
    fn from(value: VerificationMethod) -> Self {
        use VerificationMethod::*;

        match value {
            SasV1 => JsValue::from(0),
            QrCodeScanV1 => JsValue::from(1),
            QrCodeShowV1 => JsValue::from(2),
            ReciprocateV1 => JsValue::from(3),
        }
    }
}

impl TryFrom<JsValue> for VerificationMethod {
    type Error = JsError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let value = value.as_f64().ok_or_else(|| {
            JsError::new(&format!("Expect a `number`, received a `{:?}`", value.js_typeof()))
        })? as u32;

        Ok(match value {
            0 => Self::SasV1,
            1 => Self::QrCodeScanV1,
            2 => Self::QrCodeShowV1,
            3 => Self::ReciprocateV1,
            _ => {
                return Err(JsError::new(&format!(
                    "Unknown verification method (received `{:?}`)",
                    value
                )))
            }
        })
    }
}

impl From<VerificationMethod> for RumaVerificationMethod {
    fn from(value: VerificationMethod) -> Self {
        use VerificationMethod::*;

        match value {
            SasV1 => Self::SasV1,
            QrCodeScanV1 => Self::QrCodeScanV1,
            QrCodeShowV1 => Self::QrCodeShowV1,
            ReciprocateV1 => Self::ReciprocateV1,
        }
    }
}

impl TryFrom<RumaVerificationMethod> for VerificationMethod {
    type Error = JsError;

    fn try_from(value: RumaVerificationMethod) -> Result<Self, Self::Error> {
        use RumaVerificationMethod::*;

        Ok(match value {
            SasV1 => Self::SasV1,
            QrCodeScanV1 => Self::QrCodeScanV1,
            QrCodeShowV1 => Self::QrCodeShowV1,
            ReciprocateV1 => Self::ReciprocateV1,
            _ => {
                return Err(JsError::new(&format!(
                    "Unknown verification method (received `{:?}`)",
                    value
                )))
            }
        })
    }
}

pub(crate) struct Verification(pub(crate) matrix_sdk_crypto::Verification);

impl TryFrom<Verification> for JsValue {
    type Error = JsError;

    fn try_from(verification: Verification) -> Result<Self, Self::Error> {
        use matrix_sdk_crypto::Verification::*;

        Ok(match verification.0 {
            SasV1(sas) => JsValue::from(Sas { inner: sas }),

            #[cfg(feature = "qrcode")]
            QrV1(qr) => JsValue::from(Qr { inner: qr }),

            _ => {
                return Err(JsError::new(
                    "Unknown verification type, expect `m.sas.v1` only for now",
                ))
            }
        })
    }
}

/// Short Authentication String (SAS) verification.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Sas {
    inner: matrix_sdk_crypto::Sas,
}

#[wasm_bindgen]
impl Sas {
    /// Get our own user ID.
    #[wasm_bindgen(getter, js_name = "userId")]
    pub fn user_id(&self) -> UserId {
        self.inner.user_id().to_owned().into()
    }

    /// Get our own device ID.
    #[wasm_bindgen(getter, js_name = "deviceId")]
    pub fn device_id(&self) -> DeviceId {
        self.inner.device_id().to_owned().into()
    }

    /// Get the user id of the other side.
    #[wasm_bindgen(getter, js_name = "otherUserId")]
    pub fn other_user_id(&self) -> UserId {
        self.inner.other_user_id().to_owned().into()
    }

    /// Get the device ID of the other side.
    #[wasm_bindgen(getter, js_name = "otherDeviceId")]
    pub fn other_device_id(&self) -> DeviceId {
        self.inner.other_device_id().to_owned().into()
    }

    /*
    /// Get the device of the other user.
    #[wasm_bindgen(js_name = "otherDevice")]
    pub fn other_device(&self) {
        todo!()
    }
    */

    /// Get the unique ID that identifies this SAS verification flow,
    /// be either a to-device request ID or a room event ID.
    #[wasm_bindgen(getter, js_name = "flowId")]
    pub fn flow_id(&self) -> String {
        self.inner.flow_id().as_str().to_owned()
    }

    /// Get the room ID if the verification is happening inside a
    /// room.
    #[wasm_bindgen(getter, js_name = "roomId")]
    pub fn room_id(&self) -> Option<RoomId> {
        self.inner.room_id().map(ToOwned::to_owned).map(Into::into)
    }

    /// Does this verification flow support displaying emoji for the
    /// short authentication string.
    #[wasm_bindgen(js_name = "supportsEmoji")]
    pub fn supports_emoji(&self) -> bool {
        self.inner.supports_emoji()
    }

    /// Did this verification flow start from a verification request.
    #[wasm_bindgen(js_name = "startedFromRequest")]
    pub fn started_from_request(&self) -> bool {
        self.inner.started_from_request()
    }

    /// Is this a verification that is veryfying one of our own
    /// devices.
    #[wasm_bindgen(js_name = "isSelfVerification")]
    pub fn is_self_verification(&self) -> bool {
        self.inner.is_self_verification()
    }

    /// Have we confirmed that the short auth string matches.
    #[wasm_bindgen(js_name = "haveWeConfirmed")]
    pub fn have_we_confirmed(&self) -> bool {
        self.inner.have_we_confirmed()
    }

    /// Has the verification been accepted by both parties.
    #[wasm_bindgen(js_name = "hasBeenAccepted")]
    pub fn has_been_accepted(&self) -> bool {
        self.inner.has_been_accepted()
    }

    /// Get info about the cancellation if the verification flow has
    /// been cancelled.
    #[wasm_bindgen(js_name = "cancelInfo")]
    pub fn cancel_info(&self) -> Option<CancelInfo> {
        self.inner.cancel_info().map(Into::into)
    }

    /// Did we initiate the verification flow.
    #[wasm_bindgen(js_name = "weStarted")]
    pub fn we_started(&self) -> bool {
        self.inner.we_started()
    }

    /*
    pub fn accept(&self) {
        todo!()
    }
    */

    /*
    #[wasm_bindgen(js_name = "acceptWithSettings")]
    pub fn accept_with_settings(&self) {
        todo!()
    }
    */

    /*
    pub fn confirm(&self) {
        todo!()
    }
    */

    /*
    pub fn cancel(&self) {
        todo!()
    }
    */

    /*
    #[wasm_bindgen(js_name = "cancelWithCode")]
    pub fn cancel_with_code(&self) {
        todo!()
    }
    */

    /// Has the SAS verification flow timed out.
    #[wasm_bindgen(js_name = "timedOut")]
    pub fn timed_out(&self) -> bool {
        self.inner.timed_out()
    }

    /// Are we in a state where we can show the short auth string.
    #[wasm_bindgen(js_name = "canBePresented")]
    pub fn can_be_presented(&self) -> bool {
        self.inner.can_be_presented()
    }

    /// Is the SAS flow done.
    #[wasm_bindgen(js_name = "isDone")]
    pub fn is_done(&self) -> bool {
        self.inner.is_done()
    }

    /// Is the SAS flow canceled.
    #[wasm_bindgen(js_name = "isCancelled")]
    pub fn is_cancelled(&self) -> bool {
        self.inner.is_cancelled()
    }

    /// Get the emoji version of the short auth string.
    ///
    /// Returns `undefined` if we can't yet present the short auth string,
    /// otherwise seven tuples containing the emoji and description.
    pub fn emoji(&self) -> Option<Array> {
        Some(
            self.inner
                .emoji()?
                .iter()
                .map(|emoji| Emoji::from(emoji.to_owned()))
                .map(JsValue::from)
                .collect(),
        )
    }

    /// Get the index of the emoji representing the short auth string
    ///
    /// Returns `undefined` if we can’t yet present the short auth
    /// string, otherwise seven `u8` numbers in the range from 0 to 63
    /// inclusive which can be converted to an emoji using [the
    /// relevant specification
    /// entry](https://spec.matrix.org/unstable/client-server-api/#sas-method-emoji).
    #[wasm_bindgen(js_name = "emoji_index")]
    pub fn emoji_index(&self) -> Option<Array> {
        Some(self.inner.emoji_index()?.into_iter().map(JsValue::from).collect())
    }

    /// Get the decimal version of the short auth string.
    ///
    /// Returns None if we can’t yet present the short auth string,
    /// otherwise a tuple containing three 4-digit integers that
    /// represent the short auth string.
    pub fn decimals(&self) -> Option<Array> {
        let decimals = self.inner.decimals()?;

        let out = Array::new_with_length(3);
        out.set(0, JsValue::from(decimals.0));
        out.set(1, JsValue::from(decimals.1));
        out.set(2, JsValue::from(decimals.2));

        Some(out)
    }
}

/// QR code based verification.
#[cfg(feature = "qrcode")]
#[wasm_bindgen]
#[derive(Debug)]
pub struct Qr {
    inner: matrix_sdk_crypto::QrVerification,
}

#[cfg(feature = "qrcode")]
#[wasm_bindgen]
impl Qr {
    /// Has the QR verification been scanned by the other side.
    ///
    /// When the verification object is in this state it’s required
    /// that the user confirms that the other side has scanned the QR
    /// code.
    #[wasm_bindgen(js_name = "hasBeenScanned")]
    pub fn has_been_scanned(&self) -> bool {
        self.inner.has_been_scanned()
    }

    /// Has the scanning of the QR code been confirmed by us.
    #[wasm_bindgen(js_name = "hasBeenConfirmed")]
    pub fn has_been_confirmed(&self) -> bool {
        self.inner.has_been_confirmed()
    }

    /// Get our own user ID.
    #[wasm_bindgen(getter, js_name = "userId")]
    pub fn user_id(&self) -> UserId {
        self.inner.user_id().to_owned().into()
    }

    /// Get the user id of the other user that is participating in
    /// this verification flow.
    #[wasm_bindgen(getter, js_name = "otherUserId")]
    pub fn other_user_id(&self) -> UserId {
        self.inner.other_user_id().to_owned().into()
    }

    /// Get the device ID of the other side.
    #[wasm_bindgen(getter, js_name = "otherDeviceId")]
    pub fn other_device_id(&self) -> DeviceId {
        self.inner.other_device_id().to_owned().into()
    }

    /*
    #[wasm_bindgen(getter, js_name = "otherDevice")]
    pub fn other_device(&self) -> ReadOnlyDevice {}
    */

    /// Did we initiate the verification request.
    #[wasm_bindgen(js_name = "weStarted")]
    pub fn we_started(&self) -> bool {
        self.inner.we_started()
    }

    /// Get info about the cancellation if the verification flow has
    /// been cancelled.
    #[wasm_bindgen(js_name = "cancelInfo")]
    pub fn cancel_info(&self) -> Option<CancelInfo> {
        self.inner.cancel_info().map(Into::into)
    }

    /// Has the verification flow completed.
    #[wasm_bindgen(js_name = "isDone")]
    pub fn is_done(&self) -> bool {
        self.inner.is_done()
    }

    /// Has the verification flow been cancelled.
    #[wasm_bindgen(js_name = "isCancelled")]
    pub fn is_cancelled(&self) -> bool {
        self.inner.is_cancelled()
    }

    /// Is this a verification that is veryfying one of our own devices.
    #[wasm_bindgen(js_name = "isSelfVerification")]
    pub fn is_self_verification(&self) -> bool {
        self.inner.is_self_verification()
    }

    /// Have we successfully scanned the QR code and are able to send
    /// a reciprocation event.
    pub fn reciprocated(&self) -> bool {
        self.inner.reciprocated()
    }

    /// Get the unique ID that identifies this QR verification flow,
    /// be either a to-device request ID or a room event ID.
    #[wasm_bindgen(getter, js_name = "flowId")]
    pub fn flow_id(&self) -> String {
        self.inner.flow_id().as_str().to_owned()
    }

    /// Get the room id if the verification is happening inside a
    /// room.
    #[wasm_bindgen(getter, js_name = "roomId")]
    pub fn room_id(&self) -> Option<RoomId> {
        self.inner.room_id().map(ToOwned::to_owned).map(Into::into)
    }

    /// Generate a QR code object that is representing this
    /// verification flow.
    ///
    /// The QrCode can then be rendered as an image or as an unicode
    /// string.
    ///
    /// The `to_bytes` method can be used to instead output the raw
    /// bytes that should be encoded as a QR code.
    #[wasm_bindgen(js_name = "toQrCode")]
    pub fn to_qr_code(&self) -> Result<QrCode, JsError> {
        Ok(self.inner.to_qr_code().map(Into::into)?)
    }

    /// Generate a the raw bytes that should be encoded as a QR code
    /// is representing this verification flow.
    ///
    /// The `to_qr_code` method can be used to instead output a QrCode
    /// object that can be rendered.
    #[wasm_bindgen(js_name = "toBytes")]
    pub fn to_bytes(&self) -> Result<Array, JsError> {
        Ok(self.inner.to_bytes()?.into_iter().map(JsValue::from).collect())
    }

    /*
    /// Cancel the verification flow.
    pub fn cancel(&self) -> … {}
    pub fn cancel_with_code(&self, code: …) -> … {}
    pub fn reciprocate(&self) -> … {}
    pub fn confirm_scanning(&self) -> … {}
    */
}

/// Information about the cancellation of a verification request or
/// verification flow.
#[wasm_bindgen]
#[derive(Debug)]
pub struct CancelInfo {
    inner: matrix_sdk_crypto::CancelInfo,
}

impl From<matrix_sdk_crypto::CancelInfo> for CancelInfo {
    fn from(inner: matrix_sdk_crypto::CancelInfo) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl CancelInfo {
    /// Get the human readable reason of the cancellation.
    pub fn reason(&self) -> JsString {
        self.inner.reason().into()
    }

    /// Get the `CancelCode` that cancelled this verification.
    #[wasm_bindgen(js_name = "cancelCode")]
    pub fn cancel_code(&self) -> CancelCode {
        self.inner.cancel_code().into()
    }

    /// Was the verification cancelled by us?
    #[wasm_bindgen(js_name = "cancelledbyUs")]
    pub fn cancelled_by_us(&self) -> bool {
        self.inner.cancelled_by_us()
    }
}

/// An error code for why the process/request was cancelled by the
/// user.
#[wasm_bindgen]
#[derive(Debug)]
pub enum CancelCode {
    /// Unknown cancel code.
    Other,

    /// The user cancelled the verification.
    User,

    /// The verification process timed out.
    ///
    /// Verification processes can define their own timeout
    /// parameters.
    Timeout,

    /// The device does not know about the given transaction ID.
    UnknownTransaction,

    /// The device does not know how to handle the requested method.
    ///
    /// Should be sent for `m.key.verification.start` messages and
    /// messages defined by individual verification processes.
    UnknownMethod,

    /// The device received an unexpected message.
    ///
    /// Typically raised when one of the parties is handling the
    /// verification out of order.
    UnexpectedMessage,

    /// The key was not verified.
    KeyMismatch,

    /// The expected user did not match the user verified.
    UserMismatch,

    /// The message received was invalid.
    InvalidMessage,

    /// An `m.key.verification.request` was accepted by a different
    /// device.
    ///
    /// The device receiving this error can ignore the verification
    /// request.
    Accepted,

    /// The device receiving this error can ignore the verification
    /// request.
    MismatchedCommitment,

    /// The SAS did not match.
    MismatchedSas,
}

impl From<&RumaCancelCode> for CancelCode {
    fn from(code: &RumaCancelCode) -> Self {
        use RumaCancelCode::*;

        match code {
            User => Self::User,
            Timeout => Self::Timeout,
            UnknownTransaction => Self::UnknownTransaction,
            UnknownMethod => Self::UnknownMethod,
            UnexpectedMessage => Self::UnexpectedMessage,
            KeyMismatch => Self::KeyMismatch,
            UserMismatch => Self::UserMismatch,
            InvalidMessage => Self::InvalidMessage,
            Accepted => Self::Accepted,
            MismatchedCommitment => Self::MismatchedCommitment,
            MismatchedSas => Self::MismatchedSas,
            _ => Self::Other,
        }
    }
}

/// An emoji that is used for interactive verification using a short
/// auth string.
///
/// This will contain a single emoji and description from the list of
/// emojis from [the specification].
///
/// [the specification]: https://spec.matrix.org/unstable/client-server-api/#sas-method-emoji
#[wasm_bindgen]
#[derive(Debug)]
pub struct Emoji {
    inner: matrix_sdk_crypto::Emoji,
}

impl From<matrix_sdk_crypto::Emoji> for Emoji {
    fn from(inner: matrix_sdk_crypto::Emoji) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl Emoji {
    /// The emoji symbol that represents a part of the short auth
    /// string, for example: 🐶
    #[wasm_bindgen(getter)]
    pub fn symbol(&self) -> JsString {
        self.inner.symbol.into()
    }

    /// The description of the emoji, for example ‘Dog’.
    #[wasm_bindgen(getter)]
    pub fn description(&self) -> JsString {
        self.inner.description.into()
    }
}

/// A QR code.
#[cfg(feature = "qrcode")]
#[wasm_bindgen]
pub struct QrCode {
    inner: matrix_sdk_qrcode::qrcode::QrCode,
}

#[cfg(feature = "qrcode")]
impl fmt::Debug for QrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(QrCode)).finish()
    }
}

#[cfg(feature = "qrcode")]
impl From<matrix_sdk_qrcode::qrcode::QrCode> for QrCode {
    fn from(inner: matrix_sdk_qrcode::qrcode::QrCode) -> Self {
        Self { inner }
    }
}

#[cfg(feature = "qrcode")]
#[wasm_bindgen]
impl QrCode {
    /// Render the QR code into a `Uint8Array` where 1 represents a
    /// dark pixel and 0 a white pixel.
    pub fn render_into_buffer(&self) -> Result<Uint8Array, JsError> {
        let colors: Vec<u8> =
            self.inner.to_colors().into_iter().map(|color| color.select(1u8, 0u8)).collect();
        let buffer = Uint8Array::new_with_length(colors.len().try_into()?);
        buffer.copy_from(colors.as_slice());

        Ok(buffer)
    }
}

/// An object controlling key verification requests.
///
/// Interactive verification flows usually start with a verification
/// request, this object lets you send and reply to such a
/// verification request.
///
/// After the initial handshake the verification flow transitions into
/// one of the verification methods.
#[wasm_bindgen]
#[derive(Debug)]
pub struct VerificationRequest {
    inner: matrix_sdk_crypto::VerificationRequest,
}

impl From<matrix_sdk_crypto::VerificationRequest> for VerificationRequest {
    fn from(inner: matrix_sdk_crypto::VerificationRequest) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl VerificationRequest {
    #[wasm_bindgen]
    pub fn request(
        own_user_id: &UserId,
        own_device_id: &DeviceId,
        other_user_id: &UserId,
        methods: Option<Array>,
    ) -> Result<String, JsError> {
        let methods: Option<Vec<RumaVerificationMethod>> = methods
            .map(|array| {
                array
                    .iter()
                    .map(|method| VerificationMethod::try_from(method).map(Into::into))
                    .collect::<Result<_, _>>()
            })
            .transpose()?;

        Ok(serde_json::to_string(&matrix_sdk_crypto::VerificationRequest::request(
            &own_user_id.inner,
            &own_device_id.inner,
            &other_user_id.inner,
            methods,
        ))?)
    }

    /// Our own user id.
    #[wasm_bindgen(js_name = "ownUserId")]
    pub fn own_user_id(&self) -> UserId {
        self.inner.own_user_id().to_owned().into()
    }

    /// The ID of the other user that is participating in this
    /// verification request.
    #[wasm_bindgen(js_name = "otherUserId")]
    pub fn other_user_id(&self) -> UserId {
        self.inner.other_user().to_owned().into()
    }

    /// The ID of the other device that is participating in this
    /// verification.
    #[wasm_bindgen(js_name = "otherDeviceId")]
    pub fn other_device_id(&self) -> Option<DeviceId> {
        self.inner.other_device_id().map(Into::into)
    }

    /// Get the room ID if the verification is happening inside a
    /// room.
    #[wasm_bindgen(js_name = "roomId")]
    pub fn room_id(&self) -> Option<RoomId> {
        self.inner.room_id().map(ToOwned::to_owned).map(Into::into)
    }

    /// Get info about the cancellation if the verification request
    /// has been cancelled.
    #[wasm_bindgen(js_name = "cancelInfo")]
    pub fn cancel_info(&self) -> Option<CancelInfo> {
        self.inner.cancel_info().map(Into::into)
    }

    /// Has the verification request been answered by another device.
    #[wasm_bindgen(js_name = "isPassive")]
    pub fn is_passive(&self) -> bool {
        self.inner.is_passive()
    }

    /// Is the verification request ready to start a verification flow.
    #[wasm_bindgen(js_name = "isReady")]
    pub fn is_ready(&self) -> bool {
        self.inner.is_ready()
    }

    /// Has the verification flow timed out.
    #[wasm_bindgen(js_name = "timedOut")]
    pub fn timed_out(&self) -> bool {
        self.inner.timed_out()
    }

    /// Get the supported verification methods of the other side.
    ///
    /// Will be present only if the other side requested the
    /// verification or if we’re in the ready state.
    ///
    /// It return a `Option<Vec<VerificationMethod>>`.
    #[wasm_bindgen(js_name = "theirSupportedMethods")]
    pub fn their_supported_methods(&self) -> Result<Option<Array>, JsError> {
        self.inner
            .their_supported_methods()
            .map(|methods| {
                methods
                    .into_iter()
                    .map(|method| VerificationMethod::try_from(method).map(JsValue::from))
                    .collect::<Result<Array, _>>()
            })
            .transpose()
    }

    /// Get our own supported verification methods that we advertised.
    ///
    /// Will be present only we requested the verification or if we’re
    /// in the ready state.
    #[wasm_bindgen(js_name = "ourSupportedMethods")]
    pub fn our_supported_methods(&self) -> Result<Option<Array>, JsError> {
        self.inner
            .our_supported_methods()
            .map(|methods| {
                methods
                    .into_iter()
                    .map(|method| VerificationMethod::try_from(method).map(JsValue::from))
                    .collect::<Result<Array, _>>()
            })
            .transpose()
    }

    /// Get the unique ID of this verification request
    #[wasm_bindgen(js_name = "flowId")]
    pub fn flow_id(&self) -> String {
        self.inner.flow_id().as_str().to_owned()
    }

    /// Is this a verification that is veryfying one of our own
    /// devices.
    #[wasm_bindgen(js_name = "isSelfVerification")]
    pub fn is_self_verification(&self) -> bool {
        self.inner.is_self_verification()
    }

    /// Did we initiate the verification request.
    #[wasm_bindgen(js_name = "weStarted")]
    pub fn we_started(&self) -> bool {
        self.inner.we_started()
    }

    /// Has the verification flow that was started with this request
    /// finished.
    #[wasm_bindgen(js_name = "isDone")]
    pub fn is_done(&self) -> bool {
        self.inner.is_done()
    }

    /// Has the verification flow that was started with this request
    /// been cancelled.
    #[wasm_bindgen(js_name = "isCancelled")]
    pub fn is_cancelled(&self) -> bool {
        self.inner.is_cancelled()
    }

    /// Accept the verification request signaling that our client
    /// supports the given verification methods.
    ///
    /// `methods` represents the methods that we should advertise as
    /// supported by us.
    ///
    /// It returns either a `ToDeviceRequest`, a `RoomMessageRequest`
    /// or `undefined`.
    #[wasm_bindgen(js_name = "acceptWithMethods")]
    pub fn accept_with_methods(&self, methods: Array) -> Result<JsValue, JsError> {
        let methods: Vec<RumaVerificationMethod> = methods
            .iter()
            .map(|method| VerificationMethod::try_from(method).map(Into::into))
            .collect::<Result<_, _>>()?;

        self.inner
            .accept_with_methods(methods)
            .map(OutgoingVerificationRequest)
            .map(JsValue::try_from)
            .transpose()
            .map(JsValue::from)
            .map_err(Into::into)
    }

    /// Accept the verification request.
    ///
    /// This method will accept the request and signal that it
    /// supports the `m.sas.v1`, the `m.qr_code.show.v1`, and
    /// `m.reciprocate.v1` method.
    ///
    /// `m.qr_code.show.v1` will only be signaled if the `qrcode`
    /// feature is enabled. This feature is disabled by default. If
    /// it's enabled and QR code scanning should be supported or QR
    /// code showing shouldn't be supported the `accept_with_methods`
    /// method should be used instead.
    ///
    /// It returns either a `ToDeviceRequest`, a `RoomMessageRequest`
    /// or `undefined`.
    pub fn accept(&self) -> Result<JsValue, JsError> {
        self.inner
            .accept()
            .map(OutgoingVerificationRequest)
            .map(JsValue::try_from)
            .transpose()
            .map(JsValue::from)
            .map_err(Into::into)
    }

    /// Cancel the verification request.
    ///
    /// It returns either a `ToDeviceRequest`, a `RoomMessageRequest`
    /// or `undefined`.
    pub fn cancel(&self) -> Result<JsValue, JsError> {
        self.inner
            .cancel()
            .map(OutgoingVerificationRequest)
            .map(JsValue::try_from)
            .transpose()
            .map(JsValue::from)
            .map_err(Into::into)
    }

    // start_sas
    // generate_qr_code if `qrcode`
    // scan_qr_code if `qrcode`
}

// JavaScript has no complex enums like Rust. To return structs of
// different types, we have no choice that hiding everything behind a
// `JsValue`.
struct OutgoingVerificationRequest(pub(crate) matrix_sdk_crypto::OutgoingVerificationRequest);

impl TryFrom<OutgoingVerificationRequest> for JsValue {
    type Error = serde_json::Error;

    fn try_from(outgoing_request: OutgoingVerificationRequest) -> Result<Self, Self::Error> {
        use matrix_sdk_crypto::OutgoingVerificationRequest::*;

        let request_id = outgoing_request.0.request_id().to_string();

        Ok(match outgoing_request.0 {
            ToDevice(request) => {
                JsValue::from(requests::ToDeviceRequest::try_from((request_id, &request))?)
            }

            InRoom(request) => {
                JsValue::from(requests::RoomMessageRequest::try_from((request_id, &request))?)
            }
        })
    }
}