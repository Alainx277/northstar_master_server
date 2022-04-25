use crate::api::api_response;

use super::*;

pub fn routes(
    servers: SharedServerList,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let base = warp::path("server");
    base.and(routes::create_server_entry(servers.clone()))
        .or(base.and(routes::remove_server(servers.clone())))
        .or(base.and(routes::update_server(servers.clone())))
        .or(routes::list_servers(servers))
}

pub(super) fn create_server_entry(
    servers: SharedServerList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("add_server")
        .and(warp::post())
        .and(warp::query::<ServerSettings>())
        .and(warp::addr::remote())
        .and(with_servers(servers))
        .and(warp::multipart::form())
        .then(super::handlers::create_server_entry)
        .map(api_response)
}

pub(super) fn update_server(
    servers: SharedServerList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("update_values")
        .and(warp::post())
        .and(warp::query::<handlers::UpdateServerParam>())
        .and(warp::addr::remote())
        .and(with_servers(servers))
        .and(warp::multipart::form())
        .then(super::handlers::update_server)
}

pub(super) fn list_servers(
    servers: SharedServerList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("client" / "servers")
        .and(warp::get())
        .and(with_servers(servers))
        .then(super::handlers::list_servers)
}

pub(super) fn remove_server(
    servers: SharedServerList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("remove_server")
        .and(warp::delete())
        .and(warp::query::<handlers::RemoveServerParam>())
        .and(warp::addr::remote())
        .and(with_servers(servers))
        .then(super::handlers::remove_server)
}

pub fn with_servers(
    servers: SharedServerList,
) -> impl Filter<Extract = (SharedServerList,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || servers.clone())
}
