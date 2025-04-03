# Lexer Package

A Rust library for building lexers.

## Overview

This package provides a set of tools for building lexers, including a [Lexer](cci:2://file:///c:/Users/table/Documents/Code/rs_parse_lib/src/lexer.rs:6:0-15:1) trait, a [DefaultLexer](cci:2://file:///c:/Users/table/Documents/Code/rs_parse_lib/src/lexer.rs:23:0-32:1) implementation, and various utility functions for working with lexers.

## Features

* **Lexer trait**: Defines the interface for a lexer, including methods for consuming input and producing tokens.
* **DefaultLexer**: A basic implementation of the [Lexer](cci:2://file:///c:/Users/table/Documents/Code/rs_parse_lib/src/lexer.rs:6:0-15:1) trait that can be used as a starting point for building custom lexers.
* **Utility functions**: Various functions for working with lexers, including [consume_while](cci:1://file:///c:/Users/table/Documents/Code/rs_parse_lib/src/lexer_utils.rs:52:4-65:5), [consume_if](cci:1://file:///c:/Users/table/Documents/Code/rs_parse_lib/src/lexer_utils.rs:67:4-77:5), and [read_while](cci:1://file:///c:/Users/table/Documents/Code/rs_parse_lib/src/lexer_utils.rs:38:4-50:5).

## Usage

To use this package, add the following to your `Cargo.toml` file:
```toml
[dependencies]
lexer = "0.1.0"
```

Then, import the package in your Rust code:
```rust
use lexer::{Lexer, DefaultLexer};
```

## Example
Here is an example of using the Default implementations to tokenize a string:
```rust
let input = "a   b   c";
let expected = "abc";

// create the parser and peeker
let mut parser = TextParser::new(input.as_bytes());
let mut peeker = parser.peeker();

let mut parsed_string = String::new();

// Loop until the end is hit or an error occurs
loop {
    // skip whitespace
    let _ = peeker.consume_while(|ch: char| ch.is_whitespace());

    // get the next character
    let (result, _) = peeker.next();
    match result {
        TextParserResult::Ok(ch) => parsed_string.push(ch),
        TextParserResult::End => break,
        _ => {}
    }
}

//compare results
assert!(
    parsed_string == expected,
    "Expected {expected}, got {parsed_string}"
);
```