use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    SentenceStart,
    EndOfSentence,
    UnknownWord,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseIntError(e)
    }
}

pub struct WordProblem(String);

impl WordProblem {
    pub fn new(command: &str) -> WordProblem {
        WordProblem(command.to_string())
    }

    pub fn answer(&self) -> Result<i64, Error> {
        let mut witer = self.0.split(|c| c == ' ' || c == '?');
        // assert sentence start
        if witer.next() != Some("What") || witer.next() != Some("is") {
            return Err(Error::SentenceStart);
        }

        // TODO there must be a better solution for this double try!{} stuff

        // helper function to parse it
        let mut state =
            try!{ i64::from_str_radix(try!{witer.next().ok_or(Error::EndOfSentence)}, 10 )};
        loop {
            match witer.next() {
                Some("plus") => {
                    let num =
                        try!{ i64::from_str_radix(try!{witer.next().ok_or(Error::EndOfSentence)}, 10 )};
                    state += num;
                }
                Some("minus") => {
                    let num =
                        try!{ i64::from_str_radix(try!{witer.next().ok_or(Error::EndOfSentence)}, 10 )};
                    state -= num;
                }
                Some("multiplied") => {
                    // consume by token
                    witer.next();

                    let num =
                        try!{ i64::from_str_radix(try!{witer.next().ok_or(Error::EndOfSentence)}, 10 )};
                    state *= num;
                }
                Some("divided") => {
                    // consume by token
                    witer.next();

                    let num =
                        try!{ i64::from_str_radix(try!{witer.next().ok_or(Error::EndOfSentence)}, 10 )};
                    state /= num;
                }
                Some("") => {} // ignore empty entries
                Some(_) => {
                    return Err(Error::UnknownWord);
                }
                None => break,
            }
        }
        Ok(state)
    }
}
