use std::{
    fmt::{Debug, Display},
    io,
    ops::Range,
};

use colored::Colorize;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParsingError {
        message: String,
        region: Range<usize>,
        source: String,
    },
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(..) => write!(f, "{:?}", self),
            Self::ParsingError { .. } => self.display_parsing_error(),
        }
    }
}

// this is a bit slow
#[inline]
fn slice_contains<T: std::iter::Step>(a: &Range<T>, b: &Range<T>) -> bool {
    for i in a.clone() {
        if b.contains(&i) {
            return true;
        }
    }
    false
}

impl Error {
    fn display_parsing_error(&self) -> std::fmt::Result {
        if let Self::ParsingError {
            message,
            region,
            source,
        } = self
        {
            println!("| {} {}", "Parsing error:".red(), message);
            println!("| ",);
            let mut slice: Range<usize> = 0..0;
            for (line_number, line_text) in source.lines().enumerate() {
                slice.end += line_text.len();

                if slice_contains(region, &slice) {
                    println!("| {line_number} {line_text}");
                    print!("{}", " ".repeat(format!("| {} ", line_number).len()));
                    for character in slice.clone() {
                        if region.contains(&character) {
                            print!("{}", "~".red())
                        }
                    }
                    println!();
                }

                slice.start += line_text.len();
            }
        } else {
            unreachable!();
        }
        Ok(())
    }
}
