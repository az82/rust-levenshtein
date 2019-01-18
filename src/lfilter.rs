use std::io;
use levenshtein::levenshtein;


// Return only words that are within a given levenshtein distance from a search word.
//
// Arguments:
//  - words: Iterator over source words
//  - search_word: Search word
//  - max_distance: Max. levenshtein distance from the search word
//
// Returns:
//  Iterator over the result words
pub fn filter_words<'a>(words: &'a mut Iterator<Item=io::Result<String>>, search_word: &'a str, max_distance: usize) -> FilteredWords<'a> {
    return FilteredWords { words, search_word, max_distance };
}


// Result iterator type for filter_lines
pub struct FilteredWords<'a> {
    words: &'a mut Iterator<Item=io::Result<String>>,
    search_word: &'a str,
    max_distance: usize,
}


impl<'a> Iterator for FilteredWords<'a> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.words.next() {
                Some(r) => match r {
                    Ok(w) => if levenshtein(self.search_word, &w) <= self.max_distance {
                        return Some(Ok(w))
                    },
                    Err(e) => return Some(Err(e))
                },
                None => return None
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
            Some(r) => match r {
                Ok(w) => assert_eq!(w, "mouse"),
                Err(_) => panic!("assertion failed: first item should be Ok(_), but was Err(_)")
            }
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
        // Given
        let words = vec![
            Ok("tree".to_string()),
            Ok("flower".to_string()),
            Err(io::Error::new(io::ErrorKind::Other, ""))];
        let words_iter = &mut words.into_iter();

        // When
        let filtered_words = &mut filter_words(words_iter, "house", 1);

        // Then
        match filtered_words.next() {
            Some(r) => match r {
                Ok(_) => panic!("assertion failed: first item should be ERR(_))"),
                Err(_) => {}
            }
            None => panic!("assertion failed: first item should be Some(_))")
        }
    }
}