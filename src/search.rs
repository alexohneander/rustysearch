use std::{fs, path::Path};

use crate::{types::Stats, analyze::tokenizer::Tokenizer};

pub struct Rustysearch {
    base_directory: String,
    index_path: String,
    docs_path: String,
    stats_path: String,
}

impl Rustysearch {
    /// **Sets up the object & the data directory**
    ///
    /// Requires a ``base_directory`` parameter, which specifies the parent
    /// directory the index/document/stats data will be kept in.
    ///
    pub fn new(path: &str) -> Self {
        Self {
            base_directory: path.to_string(),
            index_path: format!("{}/index", path),
            docs_path: format!("{}/docs", path),
            stats_path: format!("{}/stats.json", path),
        }
    }

    /// **Handles the creation of the various data directories**
    ///
    /// If the paths do not exist, it will create them. As a side effect, you
    /// must have read/write access to the location you're trying to create
    /// the data at.
    ///
    pub fn setup(&self) {
        // Create the base directory
        if !Path::new(&self.base_directory).exists() {
            fs::create_dir(&self.base_directory).expect("Unable to create base directory");
        }
        // Create the index directory
        if !Path::new(&self.index_path).exists() {
            fs::create_dir(&self.index_path).expect("Unable to create index directory");
        }
        // Create the docs directory
        if !Path::new(&self.docs_path).exists() {
            fs::create_dir(&self.docs_path).expect("Unable to create docs directory");
        }
    }

    /// **Reads the index-wide stats**
    ///
    /// If the stats do not exist, it makes returns data with the current
    /// version of ``rustysearch`` & zero docs (used in scoring).
    ///
    pub fn read_stats(&self) -> std::io::Result<Stats> {
        let stats: Stats;

        if !Path::new(&self.stats_path).exists() {
            stats = Stats {
                version: String::from("0.1.0"),
                total_docs: 0,
            };
        } else {
            // Read the stats file
            let stats_json = fs::read_to_string(&self.stats_path).expect("Unable to read stats");
            stats = serde_json::from_str(&stats_json).unwrap();
        }

        Ok(stats)
    }

    /// **Writes the index-wide stats**
    ///
    /// Takes a ``new_stats`` parameter, which should be a dictionary of
    /// stat data. Example stat data::
    ///
    ///    {
    ///        'version': '1.0.0',
    ///        'total_docs': 25,
    ///    }
    /// 
    pub fn write_stats(&self, new_stats: Stats) -> std::io::Result<()> {
        // Write new_stats as json to stats_path
        let new_stats_json = serde_json::to_string(&new_stats).unwrap();
        fs::write(&self.stats_path, new_stats_json)?;
        Ok(())
    }

    /// **Increments the total number of documents the index is aware of**
    ///
    /// This is important for scoring reasons & is typically called as part
    /// of the indexing process.
    /// 
    pub fn increment_total_docs(&self) {
        let mut current_stats = self.read_stats().unwrap();
        current_stats.total_docs += 1;
        self.write_stats(current_stats).unwrap();
    }

    /// Returns the total number of documents the index is aware of
    /// 
    pub fn get_total_docs(&self) -> i32 {
        let stats = self.read_stats().unwrap();
        return stats.total_docs;
    }

    /// Given a string (``blob``) of text, this will return a Vector of tokens.
    /// 
    pub fn make_tokens(&self, blob: &str) -> Vec<String> {
        let tokenizer = Tokenizer::new(blob, vec![], None);
        let tokens = tokenizer.split_into_words();
        return tokens;
    }
}
