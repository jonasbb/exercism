pub fn encode(s: &str) -> String {
    let mut res = String::new();

    {
        let mut add_encoding = |count: usize, c: char| {
            if count == 1 {
                // special casing, when there is no number in front of the letter
                res.push(c);
            } else if count > 1 {
                res += &format!("{}{}", count, c);
            }
        };

        let mut count = 0;
        let mut tmpchar = 0 as char;

        for c in s.chars() {
            if tmpchar == c {
                 count += 1;
                 continue;
            }
            add_encoding(count, tmpchar);
            count = 1;
            tmpchar = c;
        }
        add_encoding(count, tmpchar);
    }
    res
}

pub fn decode(s: &str) -> String {
    let mut res = String::new();

    let mut count = 0;
    for c in s.chars() {
        if let Some(num) = c.to_digit(10) {
            count = count * 10 + num;
        } else {
            // special casing, when there is no number in front of the letter
            if count == 0 {
                count = 1;
            }

            for _ in 0..count {
                res.push(c);
            }
            count = 0;
        }
    }

    res
}