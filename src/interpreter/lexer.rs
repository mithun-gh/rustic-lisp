use std::slice::Iter;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Symbol(&'static str),
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
        lex_and_assert_eq!("(+ 2 4)", vec![
            Token::Parenthesis('('),
            Token::Symbol("+"),
            Token::Number(2.0),
            Token::Number(4.0),
            Token::Parenthesis(')'),
        ]);
    }
}
