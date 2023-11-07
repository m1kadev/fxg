use crate::{project::Project, Error};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    time::{Duration, Instant},
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

#[inline]
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

#[inline]
fn draw_progress_bar(progress: f32, label: &str) -> String {
    let building_label = "Building ".blue().bold();
    let mut progress_str = String::from("[");
    let equalses = ((progress * 10f32).round() - 1f32) as usize;
    progress_str.push_str("=".repeat(equalses).as_str());
    progress_str.push('>');
    let filler = 10 - equalses;
    progress_str.push_str(" ".repeat(filler).as_str());
    progress_str.push(']');
    format!("{}{} ({})", building_label, progress_str, label)
}

#[inline]
fn display_date(date: &Duration) -> String {
    let seconds = date.as_secs();
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{minutes}m{seconds}s")
}

pub fn build(project: Project) -> Result<Project, Error> {
    let fxg_files = project.collect_documents("fxg")?;
    let html_files = project.collect_documents("html")?;
    let src = project.src_dir();
    let dest = project.dest_dir();
    let template = project.read_template()?;
    let mut known_tags = HashSet::new();
    let mut pages = vec![];
    let begin = Instant::now();

    let mut progress = 0f32;
    let out_of = (fxg_files.len() + html_files.len()) as f32;
    println!();

    for source in fxg_files {
        progress += 1f32;
        let path = source.as_os_str().to_string_lossy();
        let label = &path[path.len() - 21..];
        print!("\r{}", draw_progress_bar(progress / out_of, label));
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
        progress += 1f32;
        let path = source.as_os_str().to_string_lossy();
        let label = &path[path.len() - 21..];
        print!("\r{}", draw_progress_bar(progress / out_of, label));
        let relative = source.strip_prefix(&src)?;
        let destination = dest.join(relative);
        fs::copy(source, destination)?;
    }
    println!();
    println!("{} metadata", "Bundling".blue().bold());

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
    let end = Instant::now();
    let delta_t = end - begin;

    println!("{} in {}", "Compiled".green().bold(), display_date(&delta_t));

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
