use std::io::Read;

use anyhow::Result;

use crate::char_parsing::read_char;

/// The trait for the text parser
///
/// Can be used to create a custom implementation of the TextParser struct
pub trait TextParserTrait {
    /// Get the current character position of the parser
    fn position(&self) -> usize;
    /// Get the character at the given offset from the current position
    fn get(&mut self, offset: usize) -> Option<Result<char>>;
    /// Consume the given number of characters
    fn consume(&mut self, count: usize);
    /// Create a peeker that uses the current parser
    fn peeker(&mut self) -> impl PeekerTrait;
}

/// The trait for the peeker
/// Can be used to create a custom implementation of the Peeker struct
pub trait PeekerTrait: TextParserTrait + Iterator<Item = Result<char>> {
    /// Lets you go back the given number of characters
    /// # Panics
    /// Panics if the amount is greater than the current peek position
    fn back(&mut self, amount: usize);
    /// Makes the underlying text parser, which may be another peeker, consume all the characters that this peeker peeked
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
    pub fn peeker<'a, P>(&'a mut self) -> P
    where
        P: PeekerTrait + From<&'a mut Self>,
    {
        P::from(self)
    }
}

impl<R: Read> TextParserTrait for TextParser<R> {
    fn position(&self) -> usize {
        self.read_pos - 1
    }

    fn get(&mut self, offset: usize) -> Option<Result<char>> {
        while offset >= self.buffer.len() {
            self.buffer.push(match read_char(&mut self.reader) {
                Some(Ok(ok)) => ok,
                Some(Err(err)) => {
                    self.errored = true;
                    return Some(Err(err));
                }
                None => {
                    self.errored = true;
                    return None;
                }
            });
        }
        Some(Ok(self.buffer[offset]))
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

pub struct Peeker<'a, T: TextParserTrait> {
    parser: &'a mut T,
    idx: usize,
}

impl<'a, T: TextParserTrait + 'a> From<&'a mut T> for Peeker<'a, T> {
    fn from(parser: &'a mut T) -> Self {
        Self { parser, idx: 0 }
    }
}

impl<'a, L: TextParserTrait> Peeker<'a, L> {
    pub fn new(parser: &'a mut L) -> Self {
        Self { parser, idx: 0 }
    }
}

impl<'a, L: TextParserTrait> TextParserTrait for Peeker<'a, L> {
    fn position(&self) -> usize {
        self.parser.position() + self.idx
    }

    fn get(&mut self, offset: usize) -> Option<Result<char>> {
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

impl<'a, T: TextParserTrait> Iterator for Peeker<'a, T> {
    type Item = Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.parser.get(self.idx);
        self.idx += 1;
        result
    }
}

impl<'a, T: TextParserTrait> PeekerTrait for Peeker<'a, T> {
    fn apply(self) {
        self.parser.consume(self.idx);
    }

    fn back(&mut self, amount: usize) {
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
        let expected = vec!['B', 'a', 'l', 'l', 's', '😊', 'ä', 'ü', 'À'];
        for i in 0..=8 {
            assert_eq!(parser.get(i).unwrap().unwrap(), expected[i]);
        }
    }

    #[test]
    fn read_parser() {
        let read_string = "Balls😊äüÀ";
        let mut parser = TextParser::new(read_string.as_bytes());
        let mut peeker: Peeker<_> = parser.peeker();

        let mut expected_chars = read_string.chars();
        loop {
            let result = peeker.next();

            match result {
                Some(Ok(ch)) => {
                    let expected_char = expected_chars.next().expect("Expected fewer characters");

                    assert_eq!(
                        ch,
                        expected_char,
                        "Expected char {ch}, got {expected_char} at {}",
                        peeker.position()
                    );
                }
                Some(Err(err)) => {
                    panic!("Expected no error, got {err}");
                }
                None => break,
            }
        }

        assert!(expected_chars.next().is_none(), "Expected more chars");
    }
}
