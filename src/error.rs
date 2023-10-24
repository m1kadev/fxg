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

#[inline]
fn row_and_col_from_index(string: &str, index: usize) -> (usize, usize) {
    let mut row = 0usize;
    let mut col = 0usize;
    let mut counter = 0;

    for char in string.chars() {
        counter += 1;
        if char == '\n' {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
        if counter == index {
            break;
        }
    }
    (row, col)
}

impl Error {
    fn display_parsing_error(&self) -> std::fmt::Result {
        if let Self::ParsingError {
            message,
            region,
            source,
        } = self
        {
            dbg!(region);
            let (row_begin, col_begin) = row_and_col_from_index(source, region.start);
            let (row_end, col_end) = row_and_col_from_index(source, region.end);

            eprintln!(
                "| [{}:{}] {} {}",
                row_begin + 1,
                col_begin,
                "Parsing error:".red(),
                message
            );
            eprintln!("| ");
            let mut lines = source.lines();
            let mut current_row = 0;
            let mut current_index = 0;
            loop {
                if current_row == row_begin {
                    break;
                }
                let line = lines.next().unwrap();
                current_index += line.len();
                current_row += 1;
            }
            eprint!("| {:1$} ", current_row + 1, row_end.to_string().len());
            for line in lines {
                current_index += line.len();
                eprintln!("{}", line);
                eprint!("| {:1$} ", "", row_end.to_string().len());
                for _ in 0..line.len() {
                    current_index += 1;
                    if region.contains(&current_index) {
                        eprint!("{}", "~".red());
                    } else {
                        eprint!(" ");
                    }
                }
                eprintln!();
                current_row += 1;
                if current_row > row_end {
                    break;
                }
            }
        } else {
            unreachable!();
        }
        Ok(())
    }
}
