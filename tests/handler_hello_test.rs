#[cfg(test)]
mod tests {
    use rustysearch::handler::hello::say_hello;
    
    #[tokio::test]
    async fn test_say_hello() {
        let result = say_hello().await;
        assert_eq!(result, "Hello, World!");
    }
}