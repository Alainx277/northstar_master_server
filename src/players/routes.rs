use warp::Filter;

use crate::{Database, accounts::with_accounts, api::api_response};


pub fn routes(
    database: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let base = warp::path("player");
    base.and(player_info(database))
}

pub(super) fn player_info(
    database: Database,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("info")
        .and(warp::get())
        .and(warp::query::<super::handlers::PlayerInfoParam>())
        .and(with_accounts(database))
        .then(super::handlers::player_info)
        .map(api_response)
}
