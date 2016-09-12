pub struct Brackets {
    data: &'static str,
}

impl From<&'static str> for Brackets {
    fn from(from: &'static str) -> Brackets {
        Brackets::new(from)
    }
}

impl Brackets {
    pub fn new(data: &'static str) -> Brackets {
        Brackets { data: data }
    }

    pub fn are_balanced(&self) -> bool {
        let mut stack = Vec::new();
        for chr in self.data.chars() {
            match chr {
                '{' | '(' | '[' => stack.push(chr),
                '}' | ')' | ']' => {
                    // check for match
                    match stack.pop() {
                        Some(c) => {
                            if !Brackets::is_matching_bracket(c, chr) {
                                // found first mismatch
                                return false;
                            }
                        }
                        // unmatched bracket
                        None => return false,
                    }
                    return true;
                }
                // ignore all non bracket chars
                _ => {}
            }
        }

        // check that all brackets are removed
        // only then everything is matched
        stack.is_empty()
    }

    /// Determine if the two characters are matching brackets
    fn is_matching_bracket(c1: char, c2: char) -> bool {
        match (c1, c2) {
            ('{', '}') | ('(', ')') | ('[', ']') => true,
            _ => false,
        }
    }
}
