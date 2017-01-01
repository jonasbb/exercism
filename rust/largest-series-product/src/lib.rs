#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;

pub fn lsp<T: AsRef<str>>(digits: T, length: usize) -> Result<u64> {
    let digits = digits.as_ref();
    if digits.len() < length {
        bail!("The digits string must at least be {} characters in size but was only {}",
              length,
              digits.len());
    }
    let mut new_digits = Vec::with_capacity(digits.len());
    for c in digits.chars() {
        new_digits.push(char_to_digit(c).chain_err(|| "Cannot convert string to int array")? as u64)
    }

    let mut max_value = 0;
    for start_index in 0..(new_digits.len() - length + 1) {
        let tmp_value = new_digits[start_index..(start_index + length)].iter().product();
        if tmp_value > max_value {
            max_value = tmp_value
        };
    }
    Ok(max_value)
}

fn char_to_digit(c: char) -> Result<u8> {
    match c {
        '0'...'9' => Ok(c as u8 - '0' as u8),
        _ => bail!("'{}' is not a digit"),
    }
}
