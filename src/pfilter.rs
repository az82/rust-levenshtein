extern crate num_cpus;

use std::io;
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use levenshtein::levenshtein;

// Read all lines from the iterator into a vector.
fn to_vec(words: &mut Iterator<Item=io::Result<String>>) -> Vec<String> {
    let mut lines = Vec::new();

    for word in words {
        lines.push(word.unwrap())
    }

    lines
}

// Return only words that are within a given levenshtein distance from a search word.
//
// Uses as many parallel workers as there are logical CPU cores
//
// Arguments:
//  - words: Iterator over source words
//  - search_word: Search word
//  - max_distance: Max. levenshtein distance from the search word
//
// Returns:
//  Iterator over the result words
pub fn filter_words(words: &mut Iterator<Item=io::Result<String>>, search_word: &str, max_distance: usize) -> FilteredWords {
    let lines = to_vec(words);
    let num_workers = num_cpus::get();

    let slice_size = (lines.len() as f64 / num_workers as f64).ceil() as usize;

    let (sender, receiver) = channel();

    for thread_num in 0..num_workers {
        let slice_start =   thread_num * slice_size;
        if slice_start < lines.len() {
            let slice_end = usize::min(slice_start + slice_size, lines.len());
            let words_slice = lines[slice_start..slice_end].to_vec();

            let sender_local = Sender::clone(&sender);
            let search_word_local = search_word.to_string();

            thread::spawn(move || {
                for word in words_slice {
                    if levenshtein(&word, &search_word_local) <= max_distance {
                        sender_local.send(word.to_string()).unwrap();
                    }
                }
            });
        }
    }

    return FilteredWords { receiver };
}


// Result iterator type for filter_lines
pub struct FilteredWords {
    receiver: Receiver<String>,
}


impl Iterator for FilteredWords {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.receiver.recv() {
            Ok(s) => Some(s),
            Err(_) => None // Indicates that there are no more senders
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn filter_words_one() {
        // Given
        let words = vec![
            Ok("tree".to_string()),
            Ok("flower".to_string()),
            Ok("mouse".to_string())];
        let words_iter = &mut words.into_iter();

        // When
        let filtered_words = &mut filter_words(words_iter, "house", 1);

        // Then
        match filtered_words.next() {
            Some(s) => assert_eq!(s, "mouse"),
            None => panic!("assertion failed: first item should be Some(_))")
        }
    }

    #[test]
    fn filter_words_none() {
        // Given
        let words = vec![
            Ok("tree".to_string()),
            Ok("flower".to_string()),
            Ok("stone".to_string())];
        let words_iter = &mut words.into_iter();

        // When
        let filtered_words = &mut filter_words(words_iter, "house", 1);

        // Then
        match filtered_words.next() {
            Some(_) => panic!("assertion failed: first item should be None() but was Some(_)"),
            None => {}
        }
    }

    #[test]
    fn filter_words_err() {
        match panic::catch_unwind(|| {
            // Given
            let words = vec![
                Ok("tree".to_string()),
                Ok("flower".to_string()),
                Err(io::Error::new(io::ErrorKind::Other, ""))];
            let mut words_iter = words.into_iter();

            // When
            filter_words(&mut words_iter, "house", 1);

            // Then
        }) {
            Ok(_) => panic!("assertion failed: expected a panic"),
            Err(_) => {}
        }
    }
}