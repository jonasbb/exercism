pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let digits_ = self.to_string();

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
}
