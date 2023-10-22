#![feature(str_split_remainder)] // fuck it

use std::{
    fs::{self, File},
    io::{self, Write},
};

use clap::{Parser, Subcommand};
use document::Document;
use lexer::Lexer;

mod document;
mod lexer;

#[derive(Parser)]
pub struct Fxg {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Build {
        file: String,

        #[arg(short, long)]
        template: String,

        #[arg(short, long)]
        output: String,
    },
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    FxgError(String),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

fn main() -> Result<(), Error> {
    let args = Fxg::parse().subcommand;

    use Subcommands::*;
    match args {
        Build {
            file,
            template,
            output,
        } => build(&file, &template, &output),
    }
}

fn build(file: &str, template: &str, output: &str) -> Result<(), Error> {
    let data = fs::read_to_string(file)?;
    let mut lexer = Lexer::lex(&data);
    let document = Document::build(&mut lexer);
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
