use std::{fs, path::Path};

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
    fn setup(&self) {
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
    pub fn read_stats(&self) -> String {
        if !Path::new(&self.stats_path).exists() {
            return String::from("{\"version\": \"0.1.0\", \"docs\": 0}");
        }

        return String::from("");
    }
}

fn main() {
    println!("Hello, world!")
}
