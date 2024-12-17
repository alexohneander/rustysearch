[![Rust](https://github.com/alexohneander/rustysearch/actions/workflows/rust.yml/badge.svg)](https://github.com/alexohneander/rustysearch/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/alexohneander/rustysearch/graph/badge.svg?token=IE0019X6NK)](https://codecov.io/gh/alexohneander/rustysearch)

# RustySearch

This project is a simple implementation of a search engine in Rust. It uses the BM25 algorithm for ranking documents.
This project is a learning exercise and is not intended for production use.

## Todo

- [x] Store index to Disk
- [ ] Save multiple Indecies
- [ ] Benchmark the Index/SearchEngine

### Features

- Indexing documents: The search engine maintains an index of documents, where each document is associated with a unique identifier.
- Searching: Given a query, the search engine returns the most relevant documents.
- BTree: The index is saved as a BTreeMap on the hard disk and loaded from the hard disk into RAM when the system is started. 

### Usage

#### Dev Setup/Usage

```bash
cargo run
```

**Add Document to Index:**
```bash
curl --request POST \
  --url http://localhost:4000/search/index/document \
  --header 'Content-Type: application/json' \
  --data '{
  "url": "https://de.wikipedia.org/wiki/Rust_(Programmiersprache)",
  "content": "Rust ist eine Multiparadigmen-Systemprogrammiersprache, die von der Open-Source-Community entwickelt wurde und unter anderem von Mozilla Research gesponsert wird.[12] Sie wurde mit dem Ziel entwickelt, sicher, nebenläufig und praxisnah zu sein.[13] Sicherheit bezieht sich dabei insbesondere auf die Vermeidung von Programmfehlern, die zu Speicherzugriffsfehlern oder Pufferüberläufen und damit unter Umständen auch zu Sicherheitslücken führen, vor allem auch in nebenläufigen Prozessen. Im Gegensatz zu anderen Programmiersprachen mit automatischer Speicherverwaltung verwendet Rust hierfür keine Garbage Collection, sondern ein besonderes Typsystem. Dessen Typsicherheit wurde formal bewiesen."
}'
```

**Search Query:**
```bash
curl --request GET \
  --url 'http://localhost:4000/search?query=Rust'
```

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
