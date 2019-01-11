extern crate clap;


use clap::{Arg, App};
use std::io::prelude::*;
use std::io;
use levenshtein::levenshtein;


const SEARCH_WORD_ARG_NAME: &str = "search-word";
const MAX_DISTANCE_ARG_NAME: &str = "max-distance";


// Parse the command line arguments
//
// Returns:
//  - search_word: Search word
//  - max_distance: Max. distance to search_word for matches
fn parse_args() -> (String, usize) {
    let matches = App::new("Rust Levenshtein Distance")
        .version("1.0")
        .author("Andreas Zitzelsberger <az@az82.de>")
        .about("Find all words in STDIN that have no more than a given levensthein distance from a search word")
        .arg(Arg::with_name(SEARCH_WORD_ARG_NAME)
            .help("The search word")
            .required(true)
            .index(1))
        .arg(Arg::with_name(MAX_DISTANCE_ARG_NAME)
            .help("The maximum distance from the search string for which results are returned. Default is 1")
            .short("d")
            .default_value("1")
            .validator(|s: String| -> Result<(), String> {
                s.parse::<u64>().and(Ok(()))
                    .map_err(|e: std::num::ParseIntError| -> String{ e.to_string() })
            }))
        .get_matches();

    return (
        String::from(matches.value_of(SEARCH_WORD_ARG_NAME).unwrap()),
        matches.value_of(MAX_DISTANCE_ARG_NAME).unwrap().parse().unwrap());
}

// Return only words that are within a given levenshtein distance from a search word.
//
// Arguments:
//  - words: Iterator over source words
//  - search_word: Search word
//  - max_distance: Max. levenshtein distance from the search word
//
// Returns:
//  Iterator over the result words
fn filter_words<'a>(words: &'a mut Iterator<Item=io::Result<String>>, search_word: &'a str, max_distance: usize) -> FilteredWords<'a> {
    return FilteredWords { words, search_word, max_distance };
}


// Result iterator type for filter_lines
struct FilteredWords<'a> {
    words: &'a mut Iterator<Item=io::Result<String>>,
    search_word: &'a str,
    max_distance: usize,
}

impl<'a> Iterator for FilteredWords<'a> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {

            let option = self.words.next();

            if option.is_some() {
                let result = option.unwrap();
                if result.is_ok() {
                    let line = result.unwrap();
                    if levenshtein(self.search_word, &line) <= self.max_distance {
                        return Some(Ok(line));
                    }
                } else {
                    let err = result.unwrap_err();
                    return Some(Err(io::Error::new(err.kind(), err.to_string())));
                }
            } else {
                return None;
            }
        }
    }
}



fn main() {
    let (search_word, max_distance) = parse_args();

    for result in filter_words(&mut io::stdin().lock().lines(), &search_word, max_distance) {
        match result {
            Ok(v) => println!("{}", v),
            Err(e) => panic!(e)
        }
    }
}
