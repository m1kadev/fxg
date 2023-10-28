use std::{
    fmt::{Debug, Display},
    io,
    ops::Range,
    path::StripPrefixError,
};

use colored::Colorize;

macro_rules! map_error {
    ($error:ty => $enum_variant:ident) => {
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Self::$enum_variant(value)
            }
        }
    };

    ($($error:ty => $enum_variant:ident,)+) => {
        $(
            map_error!($error => $enum_variant);
        )+
    };
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Yaml(serde_yaml::Error),
    Header(String),
    StripPrefix(StripPrefixError),
    Parsing {
        message: String,
        region: Range<usize>,
        source: String,
    },
}

map_error! {
    io::Error => Io,
    serde_yaml::Error => Yaml,
    StripPrefixError => StripPrefix,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(..) | Self::Yaml(..) | Self::StripPrefix(..) => write!(f, "{:?}", self),
            Self::Parsing { .. } => self.display_parsing_error(),
            Self::Header(..) => self.display_header_error(),
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
        if let Self::Parsing {
            message,
            region,
            source,
        } = self
        {
            let (row_begin, col_begin) = row_and_col_from_index(source, region.start);
            let (row_end, mut col_end) = row_and_col_from_index(source, region.end);
            let lines = source.lines().collect::<Vec<_>>();
            if row_begin != row_end {
                // collapse the error into 1 line
                col_end = lines[row_begin].len();
            }

            eprintln!(
                "| [{}:{}] {} {}",
                row_begin + 1,
                col_begin,
                "Parsing error:".red(),
                message
            );
            eprintln!("| ");
            let line = lines[row_begin];
            eprintln!(
                "| {} {}{}{}",
                row_begin + 1,
                &line[..col_begin],
                &line[col_begin..col_end].red(),
                &line[col_end..]
            );
            eprintln!(
                "| {} {}{}",
                " ".repeat(line[..col_begin].len() + (row_begin + 1).to_string().len()),
                "~".repeat(line[col_begin..col_end].len()).red(),
                " ".repeat(line[col_end..].len())
            );
        } else {
            unreachable!();
        }
        Ok(())
    }

    fn display_header_error(&self) -> std::fmt::Result {
        if let Self::Header(msg) = self {
            eprintln!("{}{}", "Header error:".red(), msg);
        } else {
            unreachable!();
        }
        Ok(())
    }
}
