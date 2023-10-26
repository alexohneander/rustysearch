use std::{cmp::min, collections::HashMap, fs, path::Path};

use serde_json::{Value, json};

use crate::{analyze::tokenizer::Tokenizer, types::Stats};

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

    /// **Converts a iterable of ``tokens`` into n-grams**
    ///
    /// This assumes front grams (all grams made starting from the left side
    /// of the token).
    ///
    /// Optionally accepts a ``min_gram`` parameter, which takes an integer &
    /// controls the minimum gram length. Default is ``3``.
    ///
    /// Optionally accepts a ``max_gram`` parameter, which takes an integer &
    /// controls the maximum gram length. Default is ``6``.
    ///
    pub fn make_ngrams(
        &self,
        tokens: Vec<String>,
        min_gram: usize,
        max_gram: usize,
    ) -> HashMap<String, Vec<usize>> {
        let mut terms: HashMap<String, Vec<usize>> = HashMap::new();

        for (position, token) in tokens.iter().enumerate() {
            for window_length in min_gram..min(max_gram + 1, token.len() + 1) {
                // Assuming "front" grams.
                let gram = &token[..window_length];
                terms
                    .entry(gram.to_string())
                    .or_insert(Vec::new())
                    .push(position);
            }
        }

        return terms;
    }

    /// Given a ``term``, hashes it & returns a string of the first N letters
    ///
    /// Optionally accepts a ``length`` parameter, which takes an integer &
    /// controls how much of the hash is returned. Default is ``6``.
    ///
    /// This is usefully when writing files to the file system, as it helps
    /// us keep from putting too many files in a given directory (~32K max
    /// with the default).
    ///
    pub fn hash_name(&self, term: &str, length: usize) -> String {
        // Make sure it's ASCII.
        let term = term.to_ascii_lowercase();

        // We hash & slice the term to get a small-ish number of fields
        // and good distribution between them.
        let hash = md5::compute(&term);
        let hashed = format!("{:x}", hash);

        // Cut string after length characters
        let hashed = &hashed[..length];

        return hashed.to_string();
    }

    /// Given a ``term``, creates a segment filename based on the hash of the term.
    ///
    /// Returns the full path to the segment.
    ///
    pub fn make_segment_name(&self, term: &str) -> String {
        let term = &self.hash_name(term, 6);

        let index_file_name = format!("{}.index", term);
        let segment_path = Path::new(&self.index_path).join(index_file_name);
        let segment_path = segment_path.to_str().unwrap().to_string();

        fs::write(&segment_path, "").expect("Unable to create segment file");

        return segment_path;
    }

    /// Given a ``line`` from the segment file, this returns the term & its info.
    ///
    /// The term info is stored as serialized JSON. The default separator
    /// between the term & info is the ``\t`` character, which would never
    /// appear in a term due to the way tokenization is done.
    ///
    pub fn parse_record(&self, line: &str) -> (String, String) {
        let mut parts = line.trim().split("\t");
        let term = parts.next().unwrap().to_string();
        let info = parts.next().unwrap().to_string();
        (term, info)
    }

    ///  Given a ``term`` and a dict of ``term_info``, creates a line for
    /// writing to the segment file.
    /// 
    pub fn make_record(&self, term: &str, term_info: &Value) -> String {
        format!("{}\t{}\n", term, json!(term_info).to_string())
    }
}
