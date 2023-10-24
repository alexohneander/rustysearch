use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats{
    pub version: String,
    pub total_docs: i32,
}