use std::{env::current_dir, fs::File, io::Write, path::PathBuf, process::exit};

use clap::{Parser, Subcommand};

mod compiler;
mod error;
mod project;

use colored::Colorize;
use compiler::build;
use error::Error;

use crate::compiler::DocumentHeader;
use crate::project::{Project, TEMPLATE_FXG};

#[derive(Parser)]
pub struct Fxg {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Build {
        folder: Option<PathBuf>,
    },

    #[cfg(feature = "developer")]
    New {
        folder: String,
    },

    #[cfg(feature = "developer")]
    Page {
        name: String,
        path: Option<PathBuf>,
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
            let _ = build(Project::from_dir(path.clone())?)?;
            println!(
                "{} project {} ({})",
                "Building".bold().green(),
                &path
                    .iter()
                    .last()
                    .ok_or(Error::PathDisplay)?
                    .to_str()
                    .ok_or(Error::PathDisplay)?,
                &path.as_os_str().to_str().ok_or(Error::PathDisplay)?
            );
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

        #[cfg(feature = "developer")]
        Page { name, path } => {
            let folder = path.unwrap_or(current_dir()?);
            let project = Project::from_dir(folder.clone())?;
            let mut dest_page = project.src_dir();
            dest_page.push(&name);
            let header = DocumentHeader {
                title: name,
                ..Default::default()
            };
            dest_page.set_extension("fxg");
            let mut dest_page_f = File::create(dest_page)?;
            dest_page_f.write_all(
                TEMPLATE_FXG
                    .replace("{{HEADER}}", &serde_yaml::to_string(&header)?)
                    .as_bytes(),
            )?;

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
