use std::fs;

use lexer::Lexer;
use util::END;

mod char_parsing;
mod lexer;
mod util;

fn main() {
    let mut lexer = Lexer::new(fs::File::open("local/invalid.txt").unwrap());

    loop {
        let ch = lexer.buffer_char();

        if let Ok(ch) = ch {
            print!("{}", ch);
        } else {
            let err = ch.unwrap_err();
            if !err.is::<END>() {
                println!("\n\nError: {:?}", err);
            }
            break;
        }
    }
}
