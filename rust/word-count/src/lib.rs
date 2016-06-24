use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut res = HashMap::new();

    // words are combinations from alphabetic chars and digits
    // alphabetic chars can be covered with a base of 36
    for word in words.split(|x: char| !x.is_digit(36)) {
        if word == "" {
            continue;
        }
        // words are normalized to lowercase
        let mut count = res.entry(word.to_lowercase().to_string()).or_insert(0);
        *count += 1;
    }
    res
}
