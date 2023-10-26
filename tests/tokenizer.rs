#[cfg(test)]
mod tests {
    use rustysearch::{analyze::tokenizer::Tokenizer};

    #[test]
    fn test_split_into_words() {
        let text = "The quick brown fox jumps over the lazy dog.";
        let stopwords = vec!["the".to_string(), "over".to_string()];
        let tokenizer = Tokenizer::new(text, stopwords, None);
        let words = tokenizer.split_into_words();
        assert_eq!(
            words,
            vec![
                "quick".to_string(),
                "brown".to_string(),
                "fox".to_string(),
                "jumps".to_string(),
                "lazy".to_string(),
                "dog".to_string(),
            ]
        );
    }

    #[test]
    fn test_split_into_sentences() {
        let text = "The quick brown fox jumps over the lazy dog. The end.";
        let stopwords = vec!["the".to_string(), "over".to_string()];
        let tokenizer = Tokenizer::new(text, stopwords, None);
        let sentences = tokenizer.split_into_sentences();
        assert_eq!(
            sentences,
            vec![
                "quick brown fox jumps lazy dog".to_string(),
                "end".to_string(),
            ]
        );
    }

    #[test]
    fn test_split_into_paragraphs() {
        let text = "The quick brown fox jumps over the lazy dog.\n\nThe end.";
        let stopwords = vec!["the".to_string(), "over".to_string()];
        let tokenizer = Tokenizer::new(text, stopwords, None);
        let paragraphs = tokenizer.split_into_paragraphs();
        assert_eq!(
            paragraphs,
            vec![
                "quick brown fox jumps lazy dog".to_string(),
                "end".to_string(),
            ]
        );
    }
}
