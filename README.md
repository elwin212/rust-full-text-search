# Simple Full-Text Search in Rust

This Rust program is a simple full-text search tool designed to search through Wikimedia dump data. It provides a straightforward way to index and search through text documents contained within the Wikimedia dump. This README provides an overview of the program, its usage, and how to get started.

## Table of Contents
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Performance](#performance)
- [License](#license)

## Features

- Indexing of text documents from Wikimedia dumps.
- Fast and efficient full-text search capability.
- Command-line interface for ease of use.
- Support for custom queries and filters.

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
   ```
   Results using full-text-search: "Wikipedia: United States v. Google LLC (2023)"   
   Results using full-text-search: "Wikipedia: Google Silicon Initiative"
   Elapsed: 15.201000µs
   ```
In this case (using google as input), the first and second methods take about ~100 ms, while the full-text search takes only 15.2 µs.
This is approximately 8,666.667 times faster!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
