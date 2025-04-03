use anyhow::Result;

use crate::{lexer::LexerConsumer, util::LexerResult};

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

pub trait LexerUtils: LexerConsumer {
    fn check_next(&mut self, ch: char) -> bool;
    fn consume_if(&mut self, ch: impl CharMatcher) -> bool;
    fn read_while(&mut self, f: impl CharMatcher) -> Result<String>;
    fn consume_while(&mut self, f: impl CharMatcher) -> Result<usize>;
}

impl<T: LexerConsumer> LexerUtils for T {
    fn check_next(&mut self, ch: char) -> bool {
        matches!(self.consumer().next().0, LexerResult::Ok(c) if c == ch)
    }

    fn read_while(&mut self, cm: impl CharMatcher) -> Result<String> {
        let mut text = String::new();
        let mut consumer = self.consumer();

        loop {
            let (result, _) = consumer.next();
            match result {
                LexerResult::Ok(ch) if cm.match_char(ch) => text.push(ch),
                LexerResult::Err(err) => return Err(err),
                _ => return Ok(text),
            }
        }
    }

    fn consume_while(&mut self, cm: impl CharMatcher) -> Result<usize> {
        let mut count: usize = 0;

        let mut consumer = self.consumer();

        loop {
            let (result, _) = consumer.next();
            match result {
                LexerResult::Ok(ch) if cm.match_char(ch) => count += 1,
                LexerResult::Err(err) => return Err(err),
                _ => return Ok(count),
            }
        }
    }

    fn consume_if(&mut self, cm: impl CharMatcher) -> bool {
        let mut consumer = self.consumer();

        match consumer.next().0 {
            LexerResult::Ok(c) if cm.match_char(c) => {
                consumer.apply();
                true
            }
            _ => false,
        }
    }
}
