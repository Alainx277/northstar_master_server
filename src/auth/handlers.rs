use std::net::{IpAddr, SocketAddr};

use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use tracing::debug;

use crate::{
    accounts::{AccountId, AccountRepository},
    api::ApiErrorKind,
    id::UniqueId,
    SharedServerList,
};

#[derive(Deserialize)]
pub(super) struct OriginAuthenticationParam {
    id: AccountId,
    token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StryderParam {
    qt: &'static str,
    #[serde(rename = "type")]
    kind: &'static str,
    code: String,
    force_trial: u32,
    proto: u32,
    json: u32,
    env: &'static str,
    user_id: String,
}

impl StryderParam {
    fn new(user_id: AccountId, code: String) -> Self {
        Self {
            qt: "origin-requesttoken",
            kind: "server_token",
            code,
            force_trial: 0,
            proto: 0,
            json: 1,
            env: "production",
            user_id: hex::encode_upper(user_id.0.to_be_bytes()),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StryderResponse {
    has_online_access: char,
    store_uri: String,
}

#[derive(Error, Debug)]
pub(super) enum OriginAuthenticationError {
    #[error("error while communicating with stryder api")]
    StryderError(#[from] Option<reqwest::Error>),
    #[error("you do not appear to own Titanfall 2")]
    NoGame,
}

impl ApiErrorKind for OriginAuthenticationError {
    fn kind(&self) -> &'static str {
        match self {
            OriginAuthenticationError::StryderError(_) => "STRYDER_RESPONSE",
            OriginAuthenticationError::NoGame => "UNAUTHORIZED_GAME",
        }
    }
}

#[derive(Serialize)]
pub(super) struct OriginAuthenticationResponse {
    token: UniqueId,
}

pub(super) async fn origin_authentication(
    param: OriginAuthenticationParam,
    remote: Option<SocketAddr>,
    accounts: crate::accounts::AccountRepository,
) -> Result<OriginAuthenticationResponse, OriginAuthenticationError> {
    let ip = remote.unwrap().ip();

    // Check if token is valid and user owns titanfall
    let stryder_param = StryderParam::new(param.id, param.token);
    debug!(
        user_id = stryder_param.user_id.as_str(),
        "requesting stryder auth"
    );
    let client = reqwest::Client::new();
    let response = client
        .get("https://r2-pc.stryder.respawn.com/nucleus-oauth.php")
        .query(&stryder_param)
        .send()
        .await?;

    // The returned JSON is not spec compliant
    // It contains literal newlines in strings
    // PHP moment
    // let text = response.text().await?;
    // if text.contains("\"success\": false") {
    //     return Err(OriginAuthenticationError::StryderError(None));
    // }

    let response: StryderResponse = response.json().await?;
    debug!(?response, account_id = param.id.0, "stryder response");
    if response.has_online_access == '1' && response.store_uri.contains("titanfall-2") {
        if !accounts.exists(param.id).await.unwrap() {
            accounts
                .create(param.id)
                .await
                .expect("Unable to create account");
        }

        let token = accounts
            .create_token(param.id, ip)
            .await
            .expect("Unable to create token");

        Ok(OriginAuthenticationResponse { token })
    } else {
        Err(OriginAuthenticationError::NoGame)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AuthenticateSelfParam {
    id: AccountId,
    player_token: UniqueId,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AuthenticateSelfResponse {
    id: String,
    auth_token: UniqueId,
    persistent_data: std::borrow::Cow<'static, [u8]>,
}

#[derive(Error, Debug)]
pub(super) enum AuthenticateSelfError {
    #[error("token is not valid")]
    InvalidToken,
}

impl ApiErrorKind for AuthenticateSelfError {
    fn kind(&self) -> &'static str {
        match self {
            AuthenticateSelfError::InvalidToken => "INVALID_MASTERSERVER_TOKEN",
        }
    }
}

pub(super) async fn authenticate_self(
    param: AuthenticateSelfParam,
    accounts: AccountRepository,
) -> Result<AuthenticateSelfResponse, AuthenticateSelfError> {
    let authenticated = accounts
        .authenticate(param.id, param.player_token)
        .await
        .unwrap();

    if !authenticated {
        return Err(AuthenticateSelfError::InvalidToken);
    }

    let auth_token = UniqueId::new(&mut rand::thread_rng());

    let data = accounts
        .get_data(param.id)
        .await
        .expect("Unable to read account data");

    Ok(AuthenticateSelfResponse {
        id: param.id.to_string(),
        auth_token,
        persistent_data: data,
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AuthenticateParam {
    id: AccountId,
    player_token: UniqueId,
    server: UniqueId,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    password: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AuthenticateResponse {
    ip: IpAddr,
    port: u16,
    auth_token: String,
}

#[derive(Error, Debug)]
pub(super) enum AuthenticateError {
    #[error("token is not valid")]
    InvalidToken,
    #[error("no game server with this id exists")]
    NoServer,
    #[error("password is incorrect")]
    WrongPassword,
    #[error("couldn't connect to game server")]
    Connection,
    #[error("game server didn't respond correctly")]
    WrongResponse,
}

impl ApiErrorKind for AuthenticateError {
    fn kind(&self) -> &'static str {
        match self {
            AuthenticateError::InvalidToken => "INVALID_MASTERSERVER_TOKEN",
            AuthenticateError::NoServer => "SERVER_NOT_FOUND",
            AuthenticateError::WrongPassword => "UNAUTHORIZED_PWD",
            AuthenticateError::WrongResponse => "BAD_GAMESERVER_RESPONSE",
            AuthenticateError::Connection => "NO_GAMESERVER_RESPONSE",
        }
    }
}

impl From<reqwest::Error> for AuthenticateError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_connect() {
            AuthenticateError::Connection
        } else {
            AuthenticateError::WrongResponse
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthenticateIncomingParam {
    id: AccountId,
    auth_token: String,
    server_auth_token: UniqueId,
    username: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthenticateIncomingResponse {
    success: bool,
}

pub(super) async fn authenticate(
    param: AuthenticateParam,
    accounts: AccountRepository,
    servers: SharedServerList,
) -> Result<AuthenticateResponse, AuthenticateError> {
    let authenticated = accounts
        .authenticate(param.id, param.player_token)
        .await
        .unwrap();

    if !authenticated {
        return Err(AuthenticateError::InvalidToken);
    }

    let servers = servers.read().await;
    let server = servers
        .get(&param.server)
        .ok_or(AuthenticateError::NoServer)?;

    if !server.check_password(param.password) {
        return Err(AuthenticateError::WrongPassword);
    }

    let auth_token = UniqueId::new(&mut rand::thread_rng());
    // TODO: Remove truncation (apparent limitation in original implementation)
    let mut truncated = auth_token.to_string();
    truncated.truncate(20);

    // Get persistent account data
    let data = accounts
        .get_data(param.id)
        .await
        .expect("Unable to read account data");

    // Tell the game server there will be a player joining
    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(reqwest_tracing::TracingMiddleware)
        .build();
    let response: AuthenticateIncomingResponse = client
        .post(format!(
            "http://{}/authenticate_incoming_player",
            server.auth_address()
        ))
        .query(&AuthenticateIncomingParam {
            id: param.id,
            auth_token: truncated.clone(),
            server_auth_token: server.auth_token(),
            username: accounts
                .get_name(param.id)
                .await
                .unwrap()
                .unwrap_or_default(),
        })
        .body(data.into_owned())
        .send()
        .await
        .map_err(|err| match err {
            reqwest_middleware::Error::Middleware(err) => panic!("{}", err),
            reqwest_middleware::Error::Reqwest(err) => err,
        })?
        .json()
        .await?;

    if !response.success {
        // TODO: Explicitly report that game server rejected attempt
        // Could be useful for custom join filters
        return Err(AuthenticateError::WrongResponse);
    }

    // Store the server as current
    accounts
        .join_server(param.id, &param.server)
        .await
        .expect("Unable to update current server");

    Ok(AuthenticateResponse {
        ip: server.ip(),
        port: server.game_port(),
        auth_token: truncated,
    })
}
