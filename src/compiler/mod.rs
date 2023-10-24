use crate::Error;
use std::{
    fs::{self, File},
    io::Write,
};

mod document;
mod document_nodes;
mod lexer;

pub use document::Document;
pub use lexer::Lexer;

pub fn build(file: &str, template: &str, output: &str) -> Result<(), Error> {
    let data = fs::read_to_string(file)?;
    let mut lexer = Lexer::lex(&data);
    let document = Document::build(&mut lexer)?;

    let final_output = fs::read_to_string(template)?;
    if !final_output.contains("{{FGX_OUTPUT}}") {
        panic!();
    }

    let mut output_file = File::create(output)?;
    output_file.write_all(
        final_output
            .replace("{{FGX_OUTPUT}}", &document.as_html())
            .as_bytes(),
    )?;

    Ok(())
}

#[cfg(debug_assertions)]
pub fn vomit_debug(file: &str, output: &str) -> Result<(), Error> {
    let data = fs::read_to_string(file)?;
    let mut lexer = Lexer::lex(&data);
    let mut output = File::create(output)?;
    while let Some(lexeme) = lexer.next() {
        writeln!(
            output,
            "{} - {:?} {:?}",
            lexer.slice(),
            lexeme,
            lexer.span()
        )?;
    }
    Ok(())
}
