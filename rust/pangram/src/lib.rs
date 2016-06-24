pub fn is_pangram(sentence: &str) -> bool {
    // case insensitive, lowercase
    let ascii_lower = "abcdefghijklmnopqrstuvwxyz";
    let sentence_lower = sentence.to_lowercase();

    for c in ascii_lower.chars() {
        if !sentence_lower.contains(c) {
            return false;
        }
    }
    // all chars tested
    true
}
