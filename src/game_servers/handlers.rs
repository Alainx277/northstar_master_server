use std::{
    borrow::Cow,
    net::SocketAddr,
    time::{Duration, Instant},
};

use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use warp::multipart::FormData;

use crate::{
    api::{api_response, ApiErrorKind},
    id::UniqueId,
    SharedServerList,
};

use super::{
    verify::VerifyServerError, AddServerError, ModInfo, Server, ServerSettings, MAX_PLAYERS_LIMIT,
};

#[derive(Error, Debug)]
pub(super) enum CreateServerError {
    #[error("server could not be verified: {0}")]
    Verification(#[from] VerifyServerError),
    #[error("mod info is invalid")]
    InvalidModInfo,
    #[error("too many servers already registered on this host")]
    MaximumServersForHost,
}

impl ApiErrorKind for CreateServerError {
    fn kind(&self) -> &'static str {
        match self {
            CreateServerError::Verification(e) => e.kind(),
            CreateServerError::InvalidModInfo => "INVALID_MOD_INFO",
            CreateServerError::MaximumServersForHost => "MAX_SERVERS_FOR_IP",
        }
    }
}

impl From<AddServerError> for CreateServerError {
    fn from(err: AddServerError) -> Self {
        match err {
            AddServerError::MaximumServersForHost => CreateServerError::MaximumServersForHost,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateServerResponse {
    id: String,
    server_auth_token: String,
}

pub(super) async fn create_server_entry(
    settings: ServerSettings,
    remote: Option<SocketAddr>,
    servers: SharedServerList,
    form: FormData,
) -> Result<CreateServerResponse, CreateServerError> {
    let ip = remote
        .ok_or(CreateServerError::Verification(VerifyServerError::Unknown))?
        .ip();

    super::verify::verify_server(ip, settings.auth_port).await?;

    let mod_info = ModInfo::from_form(form)
        .await
        .map_err(|_| CreateServerError::InvalidModInfo)?;
    let server = Server::new(ip, settings, mod_info);
    let response = CreateServerResponse {
        id: server.id.to_string(),
        server_auth_token: server.auth_token.to_string(),
    };

    tracing::debug!(
        id = response.id.as_str(),
        token = response.server_auth_token.as_str(),
        ip = ?server.ip,
        settings = ?server.settings,
        "created server entry"
    );

    {
        let mut servers = servers.write().await;
        servers.push(server)?;
    }

    Ok(response)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerListEntry<'a> {
    id: &'a UniqueId,
    name: &'a str,
    description: &'a str,
    map: &'a str,
    playlist: &'a str,
    max_players: u32,
    has_password: bool,
    player_count: u32,
    mod_info: Cow<'a, ModInfo>,
}

impl<'a> From<&'a Server> for ServerListEntry<'a> {
    fn from(server: &'a Server) -> Self {
        // Prevent reporting huge fake capacity
        let max_players = server.settings.max_players.min(MAX_PLAYERS_LIMIT);

        Self {
            id: &server.id,
            name: &server.settings.name,
            description: &server.settings.description,
            map: &server.settings.map,
            playlist: &server.settings.playlist,
            max_players,
            has_password: server.settings.password.is_some(),
            player_count: server.player_count.map(|c| c.min(max_players)).unwrap_or(0),
            mod_info: server
                .mod_info
                .as_ref()
                .map(Cow::Borrowed)
                .unwrap_or_default(),
        }
    }
}

// TODO: Cache this
pub(super) async fn list_servers(servers: SharedServerList) -> impl warp::Reply {
    let mut servers = servers.write().await;
    servers.remove_inactive();
    // Create server entries for those we have seen in the last minute
    let list: Vec<ServerListEntry> = servers
        .iter()
        .filter(|s| s.last_seen_age() < Duration::from_secs(60))
        .map(|s| s.into())
        .collect();
    warp::reply::json(&list)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UpdateServerParam {
    id: UniqueId,
    port: Option<u16>,
    auth_port: Option<u16>,
    name: Option<String>,
    description: Option<String>,
    map: Option<String>,
    playlist: Option<String>,
    max_players: Option<u32>,
    password: Option<Option<String>>,
    player_count: Option<u32>,
}

impl UpdateServerParam {
    fn apply(self, server: &mut Server) {
        if let Some(name) = self.name {
            server.settings.name = name;
        }

        if let Some(description) = self.description {
            server.settings.description = description;
        }

        if let Some(map) = self.map {
            server.settings.map = map;
        }

        if let Some(playlist) = self.playlist {
            server.settings.playlist = playlist;
        }

        if let Some(max_players) = self.max_players {
            server.settings.max_players = max_players;
        }

        if let Some(mut password) = self.password {
            // Remove password if empty string passed
            if password.as_ref().map_or(false, |p| p.is_empty()) {
                password = None;
            }
            server.settings.password = password;
        }

        if let Some(player_count) = self.player_count {
            server.player_count = Some(player_count);
        }
    }
}

impl TryFrom<UpdateServerParam> for ServerSettings {
    type Error = ();

    fn try_from(value: UpdateServerParam) -> Result<Self, Self::Error> {
        Ok(ServerSettings {
            port: value.port.ok_or(())?,
            auth_port: value.auth_port.ok_or(())?,
            name: value.name.ok_or(())?,
            description: value.description.ok_or(())?,
            map: value.map.ok_or(())?,
            playlist: value.playlist.ok_or(())?,
            max_players: value.max_players.ok_or(())?,
            password: value.password.ok_or(())?,
        })
    }
}

pub(super) async fn update_server(
    param: UpdateServerParam,
    remote: Option<SocketAddr>,
    server_list: SharedServerList,
    form: FormData,
) -> Box<dyn warp::Reply> {
    let ip = match remote {
        Some(addr) => addr.ip(),
        None => return Box::new(warp::reply()),
    };

    let exists = {
        let servers = server_list.read().await;
        servers.servers.contains_key(&param.id)
    };

    // Create server entry if none exists
    if !exists {
        // The request must contain all the necessary data
        if let Ok(settings) = param.try_into() {
            return Box::new(api_response(
                create_server_entry(settings, remote, server_list, form).await,
            ));
        } else {
            return Box::new(warp::reply());
        }
    }

    let mut servers = server_list.write().await;
    let server = match servers.servers.get_mut(&param.id) {
        Some(s) => s,
        None => return Box::new(warp::reply()),
    };

    // Only allow update from server address
    if server.ip != ip {
        return Box::new(warp::reply());
    }

    server.last_seen = Instant::now();
    param.apply(server);

    Box::new(warp::reply())
}

#[derive(Deserialize)]
pub(super) struct RemoveServerParam {
    id: UniqueId,
}

pub(super) async fn remove_server(
    param: RemoveServerParam,
    remote: Option<SocketAddr>,
    servers: SharedServerList,
) -> impl warp::Reply {
    let ip = match remote {
        Some(addr) => addr.ip(),
        None => return warp::reply(),
    };

    let mut servers = servers.write().await;
    // Remove server if IP matches
    if servers.get(&param.id).map(|s| s.ip == ip) == Some(true) {
        servers.remove(&param.id);
    }

    warp::reply()
}
