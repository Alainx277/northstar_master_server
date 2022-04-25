use std::sync::Arc;

use tokio::sync::RwLock;
use warp::Filter;

mod accounts;
mod api;
mod auth;
mod game_servers;
mod id;
mod promos;
mod players;

type SharedServerList = Arc<RwLock<game_servers::ServerList>>;
type Database = sqlx::Pool<sqlx::Sqlite>;

#[tokio::main]
async fn main() {
    // Load database
    let database = sqlx::SqlitePool::connect(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL env var must be set"),
    )
    .await
    .expect("Failed opening database");

    // Set up logging
    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "northstar_master_server=info,warp=debug".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let servers: SharedServerList = Arc::new(RwLock::default());
    let routes = api::northstar_version()
        .and(
            game_servers::routes(servers.clone())
                .or(auth::routes(database.clone(), servers.clone()))
                .or(accounts::routes(database.clone(), servers))
                .or(promos::routes())
                .or(players::routes(database.clone()))
        )
        .with(warp::trace::request())
        .recover(api::version_error_handler);

    warp::serve(routes).run(([0, 0, 0, 0], 33998)).await;

    database.close().await;
}
