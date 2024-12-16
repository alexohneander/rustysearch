#[cfg(test)]
mod tests {
    use rustysearch::types::config::Config;

    #[test]
    fn test_search_engine() {
        let config = Config::default();
        let config_two = Config::new();
        println!("{:?}", config);
        println!("{:?}", config_two);

        // assert_eq!(config.http, 1);
    }
}
