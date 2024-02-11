#[cfg(test)]
mod tests {
    use rustysearch::search::engine::SearchEngine;

    #[test]
    fn test_search_engine() {
        let mut search_engine = SearchEngine::new(1.2, 0.75);

        search_engine.index("https://www.rust-lang.org/", "Rust Programming Language");
        let result = search_engine.search("Rust");

        assert_eq!(result.len(), 1);
        assert_eq!(search_engine.posts().len(), 1);
        assert_eq!(search_engine.number_of_documents(), 1);
    }
}