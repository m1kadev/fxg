use std::{env, fs, process};

mod lexer;
mod ast;
mod trans;

fn nice_panic(message: String, code: i32) -> ! {
    eprintln!("fxg: error: {}", message);
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
        eprintln!("fxg: usage: fxg [file]");
        eprintln!("fxg: file may be - to read from stdin");
        process::exit(64)
    };
    
    let lex = lexer::lex(&source);
    dbg!("{:?}", &lex);
    let ast = ast::build_ast(lex, &source);
    dbg!("{:?}", &ast);
    trans::translate(ast, &source);

}
