use rs_parse_lib::{
    text_parser::{DeferedTextParserTrait, TextParser},
    text_parser_utils::LexerUtils,
    util::LexerResult,
};

fn main() {
    let input = "a   b   c";
    let expected = "abc";

    let mut lexer = TextParser::new(input.as_bytes());
    let mut consumer = lexer.consumer();

    let mut parsed_string = String::new();

    loop {
        // skip whitespace
        let _ = consumer.consume_while(|ch: char| ch.is_whitespace());

        let (result, _) = consumer.next();
        match result {
            LexerResult::Ok(ch) => parsed_string.push(ch),
            LexerResult::End => break,
            _ => {}
        }
    }
    assert!(
        parsed_string == expected,
        "Expected {expected}, got {parsed_string}"
    );
}
