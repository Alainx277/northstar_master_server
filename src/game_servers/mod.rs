use bytes::buf::BufMut;
use futures_util::{StreamExt, TryStreamExt};
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    net::{IpAddr, SocketAddr},
    time::{Duration, Instant},
};
use warp::{multipart::FormData, Filter};

use crate::SharedServerList;

use crate::id::UniqueId;
pub use routes::{routes, with_servers};

mod handlers;
mod routes;
mod verify;

/// A server registered with the master server.
pub struct Server {
    id: UniqueId,
    ip: IpAddr,
    auth_token: UniqueId,
    settings: ServerSettings,
    last_seen: Instant,
    player_count: Option<u32>,
    mod_info: Option<ModInfo>,
}

impl Server {
    fn new(ip: IpAddr, settings: ServerSettings, mod_info: Option<ModInfo>) -> Self {
        let mut rng = rand::thread_rng();

        Server {
            id: UniqueId::new(&mut rng),
            ip,
            auth_token: UniqueId::new(&mut rng),
            settings,
            last_seen: Instant::now(),
            player_count: None,
            mod_info,
        }
    }

    #[must_use]
    pub fn ip(&self) -> IpAddr {
        self.ip
    }

    #[must_use]
    pub fn game_port(&self) -> u16 {
        self.settings.port
    }

    #[must_use]
    pub fn check_password(&self, password: Option<impl AsRef<str>>) -> bool {
        match self.settings.password.as_ref() {
            Some(p) => match password {
                Some(given) => p.as_str() == given.as_ref(),
                None => false,
            },
            None => true,
        }
    }

    #[must_use]
    pub fn auth_token(&self) -> UniqueId {
        self.auth_token
    }

    #[must_use]
    pub fn auth_address(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.settings.auth_port)
    }

    #[must_use]
    fn last_seen_age(&self) -> Duration {
        Instant::now().duration_since(self.last_seen)
    }
}

/// How many simultaneous servers can be registered for the same ip address.
const DEFAULT_MAX_SERVERS_PER_HOST: usize = 10;

enum AddServerError {
    MaximumServersForHost,
    ConflictingAuthPort,
}

/// Stores all listed servers.
#[derive(Default)]
pub struct ServerList {
    servers: HashMap<UniqueId, Server>,
    addresses: HashMap<IpAddr, HashSet<UniqueId>>,
}

impl ServerList {
    fn iter(&self) -> impl std::iter::Iterator<Item = &Server> {
        self.servers.iter().map(|(_, v)| v)
    }

    fn push(&mut self, server: Server) -> Result<&Server, AddServerError> {
        if let Some(host_servers) = self.addresses.get(&server.ip()) {
            // Limit number of servers on the same host
            let maximum_hosts = std::env::var("MAX_SERVERS_PER_HOST")
                .map(|v| v.parse::<usize>().unwrap())
                .unwrap_or(DEFAULT_MAX_SERVERS_PER_HOST);
            if host_servers.len() + 1 > maximum_hosts {
                return Err(AddServerError::MaximumServersForHost);
            }

            // Remove existing server on same game port
            if let Some(&existing_id) = host_servers
                .iter()
                .find(|&id| self.servers.get(id).unwrap().settings.port == server.settings.port)
            {
                self.remove(&existing_id);
            }
        }

        // Check for conflicting authentication port
        if self
            .addresses
            .get(&server.ip())
            .map(|servers| {
                servers.iter().any(|id| {
                    self.servers.get(id).unwrap().settings.auth_port == server.settings.auth_port
                })
            })
            .unwrap_or(false)
        {
            return Err(AddServerError::ConflictingAuthPort);
        }

        match self.servers.entry(server.id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                panic!("Conflicting server unique id, this should never happen.")
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                // Store ip address mapping
                self.addresses
                    .entry(server.ip())
                    .or_default()
                    .insert(server.id);
                //  Store server
                Ok(v.insert(server))
            }
        }
    }

    pub fn get(&self, k: &UniqueId) -> Option<&Server> {
        self.servers.get(k)
    }

    pub fn remove(&mut self, k: &UniqueId) {
        if let Some(server) = self.servers.remove(k) {
            let ip = &server.ip;
            let host_servers = self.addresses.get_mut(ip).unwrap();
            host_servers.remove(k);
            if host_servers.is_empty() {
                self.addresses.remove(ip);
            }
        }
    }

    /// Remove servers that haven't connected for some time
    pub fn remove_inactive(&mut self) {
        #[allow(clippy::needless_collect)]
        let inactive: Vec<_> = self
            .servers
            .iter()
            .filter(|(_, s)| s.last_seen_age() > Duration::from_secs(5 * 60))
            .map(|(_, s)| s.id)
            .collect();
        for id in inactive.into_iter() {
            self.remove(&id);
        }
    }
}

const MAX_PLAYERS_LIMIT: u32 = 32;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerSettings {
    port: u16,
    auth_port: u16,
    name: String,
    description: String,
    map: String,
    playlist: String,
    max_players: u32,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct Mod {
    required_on_client: bool,
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
struct ModInfo {
    mods: Vec<Mod>,
}

impl ModInfo {
    async fn from_form(form: FormData) -> Result<Option<Self>, ()> {
        let part = match form
            .try_filter(|p| core::future::ready(p.name() == "modinfo"))
            .next()
            .await
        {
            Some(r) => r,
            None => return Ok(None),
        }
        .map_err(|_| ())?;

        let data = part
            .stream()
            .try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            })
            .await
            .map_err(|_| ())?;

        serde_json::from_str(&match String::from_utf8(data) {
            Ok(s) => s,
            Err(_) => return Err(()),
        })
        .map(Some)
        .map_err(|_| ())
    }
}
