extern crate itertools;

use itertools::Itertools;

pub fn anagrams_for<'a>(word: &str, canditates: &'a [&str]) -> Vec<&'a str> {
    let word_low = word.to_lowercase();
    let cmp = word_low.chars().sorted();

    let mut res = Vec::new();
    for &c in canditates.iter() {
        let low = c.to_lowercase();
        let sort = low.chars().sorted();
        if sort == cmp && low != word_low {
            res.push(c);
        }
    }
    res
}
