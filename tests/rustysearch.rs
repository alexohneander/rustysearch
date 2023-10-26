#[cfg(test)]
mod tests {
    use rustysearch::{search::Rustysearch, types::Stats};
    use serde_json::json;

    #[test]
    fn test_write_new_stats() {
        let stats = Stats {
            version: String::from("0.1.0"),
            total_docs: 0,
        };

        assert_eq!(stats.version, "0.1.0");
        assert_eq!(stats.total_docs, 0);

        let tmp_path = "/tmp/rustysearch_writenewstats";
        let search = Rustysearch::new(&tmp_path);
        search.setup();

        search.write_stats(stats).unwrap();
    }

    #[test]
    fn test_read_stats() {
        let tmp_path = "/tmp/rustysearch_readstats";
        let search = Rustysearch::new(&tmp_path);
        search.setup();

        clean_stats(tmp_path);

        let stats = search.read_stats().unwrap();
        assert_eq!(stats.version, "0.1.0");
        assert_eq!(stats.total_docs, 0);
    }

    #[test]
    fn test_increment_total_docs() {
        let tmp_path = "/tmp/rustysearch_incrementtotaldocs";
        let search = Rustysearch::new(&tmp_path);
        search.setup();

        clean_stats(tmp_path);

        let stats = search.read_stats().unwrap();
        assert_eq!(stats.total_docs, 0);

        search.increment_total_docs();
        let stats = search.read_stats().unwrap();
        assert_eq!(stats.total_docs, 1);
    }

    #[test]
    fn test_get_total_docs() {
        let tmp_path = "/tmp/rustysearch_gettotaldocs";
        let search = Rustysearch::new(&tmp_path);
        search.setup();

        clean_stats(tmp_path);

        let stats = search.read_stats().unwrap();
        assert_eq!(stats.total_docs, 0);

        search.increment_total_docs();
        let stats = search.read_stats().unwrap();
        assert_eq!(stats.total_docs, 1);

        let total_docs = search.get_total_docs();
        assert_eq!(total_docs, 1);
    }

    #[test]
    fn test_make_ngrams() {
        let search = Rustysearch::new("/tmp/rustysearch_makengrams");
        search.setup();

        let tokens = vec!["hello".to_string(), "world".to_string()];
        let terms = search.make_ngrams(tokens, 3, 6);

        assert_eq!(terms["hel"].len(), 1);
    }

    #[test]
    fn test_hash_name() {
        let search = Rustysearch::new("/tmp/rustysearch_hashname");
        search.setup();

        let hash = search.hash_name("hello", 6);
        assert_eq!(hash, "5d4140");
    }

    #[test]
    fn test_make_segment_name() {
        let search = Rustysearch::new("/tmp/rustysearch_makesegmentname");
        search.setup();

        let segment_name = search.make_segment_name("hello");
        assert_eq!(
            segment_name,
            "/tmp/rustysearch_makesegmentname/index/5d4140.index"
        );
    }

    #[test]
    fn test_parse_record() {
        let search = Rustysearch::new("/tmp/rustysearch_parserecord");
        search.setup();

        let line = "my_term\t{\"frequency\": 100}";
        let (term, info) = search.parse_record(line);

        assert_eq!(term, "my_term");
        assert_eq!(info, "{\"frequency\": 100}");
    }

    #[test]
    fn test_make_tokens() {
        let search = Rustysearch::new("/tmp/rustysearch");
        let tokens = search.make_tokens("Hello, world!");
        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn test_make_record() {
        let search = Rustysearch::new("/tmp/rustysearch");
        let term = "hello world";
        let term_info = json!({
            "frequency": 100,
            "idf": 1.5,
        });

        let record = search.make_record(term, &term_info);
        assert_eq!(record, "hello world\t{\"frequency\":100,\"idf\":1.5}\n");
    }

    // Helper function to clean up the stats file
    fn clean_stats(tmp_path: &str) {
        let search = Rustysearch::new(tmp_path);
        search.setup();

        let new_stats = Stats {
            version: String::from("0.1.0"),
            total_docs: 0,
        };
        search.write_stats(new_stats).unwrap();
    }
}
