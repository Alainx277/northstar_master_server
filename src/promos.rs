use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("client" / "mainmenupromos")
        .and(warp::get())
        .then(main_menu_promos)
}

async fn main_menu_promos() -> Box<dyn warp::Reply> {
    match std::fs::read("mainmenupromodata.json") {
        Ok(data) => Box::new(warp::reply::with_header(
            data,
            warp::http::header::CONTENT_TYPE,
            "application/json",
        )),
        Err(err) => {
            tracing::error!("Failed reading main menu promo data: {}", err);
            Box::new(warp::reply::with_status(
                warp::reply(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
