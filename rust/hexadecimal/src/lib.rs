pub fn hex_to_int(hex: &str) -> Option<u64> {
    let mut res = 0;
    // TODO: fails if hex digits are larger than u64

    for x in hex.chars() {
        res *= 16;
        res += match x {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,

            // error in decoding
            _ => return None,
        };
    }
    Some(res)
}
