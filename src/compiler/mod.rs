use crate::Error;
use std::{
    fs::{self, File},
    io::Write,
};

mod document;
mod header;
mod lexer;
mod nodes;

pub use document::Document;
pub use lexer::Lexer;

pub fn build(file: &str, template: &str, output: &str) -> Result<(), Error> {
    let data = fs::read_to_string(file)?;
    let lexer = Lexer::lex(&data)?;
    let document = Document::build(lexer)?;

    let final_output = fs::read_to_string(template)?;

    if !final_output.contains("{{FXG_HEADER}}") {
        return Err(Error::Header("No header field found!".to_string()));
    }

    if !final_output.contains("{{FXG_OUTPUT}}") {
        return Err(Error::Header("No output field found!".to_string()));
    }

    let mut output_file = File::create(output)?;
    output_file.write_all(
        final_output
            .replace("{{FXG_OUTPUT}}", &document.as_html())
            .replace("{{FXG_HEADER}}", &document.header_html())
            .as_bytes(),
    )?;

    Ok(())
}

#[cfg(debug_assertions)]
pub fn vomit_debug(file: &str, output: &str) -> Result<(), Error> {
    let data = fs::read_to_string(file)?;
    let mut lexer = Lexer::lex(&data)?;
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
