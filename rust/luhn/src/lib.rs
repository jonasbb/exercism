pub fn is_valid<T: AsRef<str>>(digits_: T) -> bool {
    if digits_.as_ref().chars().any(|c| !c.is_digit(10) && c != ' ') {
        // contains non-digit
        return false;
    }
    let digits = digits_.as_ref().chars().filter(|c| *c != ' ');
    if digits.count() <= 1 {
        // too few digits
        return false;
    }
    let digits = digits_.as_ref().chars().filter(|c| *c != ' ');

    fn char2digit(c: char) -> usize {
        match c {
            '0'...'9' => c as usize - '0' as usize,
            _ => 0,
        }
    }

    fn double_and_wrap(i: usize) -> usize {
        let tmp = i * 2;
        if tmp > 9 { tmp - 9 } else { tmp }
    }

    let sum: usize = digits.enumerate()
        .map(|(i, c)| if i % 2 == 1 {
            double_and_wrap(char2digit(c))
        } else {
            char2digit(c)
        })
        .sum();

    sum % 10 == 0
}
