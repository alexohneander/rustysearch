#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use actix_web::{test, web, App};
    use rustysearch::{
        handlers::search,
        search::engine::{remove_index_from_disk, SearchEngine},
        types::app_state::AppStateWithSearchEngine,
    };

    #[actix_web::test]
    async fn test_add_document_to_index() {
        let search_engine = SearchEngine::new(1.5, 0.75);

        let app_state = web::Data::new(AppStateWithSearchEngine {
            search_engine: Mutex::new(search_engine.clone()),
        });

        let app = test::init_service(App::new().app_data(app_state.clone()).route(
            "/search/index/document",
            web::post().to(search::add_document_to_index),
        ))
        .await;

        let data = search::AddDocumentRequest {
            url: "https://example.com".to_string(),
            content: "This is an example document".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/search/index/document")
            .set_json(data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == 201);

        remove_index_from_disk();
    }

    #[actix_web::test]
    async fn test_get_number_of_documents() {
        let mut search_engine = SearchEngine::new(1.5, 0.75);
        search_engine.index("https://example.com", "This is an example document");

        let app_state = web::Data::new(AppStateWithSearchEngine {
            search_engine: Mutex::new(search_engine.clone()),
        });

        let app = test::init_service(App::new().app_data(app_state.clone()).route(
            "/search/index/number_of_documents",
            web::get().to(search::get_number_of_documents),
        ))
        .await;

        let req = test::TestRequest::get()
            .uri("/search/index/number_of_documents")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == 200);

        remove_index_from_disk();
    }

    #[actix_web::test]
    async fn test_search() {
        let mut search_engine = SearchEngine::new(1.5, 0.75);
        search_engine.index("https://example.com", "This is an example document");

        let app_state = web::Data::new(AppStateWithSearchEngine {
            search_engine: Mutex::new(search_engine.clone()),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/search", web::get().to(search::search)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/search?query=example")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == 200);

        remove_index_from_disk();
    }

    #[actix_web::test]
    async fn test_search_without_query() {
        let mut search_engine = SearchEngine::new(1.5, 0.75);
        search_engine.index("https://example.com", "This is an example document");

        let app_state = web::Data::new(AppStateWithSearchEngine {
            search_engine: Mutex::new(search_engine.clone()),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/search", web::get().to(search::search)),
        )
        .await;

        let req = test::TestRequest::get().uri("/search?query=").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == 400);

        remove_index_from_disk();
    }

    #[actix_web::test]
    async fn test_debug_index() {
        let search_engine = SearchEngine::new(1.5, 0.75);

        let app_state = web::Data::new(AppStateWithSearchEngine {
            search_engine: Mutex::new(search_engine.clone()),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/search/debug", web::get().to(search::debug_index)),
        )
        .await;

        let req = test::TestRequest::get().uri("/search/debug").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == 200);

        remove_index_from_disk();
    }
}

