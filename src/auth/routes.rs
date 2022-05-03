use warp::Filter;

use crate::{
    accounts::with_accounts, api::api_response, game_servers::with_servers, Database,
    SharedServerList,
};

pub fn routes(
    database: Database,
    servers: SharedServerList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let base = warp::path("client");
    base.and(origin_authentication(database.clone()))
        .or(base.and(authenticate_self(database.clone())))
        .or(base.and(authenticate(database, servers)))
}

pub(super) fn origin_authentication(
    database: Database,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("origin_auth")
        .and(warp::get())
        .and(warp::query::<super::handlers::OriginAuthenticationParam>())
        .and(warp::addr::remote())
        .and(with_accounts(database))
        .then(super::handlers::origin_authentication)
        .map(api_response)
}

pub(super) fn authenticate_self(
    database: Database,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("auth_with_self")
        .and(warp::post())
        .and(warp::query::<super::handlers::AuthenticateSelfParam>())
        .and(with_accounts(database))
        .then(super::handlers::authenticate_self)
        .map(api_response)
}

pub(super) fn authenticate(
    database: Database,
    servers: SharedServerList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("auth_with_server")
        .and(warp::post())
        .and(warp::query::<super::handlers::AuthenticateParam>())
        .and(with_accounts(database))
        .and(with_servers(servers))
        .then(super::handlers::authenticate)
        .map(api_response)
}
