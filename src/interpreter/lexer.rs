use std::slice::Iter;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Parenthesis(char),
    Symbol(String),
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
        
        while let Some(&&ch) = self.code.peek() {
            match ch {
                ch if ch.is_ascii_digit() => {
                    let mut number = String::new();

                    while let Some(&&ch) = self.code.peek() {
                        if ch.is_ascii_digit() || ch == '.' {
                            self.code.next();
                            number.push(ch);
                        } else {
                            break;
                        }
                    }

                    if let Ok(number) = number.parse::<f64>() {
                        return Some(Token::Number(number));
                    } else {
                        panic!(format!("invalid number: {}", number));
                    }
                },
                '(' | ')' => {
                    self.code.next();
                    return Some(Token::Parenthesis(ch));
                },
                _ => {
                    let mut symbol = String::new();

                    while let Some(&ch) = self.code.next() {
                        if ch.is_ascii_whitespace() {
                            break;
                        } else {
                            symbol.push(ch);
                        }
                    }

                    if symbol.len() == 0 {
                        continue;
                    }

                    return Some(Token::Symbol(symbol));
                },
            }
        }

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
            Token::Symbol("+".to_string()),
            Token::Number(2.0),
            Token::Number(74.95),
            Token::Parenthesis(')'),
        ]);
    }

    #[test]
    fn test_missing_space_merges_number_to_symbol() {
        lex_and_assert_eq!("(+2 4)", vec![
            Token::Parenthesis('('),
            Token::Symbol("+2".to_string()),
            Token::Number(4.0),
            Token::Parenthesis(')'),
        ]);
    }

    #[test]
    fn test_all_valid_symbols() {
        lex_and_assert_eq!("+ - * / %", vec![
            Token::Symbol("+".to_string()),
            Token::Symbol("-".to_string()),
            Token::Symbol("*".to_string()),
            Token::Symbol("/".to_string()),
            Token::Symbol("%".to_string()),
        ]);
    }

    #[test]
    #[should_panic(expected = "invalid number: 98.23.35")]
    fn test_invalid_number() {
        lex_and_assert_eq!("98.23.35", Vec::<_>::new());
    }
}
