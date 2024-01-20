// Copyright 2023 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Migration from libolm to Vodozemac.

use std::{iter, time::Duration};

use anyhow::Context;
use js_sys::{Date, Uint8Array};
use matrix_sdk_common::ruma::{
    DeviceKeyAlgorithm, MilliSecondsSinceUnixEpoch, SecondsSinceUnixEpoch, UInt,
};
use matrix_sdk_crypto::{
    olm::PrivateCrossSigningIdentity,
    store::{BackupDecryptionKey, Changes, DynCryptoStore, PendingChanges},
    types::EventEncryptionAlgorithm,
    vodozemac,
    vodozemac::{Curve25519PublicKey, Ed25519PublicKey},
    Session,
};
use wasm_bindgen::prelude::*;

use crate::{
    identifiers::{DeviceId, RoomId, UserId},
    store::StoreHandle,
};

type Result<T, E = JsError> = std::result::Result<T, E>;

/// Migration routines
///
/// The public methods are exposed as static methods on this class, for
/// namespacing and to enable easier mocking in unit tests.
#[wasm_bindgen()]
#[derive(Debug, Default)]
pub struct Migration {}

/// The base dataset that is important to migrate to the Rust SDK.
///
/// Can be imported into the rust store with {@link #migrateBaseData}.
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default)]
pub struct BaseMigrationData {
    /// The user id of the account owner.
    #[wasm_bindgen(js_name = "userId")]
    pub user_id: Option<UserId>,

    /// The device ID of the account owner.
    #[wasm_bindgen(js_name = "deviceId")]
    pub device_id: Option<DeviceId>,

    /// The pickle string holding the Olm Account, as returned by
    /// `olm_pickle_account` in libolm.
    #[wasm_bindgen(js_name = "pickledAccount")]
    pub pickled_account: String,

    /// The backup version that is currently active.
    #[wasm_bindgen(js_name = "backupVersion")]
    pub backup_version: Option<String>,

    /// The backup recovery key, as a base64-encoded string.
    #[wasm_bindgen(js_name = "backupRecoveryKey")]
    pub backup_recovery_key: Option<String>,

    /// The private, base64-encoded, master cross-signing key.
    #[wasm_bindgen(js_name = "privateCrossSigningMasterKey")]
    pub private_cross_signing_master_key: Option<String>,

    /// The private, base64-encoded, self-signing key.
    #[wasm_bindgen(js_name = "privateCrossSigningSelfSigningKey")]
    pub private_cross_signing_self_signing_key: Option<String>,

    /// The private, base64-encoded, user-signing key.
    #[wasm_bindgen(js_name = "privateCrossSigningUserSigningKey")]
    pub private_cross_signing_user_signing_key: Option<String>,
}

#[wasm_bindgen]
impl BaseMigrationData {
    /// Create a new `BaseMigrationData` with default values.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
impl Migration {
    /// Import the base dataset from a libolm-based setup to a vodozemac-based
    /// setup stored in IndexedDB.
    ///
    /// Populates the user credentials, Olm account, backup data, etc. This is
    /// the first step in the migration process. Once this base data is
    /// imported, further data can be imported with {@link
    /// #migrateOlmSessions}, {@link #migrateMegolmSessions}, and TODO room settings.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be migrated
    /// * `pickle_key` - The libolm pickle key that was used to pickle the olm
    ///   account objects.
    /// * `store_handle` - A connection to the CryptoStore which will be used to
    ///   store the vodozemac data.
    #[wasm_bindgen(js_name = "migrateBaseData")]
    pub async fn migrate_base_data(
        data: &BaseMigrationData,
        pickle_key: Uint8Array,
        store_handle: &StoreHandle,
    ) -> Result<JsValue, JsError> {
        migrate_base_data_to_store(data, &(pickle_key.to_vec()), store_handle.store.as_ref())
            .await
            .map_err(|e| JsError::from(&*e))?;
        Ok(JsValue::UNDEFINED)
    }
}

async fn migrate_base_data_to_store(
    data: &BaseMigrationData,
    pickle_key: &[u8],
    store: &DynCryptoStore,
) -> anyhow::Result<()> {
    let user_id = data.user_id.clone().context("User ID not specified")?.inner;
    let account = vodozemac::olm::Account::from_libolm_pickle(&data.pickled_account, pickle_key)?;
    let account =
        matrix_sdk_crypto::olm::Account::from_pickle(matrix_sdk_crypto::olm::PickledAccount {
            user_id: user_id.clone(),
            device_id: data.device_id.clone().context("Device ID not specified")?.inner,
            pickle: account.pickle(),
            // Legacy crypto in the js-sdk does not keep a record of whether it has published the
            // device keys to the server (it does it every time the stack is started). For safety,
            // let's assume it hasn't happened yet.
            shared: false,
            // Assume we have 50 keys on the server, until we get a sync that says fewer.
            uploaded_signed_key_count: 50,
            creation_local_time: MilliSecondsSinceUnixEpoch::now(),
        })?;

    let backup_decryption_key = data
        .backup_recovery_key
        .as_ref()
        .map(|k| BackupDecryptionKey::from_base64(k.as_str()))
        .transpose()?;

    let cross_signing = PrivateCrossSigningIdentity::empty(&user_id);
    cross_signing
        .import_secrets_unchecked(
            data.private_cross_signing_master_key.as_deref(),
            data.private_cross_signing_self_signing_key.as_deref(),
            data.private_cross_signing_user_signing_key.as_deref(),
        )
        .await?;

    store.save_pending_changes(PendingChanges { account: Some(account) }).await?;
    store
        .save_changes(Changes {
            private_identity: Some(cross_signing),
            backup_decryption_key,
            backup_version: data.backup_version.clone(),
            ..Default::default()
        })
        .await?;
    Ok(())
}

/// A pickled version of a `Session`.
///
/// Holds all the information that needs to be stored in a database to restore
/// an Olm Session. Can be imported into the rust store with {@link
/// #migrateOlmSessions}.
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug)]
pub struct PickledSession {
    /// The pickle string holding the Olm Session, as returned by
    /// `olm_pickle_session` in libolm.
    pub pickle: String,
    /// The base64-encoded public curve25519 key of the other user that we share
    /// this session with.
    #[wasm_bindgen(js_name = "senderKey")]
    pub sender_key: String,
    /// Was the session created using a fallback key?
    #[wasm_bindgen(js_name = "createdUsingFallbackKey")]
    pub created_using_fallback_key: bool,
    /// When the session was created.
    #[wasm_bindgen(js_name = "creationTime")]
    pub creation_time: Date,
    /// When the session was last used.
    #[wasm_bindgen(js_name = "lastUseTime")]
    pub last_use_time: Date,
}

#[wasm_bindgen]
impl PickledSession {
    /// Construct a new `PickledSession`, with default values.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
impl Migration {
    /// Migrate Olm sessions of a libolm-based setup to a vodozemac-based setup
    /// stored in an indexedDB crypto store.
    ///
    /// Before this method can be used, {@link #migrateBaseData} must be used to
    /// import the base data into the crypto store.
    ///
    /// This method should be called a number of times, with separate batches of
    /// `sessions`. If a progress display is given, it can be updated after
    /// each batch is successfully imported.
    ///
    /// # Arguments
    ///
    /// * `sessions` - An `Array` of {@link PickledSession}s to import. Items
    ///   inside `sessions` are going to be invalidated by this method.
    /// * `pickle_key` - The libolm pickle key that was used to pickle the olm
    ///   session objects.
    /// * `store_handle` - A connection to the CryptoStore which will be used to
    ///   store the vodozemac data.
    #[wasm_bindgen(js_name = "migrateOlmSessions")]
    pub async fn migrate_olm_sessions(
        sessions: Vec<PickledSession>,
        pickle_key: Uint8Array,
        store_handle: &StoreHandle,
    ) -> Result<JsValue, JsError> {
        let pickle_key = pickle_key.to_vec();

        let rust_sessions = sessions
            .into_iter()
            .map(|session| libolm_pickled_session_to_rust_pickled_session(session, &pickle_key))
            .collect::<Result<_>>()?;

        import_olm_sessions_to_store(rust_sessions, store_handle.store.as_ref())
            .await
            .map_err(|e| JsError::from(&*e))?;
        Ok(JsValue::UNDEFINED)
    }
}

impl Default for PickledSession {
    fn default() -> Self {
        Self {
            pickle: String::new(),
            sender_key: String::new(),
            created_using_fallback_key: false,
            creation_time: Date::new(&JsValue::from(0)),
            last_use_time: Date::new(&JsValue::from(0)),
        }
    }
}

fn libolm_pickled_session_to_rust_pickled_session(
    libolm_session: PickledSession,
    pickle_key: &[u8],
) -> Result<matrix_sdk_crypto::olm::PickledSession> {
    let session = vodozemac::olm::Session::from_libolm_pickle(&libolm_session.pickle, &pickle_key)?;

    let creation_time = date_to_seconds_since_epoch(&libolm_session.creation_time)
        .ok_or_else(|| JsError::new("session creation time out of range"))?;
    let last_use_time = date_to_seconds_since_epoch(&libolm_session.last_use_time)
        .ok_or_else(|| JsError::new("session last-use time out of range"))?;

    Ok(matrix_sdk_crypto::olm::PickledSession {
        pickle: session.pickle(),
        sender_key: Curve25519PublicKey::from_base64(&libolm_session.sender_key)?,
        created_using_fallback_key: libolm_session.created_using_fallback_key,
        creation_time,
        last_use_time,
    })
}

async fn import_olm_sessions_to_store(
    pickled_sessions: Vec<matrix_sdk_crypto::olm::PickledSession>,
    store: &DynCryptoStore,
) -> anyhow::Result<()> {
    let account = store
        .load_account()
        .await?
        .context("Base data must be imported before calling `migrateOlmSessions`")?;

    let user_id = account.user_id();
    let device_id = account.device_id();
    let identity_keys = &account.identity_keys;

    let sessions = pickled_sessions
        .into_iter()
        .map(|pickled_session| {
            Session::from_pickle(
                user_id.to_owned(),
                device_id.to_owned(),
                identity_keys.clone(),
                pickled_session,
            )
        })
        .collect();

    store.save_changes(Changes { sessions, ..Default::default() }).await?;
    Ok(())
}

/// A pickled version of an `InboundGroupSession`.
///
/// Holds all the information that needs to be stored in a database to restore
/// an InboundGroupSession.
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default)]
pub struct PickledInboundGroupSession {
    /// The pickle string holding the Megolm Session, as returned by
    /// `olm_pickle_inbound_group_session` in libolm.
    pub pickle: String,

    /// The public curve25519 key of the account that sent us the session.
    #[wasm_bindgen(js_name = "senderKey")]
    pub sender_key: String,

    /// The public ed25519 key of the account that sent us the session.
    #[wasm_bindgen(js_name = "senderSigningKey")]
    pub sender_signing_key: String,

    /// The id of the room that the session is used in.
    ///
    /// Nullable so that a `PickledInboundGroupSession` can be constructed
    /// incrementally. Must be populated!
    #[wasm_bindgen(js_name = "roomId")]
    pub room_id: Option<RoomId>,

    /// Flag remembering if the session was directly sent to us by the sender
    /// or if it was imported.
    pub imported: bool,

    /// Flag remembering if the session has been backed up.
    #[wasm_bindgen(js_name = "backedUp")]
    pub backed_up: bool,
}

#[wasm_bindgen]
impl PickledInboundGroupSession {
    /// Construct a new `PickledInboundGroupSession`, with default values.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
impl Migration {
    /// Migrate Megolm sessions of a libolm-based setup to a vodozemac-based
    /// setup stored in an indexedDB crypto store.
    ///
    /// Before this method can be used, {@link #migrateBaseData} must be used to
    /// import the base data into the crypto store.
    ///
    /// This method should be called a number of times, with separate batches of
    /// `sessions`. If a progress display is given, it can be updated after
    /// each batch is successfully imported.
    ///
    /// # Arguments
    ///
    /// * `sessions` - An `Array` of {@link PickledInboundGroupSession}s to
    ///   import. Items inside `sessions` are going to be invalidated by this
    ///   method.
    /// * `pickle_key` - The libolm pickle key that was used to pickle the
    ///   megolm session objects.
    /// * `store_handle` - A connection to the CryptoStore which will be used to
    ///   store the vodozemac data.
    #[wasm_bindgen(js_name = "migrateMegolmSessions")]
    pub async fn migrate_megolm_sessions(
        sessions: Vec<PickledInboundGroupSession>,
        pickle_key: Uint8Array,
        store_handle: &StoreHandle,
    ) -> Result<JsValue, JsError> {
        let pickle_key = pickle_key.to_vec();

        let rust_sessions = sessions
            .into_iter()
            .map(|session| {
                libolm_pickled_megolm_session_to_rust_pickled_session(session, &pickle_key)
            })
            .collect::<Result<_>>()?;

        import_megolm_sessions_to_store(rust_sessions, store_handle.store.as_ref())
            .await
            .map_err(|e| JsError::from(&*e))?;
        Ok(JsValue::UNDEFINED)
    }
}

fn libolm_pickled_megolm_session_to_rust_pickled_session(
    libolm_session: PickledInboundGroupSession,
    pickle_key: &[u8],
) -> Result<matrix_sdk_crypto::olm::PickledInboundGroupSession> {
    let pickle = vodozemac::megolm::InboundGroupSession::from_libolm_pickle(
        &libolm_session.pickle,
        pickle_key,
    )?
    .pickle();

    let sender_key = Curve25519PublicKey::from_base64(&libolm_session.sender_key)?;

    Ok(matrix_sdk_crypto::olm::PickledInboundGroupSession {
        pickle,
        sender_key,
        signing_key: iter::once((
            DeviceKeyAlgorithm::Ed25519,
            Ed25519PublicKey::from_base64(&libolm_session.sender_signing_key)?.into(),
        ))
        .collect(),
        room_id: libolm_session
            .room_id
            .clone()
            .ok_or_else(|| JsError::new("Room ID not specified for megolm session"))?
            .inner,
        imported: libolm_session.imported,
        backed_up: libolm_session.backed_up,
        history_visibility: None,
        algorithm: EventEncryptionAlgorithm::MegolmV1AesSha2,
    })
}

async fn import_megolm_sessions_to_store(
    pickled_sessions: Vec<matrix_sdk_crypto::olm::PickledInboundGroupSession>,
    store: &DynCryptoStore,
) -> anyhow::Result<()> {
    let inbound_group_sessions = pickled_sessions
        .into_iter()
        .map(matrix_sdk_crypto::olm::InboundGroupSession::from_pickle)
        .collect::<Result<_, _>>()?;

    store.save_changes(Changes { inbound_group_sessions, ..Default::default() }).await?;
    Ok(())
}

/// Convert a Javascript `Date` into `SecondsSinceUnixEpoch`.
///
/// Returns `None` if the Date cannot be represented as a
/// `SecondsSinceUnixEpoch`
fn date_to_seconds_since_epoch(date: &Date) -> Option<SecondsSinceUnixEpoch> {
    // javascript Dates are defined to be in milliseconds since the epoch
    let duration_since_epoch = Duration::from_millis(date.get_time() as u64);
    return Some(SecondsSinceUnixEpoch(UInt::new(duration_since_epoch.as_secs())?));
}
