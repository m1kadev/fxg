use crate::{project::Project, Error};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

mod document;
mod header;
mod lexer;
mod nodes;

pub use document::Document;
pub use lexer::Lexer;

use self::header::Image;

#[derive(Serialize, Deserialize)]
pub struct PageInformation {
    title: String,
    tags: Vec<String>,
    image: Option<Image>,
}

#[derive(Serialize, Deserialize)]
pub struct SiteData {
    pages: Vec<PageInformation>,
    known_tags: Vec<String>,
}

fn copy_dir(src: PathBuf, dst: PathBuf) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn build(project: Project) -> Result<Project, Error> {
    let fxg_files = project.collect_documents("fxg")?;
    let html_files = project.collect_documents("html")?;
    let src = project.src_dir();
    let dest = project.dest_dir();
    let template = project.read_template()?;
    let mut known_tags = HashSet::new();
    let mut pages = vec![];

    for source in fxg_files {
        let relative = source.strip_prefix(&src)?;
        let mut destination = dest.join(relative);
        destination.set_extension("html");
        let document = build_file(source, &template, destination)?;
        let header = document.header();
        for tag in &header.tags {
            known_tags.insert(tag.to_string());
        }
        pages.push(PageInformation {
            title: header.title,
            tags: header.tags,
            image: header.ogp.image,
        });
    }

    for source in html_files {
        let relative = source.strip_prefix(&src)?;
        let destination = dest.join(relative);
        fs::copy(source, destination)?;
    }

    let mut data_path = project.dest_dir();
    data_path.push("fxg.json");
    let file = File::create(data_path)?;
    let site_data = SiteData {
        pages,
        known_tags: known_tags.into_iter().collect(),
    };
    serde_json::to_writer(file, &site_data)?;

    let static_folder = project.static_dir();
    let relative = static_folder.strip_prefix(&project.base_dir())?;
    let destination = dest.join(relative);
    copy_dir(static_folder, destination)?;

    Ok(project)
}

fn build_file(file: PathBuf, template: &str, output: PathBuf) -> Result<Document, Error> {
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

    Ok(document)
}

#[cfg(feature = "contributor")]
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
