use std::io::Read;

use crate::{char_parsing::read_char, util::TextParserResult};

pub type Pos = usize;

pub trait TextParserTrait {
    /// Get the current character position of the parser
    fn current_pos(&self) -> usize;
    /// Get the character at the given offset from the current position
    fn get(&mut self, offset: usize) -> (TextParserResult, Pos);
    /// Consume the given number of characters
    fn consume(&mut self, count: usize);
    /// Create a peeker that uses the current parser
    fn peeker(&mut self) -> impl DeferedTextParserTrait;
}

pub trait DeferedTextParserTrait: TextParserTrait {
    fn next(&mut self) -> (TextParserResult, Pos);
    fn reverse(&mut self, amount: usize);
    fn apply(self);
}

#[derive(Debug)]
pub struct TextParser<R: Read> {
    /// The position of the buffer
    read_pos: usize,
    /// The buffer containing all available characters
    buffer: Vec<char>,
    /// The reader the parser is reading from
    reader: R,
    /// Whether an error/eof has been encountered
    errored: bool,
}

impl<R: Read> TextParser<R> {
    /// Creates a new parser
    pub fn new(reader: R) -> Self {
        Self {
            read_pos: 0,
            buffer: Vec::new(),
            reader,
            errored: false,
        }
    }

    /// Returns a `Peeker` used for parsing ahead without affecting the parser
    pub fn peeker(&mut self) -> Peeker<Self> {
        Peeker::new(self)
    }
}

impl<R: Read> TextParserTrait for TextParser<R> {
    fn current_pos(&self) -> usize {
        self.read_pos - 1
    }

    fn get(&mut self, offset: usize) -> (TextParserResult, Pos) {
        while offset >= self.buffer.len() {
            self.buffer.push(match read_char(&mut self.reader) {
                TextParserResult::Ok(ok) => ok,
                TextParserResult::Err(err) => {
                    self.errored = true;
                    return (TextParserResult::Err(err), offset + self.read_pos);
                }
                TextParserResult::End => {
                    self.errored = true;
                    return (TextParserResult::End, offset + self.read_pos);
                }
            });
        }
        (
            TextParserResult::Ok(self.buffer[offset]),
            offset + self.read_pos,
        )
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
    fn peeker(&mut self) -> Peeker<Self> {
        Peeker::new(self)
    }
}

pub struct Peeker<'a, L: TextParserTrait> {
    parser: &'a mut L,
    idx: usize,
}

impl<'a, L: TextParserTrait> Peeker<'a, L> {
    pub fn new(parser: &'a mut L) -> Self {
        Self { parser, idx: 0 }
    }
}

impl<'a, L: TextParserTrait> TextParserTrait for Peeker<'a, L> {
    fn current_pos(&self) -> usize {
        self.parser.current_pos() + self.idx
    }

    fn get(&mut self, offset: usize) -> (TextParserResult, usize) {
        self.parser.get(self.idx + offset)
    }

    fn consume(&mut self, count: usize) {
        self.idx += count;
    }

    #[allow(refining_impl_trait)]
    fn peeker(&mut self) -> Peeker<Self> {
        Peeker::new(self)
    }
}

impl<'a, L: TextParserTrait> DeferedTextParserTrait for Peeker<'a, L> {
    fn next(&mut self) -> (TextParserResult, usize) {
        let (ch, idx) = self.parser.get(self.idx);
        self.idx += 1;
        (ch, idx)
    }

    fn apply(self) {
        self.parser.consume(self.idx);
    }

    fn reverse(&mut self, amount: usize) {
        self.idx -= amount;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_parser() {
        TextParser::new("Balls".as_bytes());
    }

    #[test]
    fn get_from_parser() {
        let mut parser = TextParser::new("Balls😊äüÀ".as_bytes());

        assert_eq!(parser.get(0).0.unwrap(), 'B');
        assert_eq!(parser.get(1).0.unwrap(), 'a');
        assert_eq!(parser.get(2).0.unwrap(), 'l');
        assert_eq!(parser.get(3).0.unwrap(), 'l');
        assert_eq!(parser.get(4).0.unwrap(), 's');
        assert_eq!(parser.get(5).0.unwrap(), '😊');
        assert_eq!(parser.get(6).0.unwrap(), 'ä');
        assert_eq!(parser.get(7).0.unwrap(), 'ü');
        assert_eq!(parser.get(8).0.unwrap(), 'À');
    }

    #[test]
    fn read_parser() {
        let read_string = "Balls😊äüÀ";
        let mut parser = TextParser::new(read_string.as_bytes());
        let mut peeker = parser.peeker();

        let mut expected_chars = read_string.chars();

        let mut expected_pos = 0;
        loop {
            let (result, pos) = peeker.next();

            assert_eq!(pos, expected_pos, "Expected pos {expected_pos}, got {pos}");

            match result {
                TextParserResult::Ok(ch) => {
                    let expected_char = expected_chars.next().expect("Expected fewer characters");

                    assert_eq!(
                        ch, expected_char,
                        "Expected char {ch}, got {expected_char} at {pos}"
                    );
                }
                TextParserResult::Err(err) => {
                    panic!("Expected no error, got {err}");
                }
                TextParserResult::End => break,
            }

            expected_pos += 1;
        }

        assert!(expected_chars.next().is_none(), "Expected more chars");
    }
}
