use crate::{project::Project, Error};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

mod document;
mod header;
mod lexer;
mod nodes;

pub use document::Document;
pub use lexer::Lexer;

pub fn build(project: Project) -> Result<(), Error> {
    let files = project.collect_documents()?;
    let src = project.src_dir();
    let dest = project.dest_dir();
    let template = project.read_template()?;

    for source in files {
        let relative = source.strip_prefix(&src)?;
        let destination = dest.join(relative);
        build_file(source, &template, destination)?;
    }
    Ok(())
}

fn build_file(file: PathBuf, template: &str, output: PathBuf) -> Result<(), Error> {
    let data = fs::read_to_string(file)?;
    let lexer = Lexer::lex(&data)?;
    let document = Document::build(lexer)?;

    if !template.contains("{{FXG_HEADER}}") {
        return Err(Error::Header("No header field found!".to_string()));
    }

    if !template.contains("{{FXG_OUTPUT}}") {
        return Err(Error::Header("No output field found!".to_string()));
    }

    let mut output_file = File::create(output)?;
    output_file.write_all(
        template
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
