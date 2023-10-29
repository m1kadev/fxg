// fuck it
#![feature(str_split_remainder)]
#![feature(const_mut_refs)]
#![feature(iter_advance_by)]
#![feature(step_trait)]

use std::{env::current_dir, path::PathBuf, process::exit, time::Instant};

use clap::{Parser, Subcommand};

mod compiler;
mod error;
mod project;

use colored::Colorize;
use compiler::build;
use error::Error;

use crate::project::Project;

#[derive(Parser)]
pub struct Fxg {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Build {
        folder: Option<PathBuf>,
    },

    #[cfg(feature = "developer")]
    New {
        folder: String,
    },

    #[cfg(feature = "contributor")]
    VomitDebug {
        file: String,
        #[arg(short, long)]
        output: String,
    },
}

fn do_cli(args: Subcommands) -> Result<(), Error> {
    use Subcommands::*;
    match args {
        Build { folder } => {
            let path = folder.unwrap_or(current_dir()?);
            println!(
                "{} project {} ({})",
                "Building".bold().green(),
                &path
                    .iter()
                    .last()
                    .ok_or(Error::PathDisplayError)?
                    .to_str()
                    .ok_or(Error::PathDisplayError)?,
                &path.as_os_str().to_str().ok_or(Error::PathDisplayError)?
            );
            let begin = Instant::now();
            build(Project::from_dir(path)?)?;
            let end = Instant::now();
            let diff = end - begin;
            println!("{} in {}s", "Done".bold().green(), diff.as_secs());
            Ok(())
        }

        #[cfg(feature = "developer")]
        New { folder } => {
            let mut path = current_dir()?;
            path.push(&folder);
            project::new(path)?;
            println!("{} new project ({})", "Created".bold().green(), folder);
            Ok(())
        }

        #[cfg(feature = "contributor")]
        VomitDebug { file, output } => compiler::vomit_debug(&file, &output),
    }
}

fn main() {
    let args = Fxg::parse().subcommand;
    if let Err(e) = do_cli(args) {
        println!("{}", e);
        exit(-1);
    }
}
