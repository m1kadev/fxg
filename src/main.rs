use std::{
    collections::HashMap, env::args, fmt::Display, fs::File, io::BufReader, path::PathBuf,
    process::exit,
};

use owo_colors::OwoColorize;

mod parser;

#[cfg(test)]
#[path = "../test/mod.rs"]
mod test;

#[derive(Default, Debug)]
struct Args {
    file: PathBuf,
    flags: Vec<String>,
    options: HashMap<String, String>,
}

#[inline]
fn warn<T>(msg: &T)
where
    T: Display + ?Sized,
{
    eprintln!("fxg: {}: {}", "Warn".yellow().bold(), msg);
}

#[inline]
fn error<T>(msg: &T, code: i32) -> !
where
    T: Display + ?Sized,
{
    eprintln!("fxg: {}: {}", "Error".red().bold(), msg);
    exit(code)
}

fn parse_args() -> Args {
    let mut cli_args = args().skip(1);
    let mut args = Args::default();
    let input_file = match cli_args.next() {
        Some(file) => file,
        None => error("Input file not provided", 1),
    };
    args.file = PathBuf::from(input_file);
    for arg in cli_args {
        if arg.starts_with("--") {
            if let Some((key, value)) = arg.split_once("=") {
                args.options.insert(key[2..].to_string(), value.to_string());
            } else {
                warn(&format!(
                    "Argument {} was not able to be parsed, ignoring...",
                    arg
                ));
                continue;
            }
        } else if arg.starts_with("-") {
            args.flags.push(arg[1..].to_string());
        } else {
            warn(&format!(
                "Argument {} was not able to be parsed, ignoring...",
                arg
            ));

            continue;
        }
    }
    return args;
}

fn main() {
    let args = parse_args();
    let source_file = match File::open(args.file) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{}: Input file wasn't able to be opened ({})",
                "Error".bold().red(),
                e.black()
            );
            exit(2);
        }
    };
    let mut reader = BufReader::new(source_file);
    let output = crate::parser::parse(&mut reader);
    print!("{output}");
}
