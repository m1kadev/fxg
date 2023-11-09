// fuck it
#![feature(str_split_remainder)]
#![feature(const_mut_refs)]

use std::{env::current_dir, fs::File, io::Write, path::PathBuf, process::exit};

use clap::{Parser, Subcommand};

mod compiler;
mod error;
mod project;
#[cfg(feature = "developer")]
mod server;

use colored::Colorize;
use compiler::build;
use error::Error;

use crate::{compiler::DocumentHeader, project::Project};
#[cfg(feature = "developer")]
use crate::project::TEMPLATE_FXG;

#[derive(Parser)]
pub struct Fxg {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Build {
        folder: Option<PathBuf>,
        #[cfg(feature = "developer")]
        #[arg(short, long)]
        start: bool,
    },

    #[cfg(feature = "developer")]
    New { folder: String },

    #[cfg(feature = "developer")]
    GetTheme { url: String, path: Option<PathBuf> },

    #[cfg(feature = "developer")]
    Page { name: String, path: Option<PathBuf> },

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
        Build {
            folder,
            #[cfg(feature = "developer")]
            start,
        } => {
            let path = folder.unwrap_or(current_dir()?);
            let project = build(Project::from_dir(path.clone())?)?;
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

            #[cfg(feature = "developer")]
            if start {
                server::start_server(project.dest_dir())?;
            }
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
        GetTheme { url, path } => {
            println!("gettheme");
            let regex =
                regex::Regex::new(r"(?m)https://gist\.github\.com/([a-zA-Z\-]+)/([0-9a-f]+)/?")?;
            let theme_html = if let Some(captures) = regex.captures(&url) {
                let user = &captures[1];
                let id = &captures[2];
                server::download_file(format!(
                    "https://gist.githubusercontent.com/{user}/{id}/raw"
                ))
            } else {
                server::download_file(url)
            }?;
            let folder = path.unwrap_or(current_dir()?);
            let project = Project::from_dir(folder.clone())?;
            let theme_file = project.template();
            let mut file = File::create(theme_file)?;
            file.write_all(&theme_html)?;

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
