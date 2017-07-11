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
        .rev()
        .enumerate()
        .map(|(i, c)| if i % 2 == 1 {
            double_and_wrap(char2digit(c))
        } else {
            char2digit(c)
        })
        .sum();

    sum % 10 == 0
}

impl<T: ToString> From<T> for Luhn {
    fn from(t: T) -> Luhn {
        Luhn::new(is_valid(&t.to_string()))
    }
}
