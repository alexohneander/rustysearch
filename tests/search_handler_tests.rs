#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use actix_web::{test, web, App};
    use rustysearch::{handlers::search, search::engine::SearchEngine, types::app_state::AppStateWithSearchEngine};

    #[actix_web::test]
    async fn test_add_document_to_index() {
        let search_engine = SearchEngine::new(1.5, 0.75);

        let app_state = web::Data::new(AppStateWithSearchEngine {
            search_engine: Mutex::new(search_engine.clone()),
        });

        let app = test::init_service(App::new()
            .app_data(app_state.clone())
            .route(
                "/search/index/document",
                web::post().to(search::add_document_to_index),
            )
        ).await;

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
    }
}