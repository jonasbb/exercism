pub fn square_of_sum(i: u64) -> u64 {
    // first sum (cantors formula), then square
    ((i + 1) * i / 2).pow(2)
}

pub fn sum_of_squares(i: u64) -> u64 {
    // first square, then sum
    (0..i + 1).map(|x| x.pow(2)).fold(0, |a, b| a + b)
}

pub fn difference(i: u64) -> u64 {
    square_of_sum(i) - sum_of_squares(i)
}
