use rs_parse_lib::{lexer::DefaultLexer, END};

fn main() {
    let args = std::env::args().nth(1).expect("Pass one argument");

    let mut lexer = DefaultLexer::new(args.as_bytes());

    for _ in 0..3 {
        let mut consumer = lexer.consumer();

        loop {
            let (ch, idx) = consumer.next();
            if let Ok(ch) = ch {
                print!("{}", ch);
            } else {
                let err = ch.unwrap_err();
                if !err.is::<END>() {
                    println!("\n\nError at {}: {:?}", idx + 1, err);
                }
                break;
            }
        }
        consumer.apply();
        println!();
    }
}
