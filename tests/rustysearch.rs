#[cfg(test)]
mod tests {
    use rustysearch::{types::Stats, search::Rustysearch};

    #[test]
    fn test_write_new_stats(){
        let stats = Stats{
            version: String::from("0.1.0"),
            total_docs: 0,
        };

        assert_eq!(stats.version, "0.1.0");
        assert_eq!(stats.total_docs, 0);

        let search = Rustysearch::new("/tmp/rustysearch");
        search.setup();

        search.write_stats(stats);
    }

    #[test]
    fn test_read_stats(){
        let search = Rustysearch::new("/tmp/rustysearch");
        search.setup();

        clean_stats();

        let stats = search.read_stats();
        assert_eq!(stats.version, "0.1.0");
        assert_eq!(stats.total_docs, 0);
    }

    #[test]
    fn test_increment_total_docs(){
        let search = Rustysearch::new("/tmp/rustysearch");
        search.setup();

        clean_stats();

        let stats = search.read_stats();
        assert_eq!(stats.total_docs, 0);

        search.increment_total_docs();
        let stats = search.read_stats();
        assert_eq!(stats.total_docs, 1);
    }

    #[test]
    fn test_get_total_docs(){
        let search = Rustysearch::new("/tmp/rustysearch");
        search.setup();

        clean_stats();

        let stats = search.read_stats();
        assert_eq!(stats.total_docs, 0);

        search.increment_total_docs();
        let stats = search.read_stats();
        assert_eq!(stats.total_docs, 1);

        let total_docs = search.get_total_docs();
        assert_eq!(total_docs, 1);
    }

    fn clean_stats(){
        let search = Rustysearch::new("/tmp/rustysearch");
        search.setup();

        let new_stats = Stats{
            version: String::from("0.1.0"),
            total_docs: 0,
        };
        search.write_stats(new_stats);
    }
}