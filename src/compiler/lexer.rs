use std::iter::Peekable;
use std::ops::Range;

use logos::Logos;

use crate::error::Error;

use super::header::DocumentHeader;

#[derive(Logos, Debug, PartialEq, Eq)]
#[logos(skip r"[\r\t\n\f]+")]
pub enum Lexeme {
    #[token(r"=")]
    Eq1,

    #[token(r"==")]
    Eq2,

    #[token(r"===")]
    Eq3,

    #[token(r"====")]
    Eq4,

    #[token(r"=====")]
    Eq5,

    #[token(r"======")]
    Eq6,

    #[token(r"//")]
    DoubleSlash,

    #[token(r"!!")]
    DoubleBang,

    #[token("__")]
    DoubleUnderscore,

    #[token(r"<")]
    LAngle,

    #[token("<!")]
    LAngleBang,

    #[token(r">")]
    RAngle,

    #[regex(r"\[+")]
    LNAngleBrace,

    #[regex(r"\]+")]
    RNAngleBrace,

    #[regex(r"\n{2,}|\r{2,}|(\r\n){2,}")]
    Newline,

    Text,
}

pub struct Lexer<'src> {
    lexer: logos::Lexer<'src, Lexeme>,
    slice_override: Option<Range<usize>>,
    peekable: Peekable<logos::Lexer<'src, Lexeme>>,

    pub header: DocumentHeader,
}

// public functions
impl<'src> Lexer<'src> {
    pub fn lex(mut source: &'src str) -> Result<Self, Error> {
        // ! this code should technically be in document, but logos doesnt allow anchors
        if source.starts_with("---") {
            if source.matches("---").count() < 2 {
                return Err(Error::Parsing {
                    message: "Failed to find closing --- for this ---".to_string(),
                    region: 0..3,
                    source: source.to_string(),
                });
            }
            let (yaml, remainder) = &source[3..].split_once("---").unwrap();
            let header = serde_yaml::from_str::<DocumentHeader>(yaml)?;
            source = remainder;
            let lexer = Lexeme::lexer(source);
            let peekable = Lexeme::lexer(source).peekable();
            Ok(Self {
                lexer,
                peekable,
                slice_override: None,
                header,
            })
        } else {
            dbg!(&source);
            Err(Error::Parsing {
                message: "No header found for this file".into(),
                region: 0..1,
                source: source.to_string(),
            })
        }
    }

    pub fn slice(&self) -> &str {
        if let Some(span) = &self.slice_override {
            // its 16 bytes who really cares tbh
            &self.lexer.source()[span.clone()]
        } else {
            self.lexer.slice()
        }
    }

    pub fn peek(&mut self) -> Option<&Lexeme> {
        let item = self.internal_peek()?;

        if let Ok(lexeme) = item {
            Some(lexeme)
        } else {
            Some(&Lexeme::Text)
        }
    }

    pub fn span(&self) -> Range<usize> {
        self.lexer.span()
    }

    pub fn source(&self) -> &str {
        self.lexer.source()
    }
}

// private functions
impl<'src> Lexer<'src> {
    fn find_error_chain(&mut self) {
        let begin = self.lexer.span().start;
        let mut end = self.lexer.span().start;
        while let Some(Err(..)) = self.internal_peek() {
            self.next();
            end += 1;
        }
        end += 1;
        self.slice_override = Some(begin..end);
    }

    fn next(&mut self) -> Option<Result<Lexeme, ()>> {
        self.peekable.next();
        self.lexer.next()
    }

    fn internal_peek(&mut self) -> Option<&Result<Lexeme, ()>> {
        self.peekable.peek()
    }
}

impl Iterator for Lexer<'_> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(lexeme) = self.next()? {
            self.slice_override = None;
            Some(lexeme)
        } else {
            self.find_error_chain();
            Some(Lexeme::Text)
        }
    }
}
