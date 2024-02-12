use std::sync::Mutex;

use crate::search::engine::SearchEngine;

pub struct AppStateWithSearchEngine {
    pub search_engine: Mutex<SearchEngine>, // <- Mutex is necessary to mutate safely across threads
}