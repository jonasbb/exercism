pub fn sum_of_multiples(limit: u64, bases: &[u64]) -> u64 {
    // return true if `x` is divisible by any base in bases
    let is_divisible = |x: u64| {
        for base in bases {
            if x % base == 0 {
                return true;
            }
        }
        false
    };

    (0..limit).filter(|x| is_divisible(*x)).sum()
}
