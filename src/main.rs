use rustysearch::search::engine::SearchEngine;

fn main() {
    let mut engine = SearchEngine::new(1.5, 0.75);
    engine.index("https://www.rust-lang.org/", "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.");
    engine.index("https://en.wikipedia.org/wiki/Rust_(programming_language)", "Rust is a multi-paradigm system programming language focused on safety, especially safe concurrency.");

    let query = "Rust programming language threads";
    let results = engine.search(query);
    for (url, score) in results {
        println!("{}: {}", url, score);
    }
}