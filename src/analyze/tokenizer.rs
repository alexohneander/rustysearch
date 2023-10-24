use std::collections::HashSet;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

pub struct Tokenizer {
    text: String,
    stopwords: HashSet<String>,
    punctuation: HashSet<String>,
}

impl Tokenizer {
    pub fn new(text: &str, stopwords: Vec<String>, punctuation: Option<Vec<String>>) -> Self {
        Self {
            text: text.to_owned(),
            stopwords: stopwords
                .iter()
                .map(|s| s.to_owned())
                .collect::<HashSet<String>>(),
            punctuation: punctuation
                .unwrap_or(
                    vec![
                        "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", ";", ".", "/",
                        ":", ",", "<", "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|",
                        "}", "~", "-",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                )
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
        }
    }

    // Split text into words
    pub fn split_into_words(&self) -> Vec<String> {
        self.text
            .split_word_bounds()
            .filter_map(|w| {
                process_word(
                    w,
                    &get_special_char_regex(),
                    &self.stopwords,
                    &self.punctuation,
                )
            })
            .collect::<Vec<String>>()
    }

    pub fn split_into_sentences(&self) -> Vec<String> {
        let special_char_regex = get_special_char_regex();
        get_sentence_space_regex()
            .replace_all(&self.text, ".")
            .unicode_sentences()
            .map(|s| {
                s.split_word_bounds()
                    .filter_map(|w| {
                        process_word(w, &special_char_regex, &self.stopwords, &self.punctuation)
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
    }

    pub fn split_into_paragraphs(&self) -> Vec<String> {
        get_newline_regex()
            .split(&self.text)
            .filter_map(|s| {
                if s.trim().is_empty() {
                    return None;
                }

                Some(
                    s.unicode_sentences()
                        .map(|s| {
                            s.split_word_bounds()
                                .filter_map(|w| {
                                    process_word(
                                        w,
                                        &get_special_char_regex(),
                                        &self.stopwords,
                                        &self.punctuation,
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join(" ")
                        })
                        .collect::<Vec<String>>()
                        .join(" "),
                )
            })
            .collect::<Vec<String>>()
    }
}

fn process_word(
    w: &str,
    special_char_regex: &Regex,
    stopwords: &HashSet<String>,
    punctuation: &HashSet<String>,
) -> Option<String> {
    let word = special_char_regex.replace_all(w.trim(), "").to_lowercase();

    if word.is_empty()
        || (word.graphemes(true).count() == 1) && punctuation.contains(&word)
        || stopwords.contains(&word)
    {
        return None;
    }

    Some(word)
}

fn get_special_char_regex() -> Regex {
    Regex::new(r"('s|,|\.)").unwrap()
}

fn get_sentence_space_regex() -> Regex {
    Regex::new(r"^([\.!?])[\n\t\r]").unwrap()
}

fn get_newline_regex() -> Regex {
    Regex::new(r"(\r|\n|\r\n)").unwrap()
}