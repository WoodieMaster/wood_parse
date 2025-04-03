use std::fmt::{Debug, Display};

use anyhow::Result;

#[macro_export]
macro_rules! tee {
    ($e:expr) => {{
        let e = $e;
        dbg!(&e);
        e
    }};
}

#[derive(Debug)]
pub enum LexerResult<T = char> {
    Ok(T),
    Err(anyhow::Error),
    End,
}

impl<V> From<Result<V>> for LexerResult<V> {
    fn from(value: Result<V>) -> Self {
        match value {
            Ok(v) => LexerResult::Ok(v),
            Err(e) => LexerResult::Err(e),
        }
    }
}

impl<T> LexerResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            LexerResult::Ok(v) => v,
            LexerResult::Err(e) => panic!("{}", e),
            LexerResult::End => panic!("EOF"),
        }
    }

    pub fn is_end(&self) -> bool {
        matches!(self, LexerResult::End)
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, LexerResult::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, LexerResult::Err(_))
    }
}

impl<T: Display> LexerResult<T> {
    pub fn unwrap_err(self) -> anyhow::Error {
        match self {
            LexerResult::Ok(v) => panic!("Expected error, got {}", v),
            LexerResult::Err(e) => e,
            LexerResult::End => panic!("EOF"),
        }
    }
}
