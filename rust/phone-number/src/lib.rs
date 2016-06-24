pub fn pretty_print(value: &str) -> String {
    match get_valid_digits(value) {
        None => "invalid".to_string(),
        Some((a, b, c)) => format!("({}) {}-{}", a, b, c),
    }
}

pub fn area_code(value: &str) -> Option<String> {
    get_valid_digits(value).map(|(a, _, _)| format!("{}", a))
}

pub fn number(value: &str) -> Option<String> {
    get_valid_digits(value).map(|(a, b, c)| format!("{}{}{}", a, b, c))
}

fn get_valid_digits(value: &str) -> Option<(String, String, String)> {
    let mut tmp: Vec<char> = value.chars().filter(|c| c.is_digit(10)).collect();
    if tmp.len() == 11 && tmp[0] == '1' {
        // remove leading 1
        tmp.remove(0);
    }
    if tmp.len() == 10 {
        Some((tmp.iter().map(|x| *x).take(3).collect::<String>(),
              tmp.iter().map(|x| *x).skip(3).take(3).collect::<String>(),
              tmp.iter().map(|x| *x).skip(6).take(4).collect::<String>()))
    } else {
        None
    }
}
