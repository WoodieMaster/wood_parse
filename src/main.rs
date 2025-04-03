use rs_parse_lib::{
    lexer::{DefaultLexer, LexerConsumer},
    util::LexerResult,
};

fn main() {
    let args = std::env::args().nth(1).expect("Pass one argument");

    let mut lexer = DefaultLexer::new(args.as_bytes());

    for _ in 0..3 {
        let mut consumer = lexer.consumer();

        loop {
            let (result, _) = consumer.next();
            match result {
                LexerResult::Ok(ch) => print!("{ch}"),
                LexerResult::Err(err) => {
                    println!("\n\nError: {err:?}");
                    break;
                }
                LexerResult::End => break,
            }
        }
        consumer.apply();
        println!();
    }
}
