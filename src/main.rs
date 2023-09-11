use std::io;

mod document;
mod searchdata;


fn main() {    
    let documents = document::load_file("data/data.xml");
    match documents {
        Ok(documents) => {
            let index_docs = document::assign_index_to_docs(&documents);
            let inverted_index = document::build_inverted_index(&index_docs);
            let search_data = searchdata::SearchData::new(documents, inverted_index, index_docs);
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
                    search_data.search(&input);
                }
            }            
        },
        Err(err) => {
            println!("Error: {:?}", err);
        }
    } 
}