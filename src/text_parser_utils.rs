use anyhow::Result;

use crate::{text_parser::DeferedTextParserTrait, util::TextParserResult};

pub trait CharMatcher {
    fn match_char(&self, ch: char) -> bool;
}

impl CharMatcher for char {
    fn match_char(&self, ch: char) -> bool {
        *self == ch
    }
}

impl<T: Fn(char) -> bool> CharMatcher for T {
    fn match_char(&self, ch: char) -> bool {
        self(ch)
    }
}

impl<'a> CharMatcher for &'a str {
    fn match_char(&self, ch: char) -> bool {
        self.contains(ch)
    }
}

pub trait TextParserUtils: DeferedTextParserTrait {
    fn check_next(&mut self, ch: impl CharMatcher) -> bool;
    fn consume_if(&mut self, ch: impl CharMatcher) -> bool;
    fn read_while(&mut self, f: impl CharMatcher) -> Result<String>;
    fn consume_while(&mut self, f: impl CharMatcher) -> Result<usize>;
}

impl<T: DeferedTextParserTrait> TextParserUtils for T {
    fn check_next(&mut self, cm: impl CharMatcher) -> bool {
        matches!(self.peeker().next().0, TextParserResult::Ok(c) if cm.match_char(c))
    }

    fn read_while(&mut self, cm: impl CharMatcher) -> Result<String> {
        let mut text = String::new();
        let mut peeker = self.peeker();

        loop {
            let (result, _) = peeker.next();
            match result {
                TextParserResult::Ok(ch) if cm.match_char(ch) => text.push(ch),
                TextParserResult::Err(err) => return Err(err),
                _ => {
                    peeker.reverse(1);
                    peeker.apply();
                    return Ok(text);
                }
            }
        }
    }

    fn consume_while(&mut self, cm: impl CharMatcher) -> Result<usize> {
        let mut count: usize = 0;

        let mut peeker = self.peeker();

        loop {
            let (result, _) = peeker.next();
            match result {
                TextParserResult::Ok(ch) if cm.match_char(ch) => count += 1,
                TextParserResult::Err(err) => return Err(err),
                _ => {
                    peeker.reverse(1);
                    peeker.apply();
                    return Ok(count);
                }
            }
        }
    }

    fn consume_if(&mut self, cm: impl CharMatcher) -> bool {
        let mut peeker = self.peeker();

        match peeker.next().0 {
            TextParserResult::Ok(c) if cm.match_char(c) => {
                peeker.apply();
                true
            }
            _ => {
                peeker.reverse(1);
                false
            }
        }
    }
}
