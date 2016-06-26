use std::collections::HashMap;
use std::rc::Rc;
use Token::*;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    /// Stores the definitions of each word
    /// must be initialized to the system default words
    definitions: HashMap<String, Rc<Vec<Token>>>,
}

/// List of tokens which can occur in a Forth program.
#[derive(Debug,PartialEq,Eq,Clone)]
enum Token {
    Number(Value),
    /// Represents anything that is not a number or one of the punctuation symbols. Even the
    /// predefined word like `dup` or `swap` are converted into an Identifier while tokenizing a
    /// string. These predefined word have an already existing entry to the special tokens `Dup`,
    /// `Drop`, `Over`, `Swap` which actually do work.
    Identifier(String),

    Plus,
    Minus,
    Star,
    Slash,
    Colon,
    Semicolon,

    // special tokens, represent the predefined behaviour of the corresponding Identifiers
    Dup,
    Drop,
    Over,
    Swap,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
    /// New error condition for `;` which occur without a `:` first
    EndOfDefinitionWithoutStart,
}

impl Forth {
    pub fn new() -> Forth {
        let mut definitions = HashMap::new();
        // create default mappings for pre-defined words
        definitions.insert("drop".to_string(), Rc::new(vec![Drop]));
        definitions.insert("dup".to_string(), Rc::new(vec![Dup]));
        definitions.insert("swap".to_string(), Rc::new(vec![Swap]));
        definitions.insert("over".to_string(), Rc::new(vec![Over]));
        Forth {
            stack: vec![],
            definitions: definitions,
        }
    }

    /// Prints all numbers on the stack with ` ` as separation character
    pub fn format_stack(&self) -> String {
        let mut res = String::new();
        for val in self.stack.as_slice().iter() {
            res.push_str(&format!("{}", val));
            res.push(' ');
        }
        // remove trailing ' ' if at least 1 char
        let l = if res.is_empty() {
            0
        } else {
            res.len() - 1
        };
        res.truncate(l);
        res
    }

    /// Looks up the stored definition of token for the word `word`.
    /// Returns `Error::UnknownWord` if the word is not in the internal hashmap.
    fn get_word_definition(&self, word: &str) -> Result<Rc<Vec<Token>>, Error> {
        if let Some(x) = self.definitions.get(word) {
            Ok(x.clone())
        } else {
            Err(Error::UnknownWord)
        }
    }

    /// Evaluate a string as Forth program. It may be called multiple times. The previous state
    /// will be reused.
    pub fn eval(&mut self, input: &str) -> ForthResult {
        let tokenizer = ForthTokenizer::new(input);
        try!{self.eval_vec(&tokenizer.collect::<Vec<_>>())};
        Ok(())
    }

    /// Abstraction method to process both token streams from str input and stored commands in the
    /// hashmap.
    fn eval_vec(&mut self, input: &[Token]) -> ForthResult {
        let mut tokenstream = input.iter();
        loop {
            match tokenstream.next() {
                None => break,
                Some(token) => {
                    match *token {
                        Number(n) => self.stack.push(n),

                        // arbitrary identifiers
                        Identifier(ref iden) => {
                            let tmp = try!{self.get_word_definition(iden)};
                            try!{self.eval_vec(tmp.as_ref())}
                        }

                        Semicolon => {
                            // may only occur after colon
                            return Err(Error::EndOfDefinitionWithoutStart);
                        }
                        Colon => {
                            let mut new_word_commands = Vec::new();

                            // must be Identifier
                            let t = tokenstream.next();
                            if let Some(&Identifier(ref id)) = t {
                                // consume all tokens until `;` is reached and append to
                                // `new_word_commands`
                                loop {
                                    match tokenstream.next() {
                                        None => return Err(Error::InvalidWord),
                                        Some(&Semicolon) => {
                                            // end of word, store in hashmap for later reuse
                                            self.definitions
                                                .insert(id.to_string(), Rc::new(new_word_commands));
                                            break;
                                        }
                                        Some(&Identifier(ref subiden)) => {
                                            // compiler mode:
                                            // copy definition of used word into new word
                                            new_word_commands.append(&mut try!{
                                                    self.get_word_definition(subiden)
                                                }
                                                .as_ref()
                                                .clone())
                                        }
                                        Some(token) => new_word_commands.push(token.clone()),
                                    }
                                }

                            } else {
                                // identifier expected but got something else
                                return Err(Error::InvalidWord);
                            }
                        }

                        // Operations with 1 operand
                        Dup | Drop => {
                            match self.stack.pop() {
                                Some(v1) => {
                                    match *token {
                                        Dup => {
                                            self.stack.push(v1);
                                            self.stack.push(v1);
                                        }
                                        Drop => {}
                                        _ => unreachable!(),
                                    }
                                }
                                None => return Err(Error::StackUnderflow),
                            }
                        }

                        // Operations with 2 operand
                        Plus | Minus | Star | Slash | Over | Swap => {
                            match (self.stack.pop(), self.stack.pop()) {
                                (Some(v1), Some(v2)) => {
                                    match *token {
                                        Plus => self.stack.push(v2 + v1),
                                        Minus => self.stack.push(v2 - v1),
                                        Star => self.stack.push(v2 * v1),
                                        Slash => {
                                            if v1 == 0 {
                                                return Err(Error::DivisionByZero);
                                            }
                                            self.stack.push(v2 / v1)
                                        }
                                        Swap => {
                                            self.stack.push(v1);
                                            self.stack.push(v2);
                                        }
                                        Over => {
                                            self.stack.push(v2);
                                            self.stack.push(v1);
                                            self.stack.push(v2);
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                (_, _) => return Err(Error::StackUnderflow),
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[test]
#[allow(unused_must_use)]
fn redefine_word_previously_used() {
    let mut f = Forth::new();
    f.eval(": foo dup ;");
    f.eval(": dup swap ;");
    f.eval("1 2 foo");
    assert_eq!("1 2 2", f.format_stack());
}

#[test]
#[allow(unused_must_use)]
fn redefine_in_loop() {
    let mut f = Forth::new();
    f.eval(": swap dup ;");
    f.eval(": dup swap ;");
    f.eval(": swap dup ;");
    f.eval("1 swap");
    assert_eq!("1 1", f.format_stack());
}

/// Helper tool to convert a string into a list of Forth tokens.
struct ForthTokenizer<'a> {
    data: &'a str,
    buf: String,
}

impl<'a> ForthTokenizer<'a> {
    fn new(data: &'a str) -> ForthTokenizer {
        ForthTokenizer {
            data: data,
            buf: String::with_capacity(32),
        }
    }
}

impl<'a> Iterator for ForthTokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // assumption: all data only consists of unicode symbols which are a single char
        // no glyph consisting out of multiple chars

        // reset buffer
        self.buf.truncate(0);

        // generate iterator
        let mut chars = self.data.chars();
        // need to write the for loop on my own, otherwise the iterator would be moved
        // and I cannot restore the &str value below
        // Consume everything separated by control or whitespace as a single token
        while let Some(c) = chars.next() {
            // not totally sure this is the correct filter, but seems to pass all the tests
            if c.is_control() || c.is_whitespace() {
                // exit if at least one token read
                if !self.buf.is_empty() {
                    break;
                }
            } else {
                self.buf.push(c);
            }
        }
        // save rest of string slice back
        self.data = chars.as_str();

        // process the buffer and output the token
        match self.buf.to_lowercase().as_str() {
            // empty string marks exhaustion of the chars() iterator
            // therefore the tokenizer is also exhausted
            "" => None,

            // symbols
            ":" => Some(Colon),
            ";" => Some(Semicolon),
            "+" => Some(Plus),
            "-" => Some(Minus),
            "*" => Some(Star),
            "/" => Some(Slash),

            // numbers and identifiers
            x => {
                // try to parse it as a number, otherwise it is an identifier
                match x.parse::<Value>() {
                    Ok(v) => Some(Number(v)),
                    Err(_) => Some(Identifier(x.to_lowercase())),
                }
            }
        }
    }
}

// Tests for the tokenizer
#[test]
#[allow(unused_must_use)]
fn tokenize_number() {
    let expected = vec![Number(1)];
    let input = "1";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_numbers() {
    let expected = vec![Number(1), Number(256), Number(42)];
    let input = "1 256 42";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_numbers_multi_whitespace() {
    let expected = vec![Number(1), Number(256), Number(42)];
    let input = "1       256             42";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_identifier() {
    let expected = vec![Identifier("a".to_string())];
    let input = "a";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_identifiers() {
    let expected = vec![Identifier("abc".to_string()),
                        Identifier("hello".to_string()),
                        Identifier("world".to_string())];
    let input = "abc hello world";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_identifier_normalization() {
    let expected = vec![Identifier("helloworld".to_string())];
    let input = "HeLLOwOrlD";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_non_word_separators() {
    let expected = vec![Number(1), Number(2), Number(3), Number(4), Number(5), Number(6),
                        Number(7)];
    let input = "1\u{0000}2\u{0001}3\n4\r5 6\t7";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_colon() {
    let expected = vec![Colon];
    let input = ":";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_semicolon() {
    let expected = vec![Semicolon];
    let input = ";";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_plus() {
    let expected = vec![Plus];
    let input = "+";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_minus() {
    let expected = vec![Minus];
    let input = "-";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_star() {
    let expected = vec![Star];
    let input = "*";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_slash() {
    let expected = vec![Slash];
    let input = "/";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_dup() {
    let expected = vec![Identifier("dup".to_string()), Identifier("dup".to_string())];
    let input = "dup dUp";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_drop() {
    let expected = vec![Identifier("drop".to_string()), Identifier("drop".to_string())];
    let input = "drop DrOp";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_swap() {
    let expected = vec![Identifier("swap".to_string()), Identifier("swap".to_string())];
    let input = "swap sWaP";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}

#[test]
#[allow(unused_must_use)]
fn tokenize_over() {
    let expected = vec![Identifier("over".to_string()), Identifier("over".to_string())];
    let input = "over OvEr";
    let tokenizer = ForthTokenizer::new(input);
    let result: Vec<Token> = tokenizer.collect();
    assert_eq!(result, expected);
}
