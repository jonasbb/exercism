pub fn raindrops(num: u64) -> String {
    let mut res = String::with_capacity(15);
    if num % 3 == 0 {
        res.push_str("Pling")
    };
    if num % 5 == 0 {
        res.push_str("Plang")
    };
    if num % 7 == 0 {
        res.push_str("Plong")
    };

    // not divisible by anything above
    if res.is_empty() {
        res.push_str(&num.to_string())
    };
    res
}
