use std::slice::Iter;
use std::iter::Peekable;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Parenthesis(char),
    Symbol(&'static str),
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

    macro_rules! lex_and_assert_eq {
        ($string_slice:literal, $expected:expr) => {
            let code = $string_slice.chars().collect::<Vec<char>>();
            let lexer = Lexer::new(code.iter());
            let result = lexer.collect::<Vec<Token>>();
            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn test_empty_code() {
        lex_and_assert_eq!("", Vec::<Token>::new());
    }

    #[test]
    fn test_basic_expr() {
        lex_and_assert_eq!("(+ 2 74.95)", vec![
            Token::Parenthesis('('),
            Token::Symbol("+"),
            Token::Number(2.0),
            Token::Number(74.95),
            Token::Parenthesis(')'),
        ]);
    }

    #[test]
    fn test_all_valid_symbols() {
        lex_and_assert_eq!("+ - * / %", vec![
            Token::Symbol("+"),
            Token::Symbol("-"),
            Token::Symbol("*"),
            Token::Symbol("/"),
            Token::Symbol("%"),
        ]);
    }

    #[test]
    #[should_panic(expected = "unrecognized symbol")]
    fn test_illegal_character() {
        lex_and_assert_eq!("(+2 3)", Vec::<_>::new());
    }

    #[test]
    #[should_panic(expected = "invalid number")]
    fn test_invalid_number() {
        lex_and_assert_eq!("98.23.35", Vec::<_>::new());
    }
}
