// fuck it
#![feature(str_split_remainder)]
#![feature(const_mut_refs)]

use clap::{Parser, Subcommand};

mod compiler;
mod error;

use compiler::build;
use error::Error;

#[cfg(debug_assertions)]
use compiler::vomit_debug;

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

fn do_cli(args: Subcommands) -> Result<(), Error> {
    use Subcommands::*;
    match &args {
        Build {
            file,
            template,
            output,
        } => build(file, template, output),
        VomitDebug { file, output } => vomit_debug(file, output),
    }
}

fn main() -> Result<(), Error> {
    let args = Fxg::parse().subcommand;
    do_cli(args)
}
