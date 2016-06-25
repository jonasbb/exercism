pub fn hex_to_int(hex: &str) -> Option<u64> {
    let mut res = 0;
    // TODO: fails if hex digits are larger than u64

    for x in hex.chars() {
        res *= 16;
        res += match x {
            '0'...'9' => (x as u8 - b'0') as u64,
            'A'...'F' => (x as u8 - b'A' + 10u8) as u64,
            'a'...'f' => (x as u8 - b'a' + 10u8) as u64,

            // error in decoding
            _ => return None,
        };
    }
    Some(res)
}
