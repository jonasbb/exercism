pub fn hamming_distance(strand_a: &str, strand_b: &str) -> Result<usize, &'static str> {
    if strand_a.chars().count() != strand_b.chars().count() {
        Result::Err("inputs of different length")
    } else {
        Ok(strand_a.chars().zip(strand_b.chars()).filter(|&(x, y)| x != y).count())
    }
}
