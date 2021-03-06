#![allow(unknown_lints)]

use std::iter::FromIterator;

#[allow(needless_range_loop)]
pub fn primes_up_to(limit: usize) -> Vec<usize> {
    // 0 represents non prime numbers
    // non 0 values are prime
    let mut res = Vec::from_iter(0..limit + 1);
    res[0] = 0;
    res[1] = 0;
    // test up to sqrt(limit)
    for x in 2..((limit as f64).sqrt().ceil() as usize) {
        // already proven to be non-prime?
        if res[x] == 0 {
            continue;
        } else {
            // mark all multiple as non-prime
            for v in 2..(limit / x) + 1 {
                res[x * v] = 0;
            }
        }
    }
    res.into_iter().filter(|&x| x > 0).collect()
}
