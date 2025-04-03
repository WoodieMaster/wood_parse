use std::io::Read;

use crate::{char_parsing::read_char, util::LexerResult};

pub type Pos = usize;

pub trait Lexer {
    /// Get the current character position of the lexer
    fn current_pos(&self) -> usize;
    /// Get the character at the given offset from the current position
    fn get(&mut self, offset: usize) -> (LexerResult, Pos);
    /// Consume the given number of characters
    fn consume(&mut self, count: usize);
    /// Create a consumer that uses the current lexer
    fn consumer(&mut self) -> impl LexerConsumer;
}

pub trait LexerConsumer: Lexer {
    fn next(&mut self) -> (LexerResult, Pos);
    fn apply(&mut self);
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
    pub fn consumer(&mut self) -> DefaultLexerConsumer<Self> {
        DefaultLexerConsumer::new(self)
    }
}

impl<R: Read> Lexer for DefaultLexer<R> {
    fn current_pos(&self) -> usize {
        self.read_pos - 1
    }

    fn get(&mut self, offset: usize) -> (LexerResult, Pos) {
        while offset >= self.buffer.len() {
            self.buffer.push(match read_char(&mut self.reader) {
                LexerResult::Ok(ok) => ok,
                LexerResult::Err(err) => {
                    self.errored = true;
                    return (LexerResult::Err(err), offset + self.read_pos);
                }
                LexerResult::End => {
                    self.errored = true;
                    return (LexerResult::End, offset + self.read_pos);
                }
            });
        }
        (LexerResult::Ok(self.buffer[offset]), offset + self.read_pos)
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

    #[allow(refining_impl_trait)]
    fn consumer(&mut self) -> DefaultLexerConsumer<Self> {
        DefaultLexerConsumer::new(self)
    }
}

pub struct DefaultLexerConsumer<'a, L: Lexer> {
    lexer: &'a mut L,
    idx: usize,
}

impl<'a, L: Lexer> DefaultLexerConsumer<'a, L> {
    pub fn new(lexer: &'a mut L) -> Self {
        Self { lexer, idx: 0 }
    }
}

impl<'a, L: Lexer> Lexer for DefaultLexerConsumer<'a, L> {
    fn current_pos(&self) -> usize {
        self.lexer.current_pos() + self.idx
    }

    fn get(&mut self, offset: usize) -> (LexerResult, usize) {
        self.lexer.get(self.idx + offset)
    }

    fn consume(&mut self, count: usize) {
        self.idx += count;
    }

    #[allow(refining_impl_trait)]
    fn consumer(&mut self) -> DefaultLexerConsumer<Self> {
        DefaultLexerConsumer::new(self)
    }
}

impl<'a, L: Lexer> LexerConsumer for DefaultLexerConsumer<'a, L> {
    fn next(&mut self) -> (LexerResult, usize) {
        let (ch, idx) = self.lexer.get(self.idx);
        self.idx += 1;
        (ch, idx)
    }

    fn apply(&mut self) {
        self.lexer.consume(self.idx);
    }
}

#[cfg(test)]
mod test {
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
                LexerResult::Ok(ch) => {
                    let expected_char = expected_chars.next().expect("Expected fewer characters");

                    assert_eq!(
                        ch, expected_char,
                        "Expected char {ch}, got {expected_char} at {pos}"
                    );
                }
                LexerResult::Err(err) => {
                    panic!("Expected no error, got {err}");
                }
                LexerResult::End => break,
            }

            expected_pos += 1;
        }

        assert!(expected_chars.next().is_none(), "Expected more chars");
    }
}
