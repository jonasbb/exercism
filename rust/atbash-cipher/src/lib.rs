pub fn encode<T: AsRef<str>>(plain: T) -> String {
    let plain = plain.as_ref();
    let mut res = String::with_capacity(plain.len()*6/5+1);
    let mut i = 0;
    for c in plain.chars() {
        let nc = match c {
            'A'...'Z' => (('z' as u8) - (c as u8 - 'A' as u8)) as char,
            'a'...'z' => (('z' as u8) - (c as u8 - 'a' as u8)) as char,
            '0'...'9' => c,
            _ => continue
        };
        res.push(nc);
        i += 1;
        if i == 5 {
            res.push(' ');
            i = 0;
        }
    };

    // truncate trailing ' '
    if i == 0 {
        let x = res.len()-1;
        res.truncate(x);
    };
    res
}

pub fn decode<T: AsRef<str>>(cipher: T) -> String {
    let cipher = cipher.as_ref();
    let mut res = String::with_capacity(cipher.len()*5/6+1);
    for c in cipher.chars() {
        let nc = match c {
            'a'...'z' => (('z' as u8) - (c as u8 - 'a' as u8)) as char,
            '0'...'9' => c,
            _ => continue
        };
        res.push(nc);
    };
    res
}