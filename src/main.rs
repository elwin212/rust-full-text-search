use std::collections::{HashMap, HashSet};
use std::io;
extern crate quick_xml;
use std::time::Instant;


mod document;

fn main() {    
    let documents = document::load_file("data/data.xml");
    match documents {
        Ok(documents) => {
            let index_docs = document::assign_index_to_docs(&documents);
            let inverted_index = document::build_inverted_index(&index_docs);
            performance_comparison(documents, inverted_index, index_docs);            
        },
        Err(err) => {
            println!("Error: {:?}", err);
        }
    } 
}

fn performance_comparison(documents: document::Documents, inverted_index: HashMap<String, HashSet<i32>>, index_docs: HashMap<i32, document::Item>) {
    loop {
        println!("Enter a command (or 'quit' to exit):");
                let mut input = String::new();
                // Read user input
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                // Trim whitespace and convert input to lowercase for case-insensitive comparison
                let input = input.trim().to_lowercase();
                if input == "quit" {
                    // Exit the loop when the user enters 'quit'
                    break;
                } else {          
                    /* Search using string contain */
                    let now = Instant::now();
                    let result1 = document::search_using_contains(&documents.doc, &input);
                    let elapsed = now.elapsed();
                    for ids in result1 {
                        let i32_ids = ids as i32;
                        println!("Results using string contains: {:?}", documents.doc.get(i32_ids as usize).unwrap().title);
                    }                        
                    println!("Elapsed: {:.6?}", elapsed);

                    /* Search using regex */
                    let now = Instant::now();
                    let result2 = document::search_using_regex(&documents.doc, &input);
                    let elapsed = now.elapsed();
                    for ids in result2 {
                        let i32_ids = ids as i32;
                        println!("Results using regex: {:?}", documents.doc.get(i32_ids as usize).unwrap().title);
                    }
                    println!("Elapsed: {:.6?}", elapsed);  
                    let tokenized = document::tokenize(&input);
                    if tokenized.len() == 0 {
                        println!("No results found");
                    } else {                        
                        /* Search using full-text-search */
                        let now = Instant::now();                                    
                        for token in tokenized {
                            if inverted_index.contains_key(&token) {
                                
                                for id in inverted_index.get(&token).unwrap() {
                                    println!("Results using full-text-search: {:?}", index_docs.get(id).unwrap().title);
                                }
                            } else {
                                println!("No results found");
                            }                
                        }
                        let elapsed = now.elapsed();
                        println!("Elapsed: {:.6?}", elapsed);
                    }                        
                }
    }
}