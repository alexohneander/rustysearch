#[cfg(test)]
mod tests {
    use rustysearch::search::engine::{remove_index_from_disk, SearchEngine};

    #[test]
    fn test_search_engine() {
        let mut search_engine = SearchEngine::new(1.5, 0.75);

        search_engine.index("https://www.rust-lang.org/", "Rust Programming Language");
        let result = search_engine.search("Rust");

        assert_eq!(result.len(), 1);
        assert_eq!(search_engine.posts().len(), 1);
        assert_eq!(search_engine.number_of_documents(), 1);

        remove_index_from_disk();
    }

    #[test]
    fn test_bulk_index() {
        let mut search_engine = SearchEngine::new(1.5, 0.75);

        search_engine.bulk_index(vec![
            ("https://www.rust-lang.org/", "Rust Programming Language"),
            ("https://www.wikipedia.com/", "Rust Programming Language"),
        ]);

        assert_eq!(search_engine.posts().len(), 2);
        assert_eq!(search_engine.number_of_documents(), 2);

        remove_index_from_disk();
    }
}

