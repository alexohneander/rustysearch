use axum::
    Json
;
use serde::Deserialize;

use crate::search::engine::SearchEngine;

pub fn index_new_document(mut engine: SearchEngine, Json(payload): Json<IndexNewDocument>) {
   engine.index(&payload.url, &payload.content);
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct IndexNewDocument {
    url: String,
    content: String,
}