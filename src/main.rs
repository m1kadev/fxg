use std::{env, fs, process};

mod lexer;

fn nice_panic(message: String, code: i32) -> ! {
    eprintln!("ERROR: {}", message);
    process::exit(code)
}

fn main() {
    let mut args = env::args();
    let source = if let Some(file) = args.nth(1) {
        match fs::read_to_string(file) {
            Ok(v) => v,
            Err(e) => nice_panic(format!("Unexpected situation while reading input file! ({e})"), 66) // see sysexits.h(3)
        }
    } else {
        panic!("TODO: reading from stdin");
    };
    
    let lex = lexer::lex(&source);

    for (lexeme, range) in lex {
        eprintln!("[{:?}. {:?}] {}", lexeme, range.clone(), &source[range]);
    }
}
