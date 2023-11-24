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

    (#[developer] $error:ty => $enum_variant:ident) => {
        #[cfg(feature = "developer")]
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Self::$enum_variant(value)
            }
        }
    };

    ($($(#[$attr_type:ident])? $error:ty => $enum_variant:ident,)+) => {
        $(
            map_error!($(#[$attr_type])? $error => $enum_variant);
        )+
    };
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Yaml(serde_yaml::Error),
    Json(serde_json::Error),
    Header(String),
    StripPrefix(StripPrefixError),
    PathDisplay,
    Parsing {
        message: String,
        region: Range<usize>,
        source: String,
    },
    Nice(String),

    #[cfg(feature = "developer")]
    Regex(regex::Error),
    #[cfg(feature = "developer")]
    AddrParse(std::net::AddrParseError),
    #[cfg(feature = "developer")]
    UriParse(hyper::http::uri::InvalidUri),
    #[cfg(feature = "developer")]
    Hyper(hyper::Error),
}

map_error! {
    io::Error => Io,
    serde_yaml::Error => Yaml,
    serde_json::Error => Json,
    StripPrefixError => StripPrefix,
    #[developer] hyper::Error => Hyper,
    #[developer] std::net::AddrParseError => AddrParse,
    #[developer] hyper::http::uri::InvalidUri => UriParse,
    #[developer] regex::Error => Regex,
}

#[cfg(feature = "developer")]
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(..)
            | Self::Yaml(..)
            | Self::StripPrefix(..)
            | Self::Json(..)
            | Self::PathDisplay
            | Self::Hyper(..)
            | Self::AddrParse(..)
            | Self::UriParse(..)
            | Self::Regex(..) => {
                write!(f, "{:?}", self)
            }
            Self::Parsing { .. } => self.display_parsing_error(f),
            Self::Header(..) => self.display_header_error(f),
            Self::Nice(msg) => write!(f, "{} {msg}", "Error:".red().bold()),
        }
    }
}

#[cfg(not(feature = "developer"))]
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(..)
            | Self::Yaml(..)
            | Self::Json(..)
            | Self::PathDisplay
            | Self::StripPrefix(..) => {
                write!(f, "{:?}", self)
            }
            Self::Parsing { .. } => self.display_parsing_error(f),
            Self::Header(..) => self.display_header_error(f),
            Self::Nice(msg) => write!(f, "{} {msg}", "Error:".red().bold()),
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
    fn display_parsing_error(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            write!(
                f,
                "| [{}:{}] {} {}",
                row_begin + 1,
                col_begin,
                "Parsing error:".red(),
                message
            )?;
            write!(f, "| ")?;
            let line = lines[row_begin];
            write!(
                f,
                "| {} {}{}{}",
                row_begin + 1,
                &line[..col_begin],
                &line[col_begin..col_end].red(),
                &line[col_end..]
            )?;
            write!(
                f,
                "| {} {}{}",
                " ".repeat(line[..col_begin].len() + (row_begin + 1).to_string().len()),
                "~".repeat(line[col_begin..col_end].len()).red(),
                " ".repeat(line[col_end..].len())
            )?;
        } else {
            unreachable!();
        }
        Ok(())
    }

    fn display_header_error(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Header(msg) = self {
            write!(f, "{}{}", "Header error:".red(), msg)?;
        } else {
            unreachable!();
        }
        Ok(())
    }
}
