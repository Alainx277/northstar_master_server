use std::net::IpAddr;
use thiserror::Error;

use crate::api::ApiErrorKind;

#[derive(Error, Debug)]
pub enum VerifyServerError {
    #[error("could not connect to game server auth")]
    ConnectionFailed,
    #[error("server did not respond with HTTP, is another service running?")]
    WrongProtocol,
    #[error("server did not respond with the correct message")]
    WrongResponse,
    #[error("an unexpected error occured")]
    Unknown,
}

impl From<reqwest::Error> for VerifyServerError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_connect() {
            Self::ConnectionFailed
        } else {
            Self::WrongProtocol
        }
    }
}

impl ApiErrorKind for VerifyServerError {
    fn kind(&self) -> &'static str {
        match self {
            VerifyServerError::ConnectionFailed => "NO_GAMESERVER_RESPONSE",
            VerifyServerError::WrongResponse | VerifyServerError::WrongProtocol => {
                "BAD_GAMESERVER_RESPONSE"
            }
            VerifyServerError::Unknown => "UNKNOWN",
        }
    }
}

static SERVER_VERIFY_TEXT: &str = "I am a northstar server!";

/// Tries to confirm that a Northstar server is running at the specified address
pub async fn verify_server(address: IpAddr, auth_port: u16) -> Result<(), VerifyServerError> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{address}:{auth_port}/verify"))
        .send()
        .await?;

    let text = response.text().await?;
    if text == SERVER_VERIFY_TEXT {
        Ok(())
    } else {
        Err(VerifyServerError::WrongResponse)
    }
}
