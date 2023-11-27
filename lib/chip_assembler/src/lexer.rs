use crate::token::{Delimeter, Token};
use core::iter::Peekable;
use core::str::CharIndices;
use itertools::Itertools;

#[derive(Debug)]
pub struct Lexer<'l> {
    input: &'l str,
    iter: Peekable<CharIndices<'l>>,
}

impl<'c> From<&'c str> for Lexer<'c> {
    fn from(input: &'c str) -> Self {
        Self {
            input,
            iter: input.char_indices().peekable(),
        }
    }
}

impl<'l> Lexer<'l> {
    fn skip_comment(&mut self) {
        self.iter
            .by_ref()
            .peeking_take_while(|&(_, c)| c != '\n')
            .for_each(drop);
    }

    fn lex_token(&mut self, head: usize) -> Token<'l> {
        let tail = head
            + self
                .iter
                .by_ref()
                .peeking_take_while(|&(_, c)| c.is_ascii_alphanumeric())
                .count();

        dbg!(&self.input[head..=tail]);

        match &self.input[head..=tail] {
            s if s.starts_with("0x") => {
                let number = u16::from_str_radix(&s[2..], 16).ok();
                number.map_or(Token::Unknown(s), Token::Number)
            }
            s => Token::try_from(s).unwrap_or(Token::Unknown(s)),
        }
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token<'l>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .by_ref()
            .skip_while(|(_, c)| c.is_whitespace())
            .next()
            .and_then(|(pos, c)| match c {
                ';' => {
                    self.skip_comment();
                    self.next()
                }
                ',' => Some(Token::Delimeter(Delimeter::Comma)),
                c if c.is_ascii_alphabetic() || c.is_ascii_digit() => Some(self.lex_token(pos)),
                _ => Some(Token::Unknown(&self.input[pos..=pos])),
            })
    }
}
