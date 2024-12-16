use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SavedIndex {
    pub index_btree_map: BTreeMap<String, HashMap<String, i32>>,
    pub documents_btree_map: BTreeMap<String, String>,
}
