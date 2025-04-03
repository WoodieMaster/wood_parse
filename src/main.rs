use rs_parse_lib::{
    text_parser::{DeferedTextParserTrait, TextParser},
    text_parser_utils::TextParserUtils,
    util::TextParserResult,
};

fn main() {
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
}
