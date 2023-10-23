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

    #[cfg(debug_assertions)]
    VomitDebug {
        file: String,
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

#[cfg(debug_assertions)]
fn do_cli(args: Subcommands) -> Result<(), Error> {
    use Subcommands::*;
    match args {
        Build {
            file,
            template,
            output,
        } => build(&file, &template, &output),
        VomitDebug { file, output } => vomit_debug(&file, &output),
    }
}

#[cfg(not(debug_assertions))]
fn do_cli(args: Subcommands) -> Result<(), Error> {
    match args {
        Build {
            file,
            template,
            output,
        } => build(&file, &template, &output),
    }
}

fn main() -> Result<(), Error> {
    let args = Fxg::parse().subcommand;
    do_cli(args)
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

#[cfg(debug_assertions)]
fn vomit_debug(file: &str, output: &str) -> Result<(), Error> {
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
