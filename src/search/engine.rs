use std::collections::HashMap;
use std::f64;

pub fn update_url_scores(old: &mut HashMap<String, f64>, new: &HashMap<String, f64>) {
    for (url, score) in new {
        old.entry(url.to_string()).and_modify(|e| *e += score).or_insert(*score);
    }
}

pub fn normalize_string(input_string: &str) -> String {
    let string_without_punc: String = input_string.chars().filter(|&c| !c.is_ascii_punctuation()).collect();
    let string_without_double_spaces: String = string_without_punc.split_whitespace().collect::<Vec<&str>>().join(" ");
    string_without_double_spaces.to_lowercase()
}

pub struct SearchEngine {
    index: HashMap<String, HashMap<String, i32>>,
    documents: HashMap<String, String>,
    k1: f64,
    b: f64,
}

impl SearchEngine {
    pub fn new(k1: f64, b: f64) -> SearchEngine {
        SearchEngine {
            index: HashMap::new(),
            documents: HashMap::new(),
            k1,
            b,
        }
    }

    pub fn posts(&self) -> Vec<String> {
        self.documents.keys().cloned().collect()
    }

    pub fn number_of_documents(&self) -> usize {
        self.documents.len()
    }

    pub fn avdl(&self) -> f64 {
        let total_length: usize = self.documents.values().map(|d| d.len()).sum();
        total_length as f64 / self.documents.len() as f64
    }

    pub fn idf(&self, kw: &str) -> f64 {
        let n = self.number_of_documents() as f64;
        let n_kw = self.get_urls(kw).len() as f64;
        ((n - n_kw + 0.5) / (n_kw + 0.5) + 1.0).ln()
    }

    pub fn bm25(&self, kw: &str) -> HashMap<String, f64> {
        let mut result = HashMap::new();
        let idf_score = self.idf(kw);
        let avdl = self.avdl();
        for (url, freq) in self.get_urls(kw) {
            let numerator = freq as f64 * (self.k1 + 1.0);
            let denominator = freq as f64 + self.k1 * (1.0 - self.b + self.b * self.documents.get(&url).unwrap().len() as f64 / avdl);
            result.insert(url.to_string(), idf_score * numerator / denominator);
        }
        result
    }

    pub fn search(&mut self, query: &str) -> HashMap<String, f64> {
        let keywords = normalize_string(query).split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        let mut url_scores: HashMap<String, f64> = HashMap::new();
        for kw in keywords {
            let kw_urls_score = self.bm25(&kw);
            update_url_scores(&mut url_scores, &kw_urls_score);
        }
        url_scores
    }

    pub fn index(&mut self, url: &str, content: &str) {
        self.documents.insert(url.to_string(), content.to_string());
        let words = normalize_string(content).split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        for word in words {
            *self.index.entry(word).or_insert(HashMap::new()).entry(url.to_string()).or_insert(0) += 1;
        }
    }

    pub fn bulk_index(&mut self, documents: Vec<(&str, &str)>) {
        for (url, content) in documents {
            self.index(url, content);
        }
    }

    pub fn get_urls(&self, keyword: &str) -> HashMap<String, i32> {
        let keyword = normalize_string(keyword);
        self.index.get(&keyword).cloned().unwrap_or(HashMap::new())
    }
}