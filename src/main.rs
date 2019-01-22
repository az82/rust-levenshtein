extern crate clap;

mod lfilter;
mod pfilter;

use clap::{Arg, App};
use std::io::prelude::*;
use std::io;

const SEARCH_WORD_ARG_NAME: &str = "search-word";
const MAX_DISTANCE_ARG_NAME: &str = "max-distance";
const PARALLEL_ARG_NAME: &str = "parallel";


// Run mode for the CLI executable
enum Mode {
    SERIAL,
    PARALLEL,
}


// Parse the command line arguments
//
// Returns:
//  - search_word: Search word
//  - max_distance: Max. distance to search_word for matches
fn parse_args() -> (String, usize, Mode) {
    let matches = App::new("Rust Levenshtein Distance")
        .version("1.0")
        .author("Andreas Zitzelsberger <az@az82.de>")
        .about("Find all lines in STDIN that are within a given Levensthein distance from a search word")
        .arg(Arg::with_name(SEARCH_WORD_ARG_NAME)
            .help("Search word")
            .required(true)
            .index(1))
        .arg(Arg::with_name(MAX_DISTANCE_ARG_NAME)
            .help("Maximum distance from the search string for which results are returned")
            .short("d")
            .default_value("1")
            .validator(|s: String| -> Result<(), String> {
                s.parse::<u64>().and(Ok(()))
                    .map_err(|e: std::num::ParseIntError| -> String{ e.to_string() })
            }))
        .arg(Arg::with_name(PARALLEL_ARG_NAME)
            .help("Run the search in parallel mode")
            .short("p"))
        .get_matches();

    return (
        String::from(matches.value_of(SEARCH_WORD_ARG_NAME).unwrap()),
        matches.value_of(MAX_DISTANCE_ARG_NAME).unwrap().parse().unwrap(),
        match matches.is_present(PARALLEL_ARG_NAME) {
            true => Mode::PARALLEL,
            false => Mode::SERIAL
        });
}


fn filter_serial(search_word: &str, max_distance: usize) {
    for result in lfilter::filter_words(&mut io::stdin().lock().lines(), &search_word, max_distance) {
        match result {
            Ok(v) => println!("{}", v),
            Err(e) => panic!(e)
        }
    }
}

fn filter_parallel(search_word: &str, max_distance: usize) {
    for word in pfilter::filter_words(&mut io::stdin().lock().lines(), &search_word, max_distance) {
        println!("{}", word);
    }
}


fn main() {
    let (search_word, max_distance, mode) = parse_args();

    match mode {
        Mode::SERIAL => filter_serial(&search_word, max_distance),
        Mode::PARALLEL => filter_parallel(&search_word, max_distance)
    }
}
