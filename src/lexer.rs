use core::panic;
use std::io::Read;

use anyhow::Result;

use crate::char_parsing::read_char;

pub type LexerResult = (Result<char>, usize);

pub trait Lexer {
    /// Get the current character position of the lexer
    fn current_pos(&self) -> usize;
    /// Get the character at the given offset from the current position
    fn get(&mut self, offset: usize) -> LexerResult;
    /// Consume the given number of characters
    fn consume(&mut self, count: usize);
}

#[derive(Debug)]
pub struct DefaultLexer<R: Read> {
    /// The position of the buffer
    read_pos: usize,
    /// The buffer containing all available characters
    buffer: Vec<char>,
    /// The reader the lexer is reading from
    reader: R,
    /// Whether an error/eof has been encountered
    errored: bool,
}

impl<R: Read> DefaultLexer<R> {
    /// Creates a new lexer
    pub fn new(reader: R) -> Self {
        Self {
            read_pos: 0,
            buffer: Vec::new(),
            reader,
            errored: false,
        }
    }

    /// Returns a `LexerConsumer` used for parsing ahead without affecting the lexer
    pub fn consumer(&mut self) -> LexerConsumer<Self> {
        LexerConsumer::new(self)
    }
}

impl<R: Read> Lexer for DefaultLexer<R> {
    fn current_pos(&self) -> usize {
        self.read_pos - 1
    }

    fn get(&mut self, offset: usize) -> LexerResult {
        while offset >= self.buffer.len() {
            self.buffer.push(match read_char(&mut self.reader) {
                Ok(ok) => ok,
                Err(err) => {
                    self.errored = true;
                    return (Err(err), offset + self.read_pos);
                }
            });
        }
        return (Ok(self.buffer[offset]), offset + self.read_pos);
    }

    fn consume(&mut self, count: usize) {
        if count > self.buffer.len() {
            if !self.errored {
                panic!(
                    "Can't consume more than buffer size! Removing: {count}, length: {}",
                    self.buffer.len()
                );
            }
            self.buffer.clear();
            return;
        }
        self.buffer.drain(0..count);
    }
}

pub struct LexerConsumer<'a, L: Lexer> {
    lexer: &'a mut L,
    idx: usize,
}

impl<'a, L: Lexer> LexerConsumer<'a, L> {
    pub fn new(lexer: &'a mut L) -> Self {
        Self { lexer, idx: 0 }
    }

    pub fn next(&mut self) -> (Result<char>, usize) {
        let (ch, idx) = self.lexer.get(self.idx);
        self.idx += 1;
        (ch, idx)
    }

    pub fn consumer(&mut self) -> LexerConsumer<Self> {
        LexerConsumer::new(self)
    }

    pub fn apply(&mut self) {
        self.lexer.consume(self.idx);
    }
}

impl<'a, L: Lexer> Lexer for LexerConsumer<'a, L> {
    fn current_pos(&self) -> usize {
        self.lexer.current_pos() + self.idx
    }

    fn get(&mut self, offset: usize) -> LexerResult {
        self.lexer.get(self.idx + offset)
    }

    fn consume(&mut self, count: usize) {
        self.idx += count;
    }
}

#[cfg(test)]
mod test {
    use crate::END;

    use super::*;

    #[test]
    fn create_lexer() {
        DefaultLexer::new("Balls".as_bytes());
    }

    #[test]
    fn get_from_lexer() {
        let mut lexer = DefaultLexer::new("Balls😊äüÀ".as_bytes());

        assert_eq!(lexer.get(0).0.unwrap(), 'B');
        assert_eq!(lexer.get(1).0.unwrap(), 'a');
        assert_eq!(lexer.get(2).0.unwrap(), 'l');
        assert_eq!(lexer.get(3).0.unwrap(), 'l');
        assert_eq!(lexer.get(4).0.unwrap(), 's');
        assert_eq!(lexer.get(5).0.unwrap(), '😊');
        assert_eq!(lexer.get(6).0.unwrap(), 'ä');
        assert_eq!(lexer.get(7).0.unwrap(), 'ü');
        assert_eq!(lexer.get(8).0.unwrap(), 'À');
    }

    #[test]
    fn read_lexer() {
        let read_string = "Balls😊äüÀ";
        let mut lexer = DefaultLexer::new(read_string.as_bytes());
        let mut consumer = lexer.consumer();

        let mut expected_chars = read_string.chars();

        let mut expected_pos = 0;
        loop {
            let (result, pos) = consumer.next();

            assert_eq!(pos, expected_pos, "Expected pos {expected_pos}, got {pos}");

            match result {
                Ok(ch) => {
                    let expected_char = expected_chars.next().expect("Expected fewer characters");

                    assert_eq!(
                        ch, expected_char,
                        "Expected char {ch}, got {expected_char} at {pos}"
                    );
                }
                Err(err) => {
                    assert!(err.is::<END>());
                    break;
                }
            }

            expected_pos += 1;
        }

        assert!(expected_chars.next().is_none(), "Expected more chars");
    }
}
