use warp::Filter;

use crate::{api::api_response, game_servers::with_servers, Database, SharedServerList};

use super::AccountRepository;

pub fn routes(
    database: Database,
    servers: SharedServerList,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let base = warp::path("accounts");
    base.and(write_persistence(database, servers))
}

pub(super) fn write_persistence(
    database: Database,
    servers: SharedServerList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("write_persistence")
        .and(warp::post())
        .and(warp::query::<super::handlers::WritePersistenceParam>())
        .and(warp::multipart::form())
        .and(warp::addr::remote())
        .and(with_accounts(database))
        .and(with_servers(servers))
        .then(super::handlers::write_persistence)
        .map(api_response)
}

pub fn with_accounts(
    database: Database,
) -> impl Filter<Extract = (AccountRepository,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || AccountRepository::new(database.clone()))
}
