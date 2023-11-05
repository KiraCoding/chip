use crate::token::{Delimeter, Token};
use core::iter::Peekable;
use core::str::CharIndices;
use itertools::Itertools;

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
            .for_each(|_| {});
    }

    fn lex_token(&mut self, head: usize) -> Token<'l> {
        let tail = head
            + self
                .iter
                .by_ref()
                .peeking_take_while(|&(_, c)| c.is_ascii_alphanumeric())
                .count();

        dbg!(&self.input[head..tail]);

        match &self.input[head..tail] {
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
        // while let Some(&(pos, c)) = self.iter.peek() {
        //     match c {
        //         c if c.is_whitespace() => {
        //             self.iter.next();
        //         }
        //         ';' => self.skip_comment(),
        //         ',' => {
        //             self.iter.next();
        //             return Some(Token::Delimeter(Delimeter::Comma));
        //         }
        //         c if c.is_ascii_alphabetic() || c.is_ascii_digit() => {
        //             return Some(self.lex_token(pos))
        //         }
        //         _ => {
        //             self.iter.next();
        //             return Some(Token::Unknown(&self.input[pos..pos + 1]));
        //         }
        //     }
        // }

        // None

        self.iter
            .by_ref()
            .skip_while(|&(_, c)| c.is_whitespace())
            .next()
            .map(|(pos, c)| {
                if c == ';' {
                    self.skip_comment();
                }

                match c {
                    ',' => Token::Delimeter(Delimeter::Comma),
                    c if c.is_ascii_alphabetic() || c.is_ascii_digit() => self.lex_token(pos),
                    _ => Token::Unknown(&self.input[pos..pos + 1]),
                }
            })
    }
}

#[derive(Debug)]
pub enum LexerError {
    Invalid(),
}
