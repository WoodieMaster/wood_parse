use anyhow::Result;

use crate::text_parser::PeekerTrait;

/// A trait that defines a method for matching against a single character
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

/// The trait defining useful methods for text pasing
/// The trait is automatically implemented for all types implementing its super trait
pub trait TextParserUtils: PeekerTrait {
    /// Checks if the next character matches the given character matcher
    fn check_next(&mut self, ch: impl CharMatcher) -> bool;
    /// Consumes the next character if it matches the given character matcher
    fn consume_if(&mut self, ch: impl CharMatcher) -> bool;
    /// Consumes characters while they match the given character matcher.
    /// returns the consumed text
    fn read_while(&mut self, f: impl CharMatcher) -> Result<String>;
    /// Consumes characters while they match the given character matcher
    /// returns the amount of characters consumed
    fn consume_while(&mut self, f: impl CharMatcher) -> Result<usize>;
}

impl<T: PeekerTrait> TextParserUtils for T {
    fn check_next(&mut self, cm: impl CharMatcher) -> bool {
        matches!(self.peeker().next(), Some(Ok(c)) if cm.match_char(c))
    }

    fn read_while(&mut self, cm: impl CharMatcher) -> Result<String> {
        let mut text = String::new();
        let mut peeker = self.peeker();

        loop {
            match peeker.next() {
                Some(Ok(ch)) if cm.match_char(ch) => text.push(ch),
                Some(Err(err)) => return Err(err),
                _ => {
                    peeker.back(1);
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
            match peeker.next() {
                Some(Ok(ch)) if cm.match_char(ch) => count += 1,
                Some(Err(err)) => return Err(err),
                _ => {
                    peeker.back(1);
                    peeker.apply();
                    return Ok(count);
                }
            }
        }
    }

    fn consume_if(&mut self, cm: impl CharMatcher) -> bool {
        let mut peeker = self.peeker();

        match peeker.next() {
            Some(Ok(c)) if cm.match_char(c) => {
                peeker.apply();
                true
            }
            _ => false,
        }
    }
}
