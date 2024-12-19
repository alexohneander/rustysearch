#[cfg(test)]
mod tests {
    use rustysearch::types::config::Config;

    #[test]
    fn test_create_new_config() {
        let config = Config::default();
        let config_two = Config::new();

        assert_eq!(config.http_addr, "127.0.0.1:4000");
        assert_eq!(config_two.http_addr, "127.0.0.1:4000");
    }
}
