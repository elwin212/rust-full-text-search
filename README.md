# Simple Full-Text Search in Rust

This Rust program is a simple full-text search engine designed to search through Wikimedia dump data. It provides a straightforward way to index and search through text documents contained within the Wikimedia dump. This README provides an overview of the program, its performance, and how to get started.
This project is inspired by [Artem Krylysov](https://github.com/akrylysov?tab=repositories) he built a full-text-search using Golang.

## Table of Contents
- [Features](#features)
- [Prerequisites](#prerequisites)
- [What You Need To Know](#what-you-need-to-know)
- [Installation](#installation)
- [Performance](#performance)
- [License](#license)

## Features

- Indexing of text documents from Wikimedia dumps.
- Fast and efficient full-text search capability.
- Support for custom queries and filters.

## What-You-Need-To-Know

In this project, I just implemented a full-text-search engine with basic operation, if you want to build a fully functional engine,
you might need more advanced features such as Relevance Ranking: Implement advanced ranking algorithms, like TF-IDF (Term Frequency-Inverse Document Frequency) or BM25, to provide more accurate and context-aware search results.

### Faceted Search:

Enable users to filter and refine search results by various attributes or categories, making it easier to find specific content.

### Query Suggestions: 

Implement query auto-suggestions based on user input to help users refine their search queries.

### Spell Correction: 

Integrate spell-check and correction functionality to handle typos and misspellings in search queries.

### Synonym Handling: 

Support synonyms and related term searches to broaden search results and improve user experience.

### Phrase Searching: 

Allow users to search for exact phrases by preserving word order and proximity.

### Language Support: 

Extend language support and implement language-specific tokenization and stemming algorithms.

### Scalable Indexing: 

Optimize the indexing process for handling large datasets efficiently. Implement incremental or distributed indexing as needed.

### Cross-Language Search: 

Enable searching across multiple languages by implementing language detection and translation features.

### Geo-Spatial Search: 

Add geospatial search capabilities, allowing users to find content based on location or geographic coordinates.

### Real-Time Index Updates: 

Support real-time updates to the search index to keep data current and responsive to changes.

### Content Highlighting: 

Highlight search query matches within search results to provide users with context.

## Prerequisites

Before you can use this program, ensure you have the following prerequisites installed:

- [Rust](https://www.rust-lang.org/) and [Cargo](https://crates.io/): The program is written in Rust, so you need the Rust programming language and its package manager, Cargo.

## Installation

1. Clone this repository to your local machine:

   ```shell
   git clone https://github.com/elwin212/rust-full-text-search.git
   ```
2. Navigate to the project directory:
   ```shell
   cd rust-full-text-search
   ```

3. Download the Wikipedia dump dataset from [Wikimedia Downloads](https://dumps.wikimedia.org/).

4. Create a new folder name "data".

5. Put the dataset you just downloaded into data folder and name it data.xml

6. Run the project:
    ```shell
    cargo run
    ```
## Performance

In this section, we'll compare the performance of three different title search methods: string contains, regex, and full-text search.
The Wikipedia dump contains over 600K data and it should be a good case to test the performance.
Here I will use word "google" as the input.

### String Contains Search

String contains search involves searching for a substring within the title. This method is simple and straightforward but may not be the most efficient for large datasets.

```shell
Results using string contains: "Wikipedia: United States v. Google LLC (2023)"
Results using string contains: "Wikipedia: Google Silicon Initiative"
Elapsed: 129.966060ms
```


### Regex Search

Regex search allows for more complex pattern matching in titles. While it provides flexibility, it can be slower than string contains search, especially for complex regex patterns or large datasets.

```shell
Results using regex: "Wikipedia: United States v. Google LLC (2023)"
Results using regex: "Wikipedia: Google Silicon Initiative"
Elapsed: 96.607663ms
```

### Full-Text Search

Full-text search is a powerful and efficient way to search through large amount of text. It indexes the dataset and uses advanced algorithms to find relevant text quickly. This method is recommended for large datasets and complex queries.

```shell
Results using full-text-search: "Wikipedia: United States v. Google LLC (2023)"   
Results using full-text-search: "Wikipedia: Google Silicon Initiative"
Elapsed: 15.201000µs
```

In this case (using *google* as input), the first and second methods take about *~100 ms*, while the full-text search takes only *15.2 µs*.
This is approximately **8,500** times faster!
But to be honest, I believe that matching all the titles using regex in under *200ms* across over 600K documents is quite fast. I think this is one of the reasons why many people consider Rust to be a 'fast' language.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
