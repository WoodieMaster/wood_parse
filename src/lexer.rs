use std::{io::Read, thread::current};

use anyhow::Result;

use crate::char_parsing::read_char;

#[derive(Debug)]
pub struct Lexer<R: Read> {
    /// The position of the buffer
    read_pos: usize,
    /// The index of the current char in the buffer
    buf_pos: usize,
    buffer: Vec<char>,
    reader: R,
}

impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            read_pos: 0,
            buf_pos: 0,
            buffer: Vec::new(),
            reader,
        }
    }

    pub fn current_pos(&self) -> usize {
        self.read_pos + self.buf_pos
    }

    pub fn next(&mut self) -> Result<(char, usize)> {
        let idx = self.current_pos();
        let buf_pos = self.buf_pos;
        self.buf_pos += 1;

        if buf_pos < self.buffer.len() {
            return Ok((self.buffer[buf_pos], idx));
        }
        read_char(&mut self.reader).map(|ch| (ch, self.current_pos()))
    }
}
