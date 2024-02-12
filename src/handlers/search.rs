use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::types::app_state::AppStateWithSearchEngine;

#[derive(Deserialize)]
pub struct AddDocumentRequest {
    url: String,
    content: String,
}

#[derive(Deserialize)]
pub struct QueryRequest {
    query: String,
}


pub async fn add_document_to_index(data: web::Data<AppStateWithSearchEngine>, req: web::Json<AddDocumentRequest>) -> impl Responder {
    data.search_engine.lock().unwrap().index(&req.url, &req.content);
    HttpResponse::Created().body("Document added to index!")
}

pub async fn get_number_of_documents(data: web::Data<AppStateWithSearchEngine>) -> impl Responder {
    let number_of_documents = data.search_engine.lock().unwrap().number_of_documents();
    HttpResponse::Ok().body(format!("Number of documents: {}", number_of_documents))
}

pub async fn search(data: web::Data<AppStateWithSearchEngine>, req: web::Query<QueryRequest>) -> impl Responder {
    if req.query.is_empty() {
        return HttpResponse::BadRequest().body("Query is empty");
    }

    // Get the query string from query parameters
    log::debug!("Searching for: {}", &req.query);

    let results = data.search_engine.lock().unwrap().search(&req.query);
    HttpResponse::Ok().json(results)
}

pub async fn debug_index(data: web::Data<AppStateWithSearchEngine>) -> impl Responder {
    data.search_engine.lock().unwrap().debug_index();
    HttpResponse::Ok().json("Index debugged!")
}