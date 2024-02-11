use axum::routing::post;
use axum::{
    routing::get,
    Router,
};

use rustysearch::search::engine::SearchEngine;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // initialize our search engine
    let mut search_engine = SearchEngine::new(1.5, 0.75);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(rustysearch::handler::hello::say_hello))
        .route("/search/add", post(rustysearch::handler::search::index_new_document));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

