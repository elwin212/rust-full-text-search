use std::{collections::{HashSet, HashMap}, path::Path, io, fs::File};
use regex::Regex;
use strip_markdown::strip_markdown;
use serde::{Deserialize, Serialize};
extern crate quick_xml;
use serde_xml_rs::{from_str};
use std::io::prelude::*;



#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Item {
    pub title: String,
    pub url: String,
    pub r#abstract: Option<String>,    
}

impl Item {
    pub fn new(title: &str, url: &str, r#abstract: Option<&str>) -> Item {
        Item {
            title: title.to_string(),
            url: url.to_string(),
            r#abstract: r#abstract.map(|s| s.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Document {
    pub doc: Vec<Item>
}


pub fn build_inverted_index(documents: &HashMap<i32, Item>) -> HashMap<String, HashSet<i32>> {
    let mut inverted_index: HashMap<String, HashSet<i32>> = HashMap::new();

    for (id, item) in documents.iter() {
        for token in tokenize(&item.title) {
            let entry = inverted_index.entry(token.clone()).or_insert_with(HashSet::new);
            entry.insert(*id);
        }
    }

    inverted_index
}

pub fn load_file(file_str: &str) -> Result<Document, io::Error> {
    let path = Path::new(&file_str);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("couldn't open {}: {}", display, err),
        Ok(file) => file,
    };

    let mut xml = String::new();
    match file.read_to_string(&mut xml) {
        Err(err) => panic!("couldn't read {}: {}", display, err),
        Ok(_) => print!("{} ok", display),
    }

    print!("'{}'", xml);
    let document = from_str(&xml);
    match document {
        Err(err) => panic!("couldn't parse {}: {}", display, err),
        Ok(document) => {
            return Ok(document);
        }
    }    
}

pub fn cleanup(s: String) -> String {
    s.replace(|c: char| !(c.is_alphabetic() || c == '\''), " ")
}

pub fn tokenize(text: &str) -> HashSet<String> {
    let stopwords: HashSet<String> = include_str!("../stopwords/en.txt")
    .split_whitespace()
    .map(String::from)
    .collect();
    cleanup(strip_markdown(text)).split_whitespace()
        .filter(|word| !word.trim().is_empty())
        .map(str::to_lowercase)
        .filter(|word| !stopwords.contains(word))
        .collect()
}

pub fn search_using_contains(docs: &Vec<Item>, term: &str) -> Vec<usize> {
    let mut indexes = Vec::with_capacity(20);
    for (pos, e) in docs.iter().enumerate() {
        if e.title.contains(term) {
            indexes.push(pos)
        }        
    }
    return indexes;
}

pub fn search_using_regex(docs: &Vec<Item>, term: &str) -> Vec<usize> {
    //let formatted = format!(r"(?i)\b{}\b", term);
    let formatted = format!(r"(?i)\w*{}\w*", term);
    let re = Regex::new(formatted.as_str()).unwrap();

    let mut indexes = Vec::with_capacity(20);
    for (pos, e) in docs.iter().enumerate() {
        if re.is_match(&e.title) {
            indexes.push(pos)
        }
    }
    return indexes;
}

pub fn intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut r = Vec::with_capacity(a.len().max(b.len()));
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            i += 1;
        } else if a[i] > b[j] {
            j += 1;
        } else {
            r.push(a[i]);
            i += 1;
            j += 1;
        }
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![3, 4, 5, 6, 7];
        let c = vec![3, 4, 5];
        let d = vec![1, 2, 3, 4, 5, 6, 7];

        assert_eq!(intersection(&a, &b), c);
        assert_eq!(intersection(&b, &a), c);
        assert_eq!(intersection(&a, &d), a);
        assert_eq!(intersection(&d, &a), a);
    }    
    
}