use std::fmt::{Debug, Display};

use anyhow::Result;

#[derive(Debug)]
pub enum TextParserResult<T = char> {
    Ok(T),
    Err(anyhow::Error),
    End,
}

impl<V> From<Result<V>> for TextParserResult<V> {
    fn from(value: Result<V>) -> Self {
        match value {
            Ok(v) => TextParserResult::Ok(v),
            Err(e) => TextParserResult::Err(e),
        }
    }
}

impl<T> TextParserResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            TextParserResult::Ok(v) => v,
            TextParserResult::Err(e) => panic!("{}", e),
            TextParserResult::End => panic!("EOF"),
        }
    }

    pub fn is_end(&self) -> bool {
        matches!(self, TextParserResult::End)
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, TextParserResult::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, TextParserResult::Err(_))
    }
}

impl<T: Display> TextParserResult<T> {
    pub fn unwrap_err(self) -> anyhow::Error {
        match self {
            TextParserResult::Ok(v) => panic!("Expected error, got {}", v),
            TextParserResult::Err(e) => e,
            TextParserResult::End => panic!("EOF"),
        }
    }
}
