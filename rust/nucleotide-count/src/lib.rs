use std::collections::HashMap;

pub fn count(marker: char, strand: &str) -> usize {
    strand.chars().filter(|x| x == &marker).count()
}

pub fn nucleotide_counts(strand: &str) -> HashMap<char, usize> {
    let mut res = HashMap::with_capacity(4);
    // all possible DNA markers: ATCG
    for c in "ACGT".chars() {
        res.insert(c, count(c, strand));
    }
    res
}
