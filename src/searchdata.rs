use std::{time::Instant, collections::{HashSet, HashMap}};

use crate::document;

pub struct SearchData {
    documents: document::Documents,
    inverted_index: HashMap<String, HashSet<i32>>,
    index_docs: HashMap<i32, document::Item>,
}

impl SearchData {
    pub fn new(
        documents: document::Documents,
        inverted_index: HashMap<String, HashSet<i32>>,
        index_docs: HashMap<i32, document::Item>,
    ) -> Self {
        SearchData {
            documents,
            inverted_index,
            index_docs,
        }
    }

    pub fn search(&self, input: &str) {
        let input = input.trim().to_lowercase();
        match input.as_str() {
            "quit" => {}
            _ => {
                self.search_using_contains(&input);
                self.search_using_regex(&input);
                self.search_using_full_text_search(&input);
            }
        }
    }

    fn search_using_contains(&self, input: &str) {
        let now = Instant::now();
        let result = document::search_using_contains(&self.documents.doc, input);
        let elapsed = now.elapsed();

        for ids in result {
            let i32_ids = ids as i32;
            println!("Results using string contains: {:?}", self.documents.doc.get(i32_ids as usize).unwrap().title);
        } 
        println!("Elapsed: {:.6?}", elapsed);
    }

    fn search_using_regex(&self, input: &str) {
        let now = Instant::now();
        let result = document::search_using_regex(&self.documents.doc, input);
        let elapsed = now.elapsed();

        for ids in result {
            let i32_ids = ids as i32;
            println!("Results using regex: {:?}", self.documents.doc.get(i32_ids as usize).unwrap().title);
        }
        println!("Elapsed: {:.6?}", elapsed);
    }

    fn search_using_full_text_search(&self, input: &str) {
        let tokenized = document::tokenize(input);

        if tokenized.is_empty() {
            println!("No results found");
            return;
        }

        let now = Instant::now();
        for token in tokenized {
            if let Some(ids) = self.inverted_index.get(&token) {
                for id in ids {
                    if let Some(item) = self.index_docs.get(id) {
                        println!("Results using full-text-search: {:?}", item.title);
                    }
                }
            } else {
                println!("No results found");
            }
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.6?}", elapsed);
    }
}