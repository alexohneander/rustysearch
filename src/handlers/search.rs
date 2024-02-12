use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::types::app_state::AppStateWithSearchEngine;

#[derive(Deserialize)]
pub struct AddDocumentRequest {
    url: String,
    content: String,
}

pub async fn add_document_to_index(data: web::Data<AppStateWithSearchEngine>, req: web::Json<AddDocumentRequest>) -> impl Responder {
    data.search_engine.lock().unwrap().index(&req.url, &req.content);
    HttpResponse::Created().body("Document added to index!")
}

pub async fn get_number_of_documents(data: web::Data<AppStateWithSearchEngine>) -> impl Responder {
    let number_of_documents = data.search_engine.lock().unwrap().number_of_documents();
    HttpResponse::Ok().body(format!("Number of documents: {}", number_of_documents))
}