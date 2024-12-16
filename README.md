[![Rust](https://github.com/alexohneander/rustysearch/actions/workflows/rust.yml/badge.svg)](https://github.com/alexohneander/rustysearch/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/alexohneander/rustysearch/graph/badge.svg?token=IE0019X6NK)](https://codecov.io/gh/alexohneander/rustysearch)

# RustySearch

This project is a simple implementation of a search engine in Rust. It uses the BM25 algorithm for ranking documents.
This project is a learning exercise and is not intended for production use.

## Todo

- [x] Store index to Disk
- [] Save multiple indecies

### Features

- Indexing documents: The search engine maintains an index of documents, where each document is associated with a unique identifier.
- Searching: Given a query, the search engine returns the most relevant documents.

### Usage

#### Creating a new instance of SearchEngine

You can create a new instance of the SearchEngine with the new function. It takes two parameters:

- `k1`: The k1 parameter of the BM25 algorithm.
- `b`: The b parameter of the BM25 algorithm.

### Project Structure

The main components of the project are:

- `SearchEngine`: This is the main class that provides the functionality of the search engine.
- `index`: A HashMap that stores the index of the documents.
- `documents`: A HashMap that stores the documents with their unique identifiers.

### Contributing

Contributions are welcome. Please submit a pull request.

### License

This project is licensed under the MIT License.
