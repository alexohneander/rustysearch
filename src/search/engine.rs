use bincode::{deserialize_from, serialize_into};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::f64;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
struct SavedIndex {
    index_btree_map: BTreeMap<String, HashMap<String, i32>>,
    documents_btree_map: BTreeMap<String, String>,
}

fn update_url_scores(old: &mut HashMap<String, f64>, new: &HashMap<String, f64>) {
    for (url, score) in new {
        old.entry(url.to_string())
            .and_modify(|e| *e += score)
            .or_insert(*score);
    }
}

fn normalize_string(input_string: &str) -> String {
    let string_without_punc: String = input_string
        .chars()
        .filter(|&c| !c.is_ascii_punctuation())
        .collect();
    let string_without_double_spaces: String = string_without_punc
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    string_without_double_spaces.to_lowercase()
}

/// SearchEngine represents a search engine that indexes and searches documents based on the BM25 ranking algorithm.
///
/// The search engine maintains an index of words and their frequencies in each document, as well as the actual document content.
/// It provides methods to index documents, perform searches, and calculate relevance scores using the BM25 algorithm.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use rustysearch::search::engine::SearchEngine;
///
/// // Create a new search engine with k1 = 1.2 and b = 0.75
/// let mut engine = SearchEngine::new(1.2, 0.75);
///
/// // Index a document
/// engine.index("https://example.com/doc1", "This is the content of document 1");
///
/// // Perform a search
/// let results = engine.search("content");
///
/// // Print the search results
/// for (url, score) in results {
///     println!("{} - Relevance Score: {}", url, score);
/// }
/// ```
#[derive(Default, Debug, Clone)]
pub struct SearchEngine {
    index: BTreeMap<String, HashMap<String, i32>>,
    documents: BTreeMap<String, String>,
    k1: f64,
    b: f64,
}

impl SearchEngine {
    /// Creates a new instance of SearchEngine with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `k1` - The k1 parameter of the BM25 algorithm.
    /// * `b` - The b parameter of the BM25 algorithm.
    ///
    /// **Returns**
    ///
    /// A new instance of SearchEngine.
    pub fn new(k1: f64, b: f64) -> SearchEngine {
        // try to get Index from disk
        let mut index_btreemap = BTreeMap::new();
        let mut documents_btreemap = BTreeMap::new();
        let saved_index = get_index_from_disk();

        if !saved_index.index_btree_map.is_empty() && !saved_index.documents_btree_map.is_empty() {
            index_btreemap = saved_index.index_btree_map;
            documents_btreemap = saved_index.documents_btree_map;
        }

        SearchEngine {
            index: index_btreemap,
            documents: documents_btreemap,
            k1,
            b,
        }
    }

    /// Returns a vector of all the document URLs in the search engine's index.
    ///
    /// **Returns**
    ///
    /// A vector of document URLs.
    pub fn posts(&self) -> Vec<String> {
        self.documents.keys().cloned().collect()
    }

    /// Returns the number of documents in the search engine's index.
    ///
    /// **Returns**
    ///
    /// The number of documents.
    pub fn number_of_documents(&self) -> usize {
        self.documents.len()
    }

    /// Returns the average document length in terms of number of words.
    ///
    /// **Returns**
    ///
    /// The average document length.
    pub fn avdl(&self) -> f64 {
        let total_length: usize = self.documents.values().map(|d| d.len()).sum();
        total_length as f64 / self.documents.len() as f64
    }

    /// Calculates the inverse document frequency (IDF) score for a given keyword.
    ///
    /// **Arguments**
    ///
    /// * `kw` - The keyword for which to calculate the IDF score.
    ///
    /// **Returns**
    ///
    /// The IDF score.
    pub fn idf(&self, kw: &str) -> f64 {
        let n = self.number_of_documents() as f64;
        let n_kw = self.get_urls(kw).len() as f64;
        ((n - n_kw + 0.5) / (n_kw + 0.5) + 1.0).ln()
    }

    /// Calculates the BM25 relevance scores for a given keyword.
    ///
    /// **Arguments**
    ///
    /// * `kw` - The keyword for which to calculate the relevance scores.
    ///
    /// **Returns**
    ///
    /// A HashMap containing the document URLs as keys and their relevance scores as values.
    pub fn bm25(&self, kw: &str) -> HashMap<String, f64> {
        let mut result = HashMap::new();
        let idf_score = self.idf(kw);
        let avdl = self.avdl();
        for (url, freq) in self.get_urls(kw) {
            let numerator = freq as f64 * (self.k1 + 1.0);
            let denominator = freq as f64
                + self.k1
                    * (1.0 - self.b
                        + self.b * self.documents.get(&url).unwrap().len() as f64 / avdl);
            result.insert(url.to_string(), idf_score * numerator / denominator);
        }
        result
    }

    /// Performs a search for the given query and returns the relevance scores for the matching documents.
    ///
    /// **Arguments**
    ///
    /// * `query` - The search query.
    ///
    /// **Returns**
    ///
    /// A HashMap containing the document URLs as keys and their relevance scores as values.
    pub fn search(&mut self, query: &str) -> HashMap<String, f64> {
        let keywords = normalize_string(query)
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut url_scores: HashMap<String, f64> = HashMap::new();
        for kw in keywords {
            let kw_urls_score = self.bm25(&kw);
            update_url_scores(&mut url_scores, &kw_urls_score);
        }
        url_scores
    }

    /// Indexes a document with the given URL and content.
    ///
    /// **Arguments**
    ///
    /// * `url` - The URL of the document.
    /// * `content` - The content of the document.
    pub fn index(&mut self, url: &str, content: &str) {
        self.documents.insert(url.to_string(), content.to_string());
        let words = normalize_string(content)
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for word in words {
            *self
                .index
                .entry(word)
                .or_default()
                .entry(url.to_string())
                .or_insert(0) += 1;
        }

        // TODO: After updating the index
        self.write_index_to_disk();
    }

    /// Bulk indexes multiple documents.
    ///
    /// **Arguments**
    ///
    /// * `documents` - A vector of tuples containing the URL and content of each document.
    pub fn bulk_index(&mut self, documents: Vec<(&str, &str)>) {
        for (url, content) in documents {
            self.index(url, content);
        }
    }

    /// Returns the URLs and frequencies of a given keyword in the search engine's index.
    ///
    /// **Arguments**
    ///
    /// * `keyword` - The keyword to search for.
    ///
    /// **Returns**
    ///
    /// A HashMap containing the document URLs as keys and their frequencies as values.
    pub fn get_urls(&self, keyword: &str) -> HashMap<String, i32> {
        let keyword = normalize_string(keyword);
        self.index.get(&keyword).cloned().unwrap_or(HashMap::new())
    }

    // Write the current index to disk as binary to the configured location
    fn write_index_to_disk(&self) {
        let index_hash_map = self.index.clone();
        let documents_hash_map = self.documents.clone();

        let btree_index: BTreeMap<_, _> = index_hash_map.into_iter().collect();
        let btree_documents: BTreeMap<_, _> = documents_hash_map.into_iter().collect();

        let data = SavedIndex {
            index_btree_map: btree_index,
            documents_btree_map: btree_documents,
        };

        let mut file = BufWriter::new(File::create("/tmp/search.db").unwrap());
        serialize_into(&mut file, &data).unwrap();
        log::debug!("Wrote Index as BTreeMap to Disk");
    }

    /// Prints the current state of the search engine's index and document collection for debugging purposes.
    pub fn debug_index(&self) {
        log::debug!("Index: {:?}", self.index);
        log::debug!("Documents: {:?}", self.documents);
    }
}

fn get_index_from_disk() -> SavedIndex {
    let file = File::open("/tmp/search.db");
    let mut data = SavedIndex {
        documents_btree_map: BTreeMap::new(),
        index_btree_map: BTreeMap::new(),
    };

    if file.is_ok() {
        let reader = BufReader::new(file.unwrap());
        data = deserialize_from(reader).unwrap();
        log::debug!("Got BTreeMap Index from Disk");
    }

    data
}
