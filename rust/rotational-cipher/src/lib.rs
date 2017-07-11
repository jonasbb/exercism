pub fn rotate(s: &str, n: u8) -> String {
    s.chars().map(|c| rotate_char(c, n)).collect()
}

fn rotate_char(c: char, n: u8) -> char {
    match c {
        _ if c >= 'a' && c <= 'z' => ((((c as u8 - b'a') + n) % 26) + b'a') as char,
        _ if c >= 'A' && c <= 'Z' => ((((c as u8 - b'A') + n) % 26) + b'A') as char,
        _ => c,
    }
}