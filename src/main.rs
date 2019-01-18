extern crate clap;

mod lfilter;

use clap::{Arg, App};
use std::io::prelude::*;
use std::io;

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

fn main() {
    let (search_word, max_distance) = parse_args();

    for result in lfilter::filter_words(&mut io::stdin().lock().lines(), &search_word, max_distance) {
        match result {
            Ok(v) => println!("{}", v),
            Err(e) => panic!(e)
        }
    }
}
