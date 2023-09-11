use std::collections::HashMap;
use std::io;
extern crate quick_xml;
use std::time::Instant;


mod document;

fn main() {    
    let documents = document::load_file("data/data.xml");
    match documents {
        Ok(documents) => {
            let mut index_docs: HashMap<i32, document::Item> = HashMap::new();
            for (id, item) in documents.doc.iter().enumerate() {
                let id = id as i32 + 1; // Assign IDs starting from 1
                index_docs.insert(id, item.clone());
            }
            let inverted_index = document::build_inverted_index(&index_docs);
            /*let now = Instant::now();
            let result1 = search_using_contains(&documents.doc, "test");
            let elapsed = now.elapsed();
            println!("Elapsed: {:.6?}", elapsed);
            println!("Search: test; Len: {}; First: {}", result1.len(), documents.doc[result1[0]].title);

            let now = Instant::now();
            let result2 = search_using_regex(&documents.doc, "test");
            let elapsed = now.elapsed();
            println!("Search: test; Len: {}; First: {}", result2.len(), documents.doc[result2[0]].title);
            println!("Elapsed: {:.6?}", elapsed);
            
            
            
            let now = Instant::now();
            let result3 = inverted_index.get("test");
            let elapsed = now.elapsed();
            println!("Elapsed: {:.6?}", elapsed);
            println!("Search: test; Len: {}", result3.unwrap().len());
            */
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
                    let tokenized = document::tokenize(&input);
                    if tokenized.len() == 0 {
                        println!("No results found");
                    } else {
                        let now = Instant::now();            
                        
                        for token in tokenized {
                            if inverted_index.contains_key(&token) {
                                for id in inverted_index.get(&token).unwrap() {
                                    println!("Result: {:?}", index_docs.get(id).unwrap().title);
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
            
        },
        Err(err) => {
            println!("Error: {:?}", err);
        }
    } 
}