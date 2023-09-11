# Simple Full-Text Search in Rust

This Rust program is a simple full-text search tool designed to search through Wikimedia dump data. It provides a straightforward way to index and search through text documents contained within the Wikimedia dump. This README provides an overview of the program, its usage, and how to get started.

## Table of Contents
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
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
   ```
   cd rust-full-text-search

   ```

3. Download the Wikipedia dump dataset from [Wikimedia Downloads](https://dumps.wikimedia.org/).

4. Create a new folder name "data".

5. Put the dataset you just downloaded into data folder and name it data.xml

6. Run the project:
    ```
    cargo run

    ```

