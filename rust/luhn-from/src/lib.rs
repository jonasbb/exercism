pub struct Luhn(bool);

impl Luhn {
    pub fn new(b: bool) -> Luhn {
        Luhn(b)
    }

    pub fn is_valid(&self) -> bool {
        self.0
    }
}

pub fn is_valid(digits_: &str) -> bool {
    if digits_.chars().any(|c| !c.is_digit(10) && c != ' ') {
        // contains non-digit
        return false;
    }
    let digits = digits_.chars().filter(|c| *c != ' ');
    if digits.count() <= 1 {
        // too few digits
        return false;
    }
    let digits = digits_.chars().filter(|c| *c != ' ');

    fn char2digit(c: char) -> usize {
        c.to_digit(10).unwrap_or(0) as usize
    }

    fn double_and_wrap(i: usize) -> usize {
        let tmp = i * 2;
        if tmp > 9 { tmp - 9 } else { tmp }
    }

    let sum: usize = digits
        .enumerate()
        .map(|(i, c)| if i % 2 == 1 {
            double_and_wrap(char2digit(c))
        } else {
            char2digit(c)
        })
        .sum();

    sum % 10 == 0
}

impl<'a> From<&'a str> for Luhn {
    fn from(s: &'a str) -> Luhn {
        Luhn::new(is_valid(s))
    }
}

impl From<String> for Luhn {
    fn from(s: String) -> Luhn {
        Luhn::from(s.as_ref())
    }
}

impl From<u8> for Luhn {
    fn from(i: u8) -> Luhn {
        Luhn::from(format!("{:03}", i))
    }
}

impl From<u16> for Luhn {
    fn from(i: u16) -> Luhn {
        Luhn::from(format!("{:05}", i))
    }
}

impl From<u32> for Luhn {
    fn from(i: u32) -> Luhn {
        Luhn::from(format!("{:09}", i))
    }
}

impl From<u64> for Luhn {
    fn from(i: u64) -> Luhn {
        Luhn::from(format!("{:016}", i))
    }
}

impl From<usize> for Luhn {
    fn from(i: usize) -> Luhn {
        Luhn::from(format!("{:016}", i))
    }
}
