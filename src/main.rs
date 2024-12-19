use std::sync::Mutex;

use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use env_logger::Env;

use rustysearch::{
    cmd::arguments::Args,
    handlers::{hello, search},
    search::engine::SearchEngine,
    types::app_state::AppStateWithSearchEngine,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get Command Line Arguments
    let args = Args::parse();

    // Initialize logger
    if !args.log_level.is_empty() {
        env_logger::init_from_env(Env::default().default_filter_or(args.log_level));
    } else {
        env_logger::init_from_env(Env::default().default_filter_or("info"));
    }

    // Initialize the search engine
    let search_engine = SearchEngine::new(1.5, 0.75);

    // Wrap the search engine in a Mutex and then in an AppState
    let app_state = web::Data::new(AppStateWithSearchEngine {
        search_engine: Mutex::new(search_engine.clone()),
    });

    HttpServer::new(move || {
        App::new()
            // Inject the search engine into the application state
            .app_data(app_state.clone())
            // enable logger
            .wrap(Logger::default())
            // Hello Routes
            .service(hello::say_hello)
            // Search Routes
            .route(
                "/search/index/document",
                web::post().to(search::add_document_to_index),
            )
            .route(
                "/search/index/number_of_documents",
                web::get().to(search::get_number_of_documents),
            )
            .route("/search", web::get().to(search::search))
            .route("/search/debug", web::get().to(search::debug_index))
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
