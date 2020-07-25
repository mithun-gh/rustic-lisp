use std::slice::Iter;
use std::iter::Peekable;

pub enum Token {
    Number(f64),
    Symbol(String),
    Parenthesis(char),
}

#[allow(unused)]
pub struct Lexer<'a> {
    code: Peekable<Iter<'a, char>>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: Iter<'a, char>) -> Self {
        Lexer { code: code.peekable() }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! prepare_input {
        ($name: ident, $string_slice: literal) => {
            let code = $string_slice.chars().collect::<Vec<char>>();
            let mut $name = Lexer::new(code.iter());
        };
    }

    #[test]
    fn empty_code_should_return_none() {
        prepare_input!(lexer, "");
        assert!(lexer.next().is_none());
    }
}
