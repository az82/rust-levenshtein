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
                        return Some(Ok(w));
                    },
                    Err(e) => return Some(Err(e))
                },
                None => return None
            }
        }
    }
}
