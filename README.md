# Lexer Package
This crate provides some methods for parsing text.

I mostly developed it for a custom language I want to build.

## Features
This crate exposes two [main structs](./src/text_parser.rs):
* **TextParser**: The base parser that loads the text from a `Read` trait.
* **Peeker**: Allows you to read ahead without consuming the previous characters until you consume all read characters.

## Usage

To use this package, add the following to your `Cargo.toml` file:
```toml
[dependencies]
lexer = "0.1.0"
```

or run `cargo add`

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