use std::{io::Read, net::SocketAddr};

use futures_util::StreamExt;
use serde::Deserialize;
use thiserror::Error;
use tracing::debug;
use warp::{multipart::FormData, Buf};

use crate::{accounts::default_persistent_data, api::ApiErrorKind, id::UniqueId, SharedServerList};

use super::{AccountId, AccountRepository};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct WritePersistenceParam {
    id: AccountId,
    server_id: UniqueId,
}

#[derive(Error, Debug)]
pub(super) enum WritePersistenceError {
    #[error("account does not exist")]
    InvalidAccount,
    #[error("this server is not allowed to update the persistent data")]
    NotPermitted,
    #[error("persistent data is invalid")]
    InvalidData,
    #[error("persistent data is missing")]
    MissingData,
}

impl ApiErrorKind for WritePersistenceError {
    fn kind(&self) -> &'static str {
        match self {
            WritePersistenceError::InvalidAccount => "PLAYER_NOT_FOUND",
            WritePersistenceError::NotPermitted => "UNAUTHORIZED_GAMESERVER",
            WritePersistenceError::InvalidData | WritePersistenceError::MissingData => {
                "INVALID_PERSISTENT_DATA"
            }
        }
    }
}

pub(super) async fn write_persistence(
    param: WritePersistenceParam,
    mut data: FormData,
    remote: Option<SocketAddr>,
    accounts: AccountRepository,
    servers: SharedServerList,
) -> Result<(), WritePersistenceError> {
    let ip = remote.unwrap().ip();

    if !accounts.exists(param.id).await.unwrap() {
        return Err(WritePersistenceError::InvalidAccount);
    }

    let auth_data = accounts
        .get_auth(param.id)
        .await
        .map_err(|_| WritePersistenceError::NotPermitted)?;
    let mut allowed = auth_data.last_auth_ip == ip;
    if !allowed {
        // Check if player is on given server and request was sent by it
        if let Some(current_server_id) = auth_data.current_server {
            if param.server_id == current_server_id {
                let servers = servers.read().await;
                if let Some(server) = servers.get(&param.server_id) {
                    if server.ip() == ip {
                        allowed = true;
                    }
                }
            }
        }
    }

    if !allowed {
        return Err(WritePersistenceError::NotPermitted);
    }

    // TODO: Way too many unwraps here, figure out how to clean this mess
    let mut file = data
        .next()
        .await
        .ok_or(WritePersistenceError::MissingData)
        .unwrap()
        .unwrap();
    let mut buffer = Vec::new();
    file.data()
        .await
        .unwrap()
        .unwrap()
        .reader()
        .read_to_end(&mut buffer)
        .unwrap();
    let expected_length = default_persistent_data().len();
    if buffer.len() != expected_length {
        debug!(
            expected_length,
            actual_length = buffer.len(),
            "invalid persistent data length"
        );
        return Err(WritePersistenceError::InvalidData);
    }

    accounts
        .set_data(param.id, &buffer)
        .await
        .expect("Error writing account persistent data");
    Ok(())
}
