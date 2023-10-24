use crate::compiler::document_nodes::DocumentNode;
use crate::compiler::lexer::Lexeme;
use crate::compiler::Lexer;
use crate::Error;

macro_rules! parser_xtx {
    ($func:ident : $lexeme:ident => $node:ident) => {
        fn $func(lexer: &mut Lexer) -> Result<DocumentNode, Error> {
            let mut nodes = vec![];
            let region = lexer.span();
            dbg!(lexer.peek());
            loop {
                match lexer.peek() {
                    Some(Lexeme::$lexeme) => {
                        lexer.next();
                        break;
                    }
                    Some(_) => nodes.push(Self::parse_node(lexer)?),
                    None => {
                        return Err(Error::ParsingError {
                            message: format!(
                                "Failed to find closing {:?} for this {:?}",
                                Lexeme::$lexeme,
                                Lexeme::$lexeme,
                            ),
                            region,
                            source: lexer.source().to_string(),
                        })
                    }
                }
            }
            Ok(DocumentNode::$node(nodes))
        }
    };
}

#[derive(Default, Debug)]
pub struct Document(Vec<DocumentNode>);

impl Document {
    pub fn as_html(&self) -> String {
        let mut output = String::new();
        for node in &self.0 {
            node.as_html(&mut output);
        }
        output
    }

    pub fn build(lexer: &mut Lexer) -> Result<Self, Error> {
        let mut this = Self::default();
        loop {
            let node = Self::parse_node(lexer)?;
            if node == DocumentNode::Eof {
                break;
            } else {
                this.0.push(node);
            }
        }
        Ok(this)
    }

    fn parse_node(lexer: &mut Lexer) -> Result<DocumentNode, Error> {
        let node = lexer.next();
        if let Some(lexeme) = node {
            match lexeme {
                Lexeme::Eq1 => Self::parse_eq1(lexer),
                Lexeme::Eq2 => Self::parse_eq2(lexer),
                Lexeme::Eq3 => Self::parse_eq3(lexer),
                Lexeme::Eq4 => Self::parse_eq4(lexer),
                Lexeme::Eq5 => Self::parse_eq5(lexer),
                Lexeme::Eq6 => Self::parse_eq6(lexer),

                Lexeme::DoubleBang => Self::parse_bold(lexer),
                Lexeme::DoubleSlash => Self::parse_cursive(lexer),
                Lexeme::DoubleUnderscore => Self::parse_underline(lexer),

                Lexeme::LAngle => Self::parse_link(lexer),
                Lexeme::LAngleBang => Self::parse_img(lexer),
                Lexeme::LNAngleBrace => Self::parse_raw(lexer),

                Lexeme::Text => {
                    lexer.next();
                    Ok(DocumentNode::Text(lexer.slice().to_string()))
                }
                Lexeme::Newline => {
                    lexer.next();
                    Ok(DocumentNode::LineBreak)
                }
                lexeme => panic!("Unexpected token: {lexeme:?}"),
            }
        } else {
            Ok(DocumentNode::Eof)
        }
    }

    parser_xtx!(parse_eq1: Eq1 => Header1);
    parser_xtx!(parse_eq2: Eq2 => Header2);
    parser_xtx!(parse_eq3: Eq3 => Header3);
    parser_xtx!(parse_eq4: Eq4 => Header4);
    parser_xtx!(parse_eq5: Eq5 => Header5);
    parser_xtx!(parse_eq6: Eq6 => Header6);

    parser_xtx!(parse_cursive: DoubleSlash => Italic);
    parser_xtx!(parse_bold: DoubleBang => Bold);
    parser_xtx!(parse_underline: DoubleUnderscore => Underline);

    fn parse_link(lexer: &mut Lexer) -> Result<DocumentNode, Error> {
        let region = lexer.span();

        let mut tag_contents = String::new();
        loop {
            let tok = lexer.next();
            if let Some(Lexeme::RAngle) = tok {
                break;
            } else if tok.is_some() {
                tag_contents.push_str(lexer.slice());
            } else {
                return Err(Error::ParsingError {
                    message: format!(
                        "Failed to find closing {:?} for this {:?}",
                        Lexeme::RAngle,
                        Lexeme::LAngle
                    ),
                    region,
                    source: lexer.source().to_string(),
                });
            }
        }
        let mut split = tag_contents.split(' ');
        let href = split.next();
        let remainder = split.remainder();
        if href.is_none() {
            let whole_region = region.start..lexer.span().end;
            return Err(Error::ParsingError {
                message: "No href was found for this link element.".to_string(),
                region: whole_region,
                source: lexer.source().to_string(),
            });
        }
        if remainder.is_none() {
            let whole_region = region.start..lexer.span().end;
            return Err(Error::ParsingError {
                message: "No link text was found for this link element.".to_string(),
                region: whole_region,
                source: lexer.source().to_string(),
            });
        }
        Ok(DocumentNode::Link {
            text: Document::build(&mut Lexer::lex(remainder.unwrap()))?.0,
            href: split.next().unwrap().to_string(),
        })
    }

    fn parse_img(lexer: &mut Lexer) -> Result<DocumentNode, Error> {
        let region = lexer.span();
        let mut tag_contents = String::new();

        loop {
            let lexeme = lexer.next();
            if lexeme == Some(Lexeme::RAngle) {
                break;
            } else if lexeme.is_some() {
                tag_contents.push_str(lexer.slice());
            } else {
                return Err(Error::ParsingError {
                    message: format!(
                        "Failed to find closing {:?} for this {:?}",
                        Lexeme::RAngle,
                        Lexeme::LAngleBang
                    ),
                    region,
                    source: lexer.source().to_string(),
                });
            }
        }

        let mut split = tag_contents.split(' ');
        let src = split.next().map(|x| x.to_string());
        let mut alt = split.remainder().map(|x| x.to_string());
        if src.is_none() {
            let whole_region = region.start..lexer.span().end;
            return Err(Error::ParsingError {
                message: "No image source was found for this image element.".to_string(),
                region: whole_region,
                source: lexer.source().to_string(),
            });
        }
        if alt.is_none() {
            alt = Some(String::new());
        }
        Ok(DocumentNode::Image {
            src: src.unwrap(),
            alt: alt.unwrap(),
        })
    }

    fn parse_raw(lexer: &mut Lexer) -> Result<DocumentNode, Error> {
        let count = lexer.slice().len();
        let mut text = String::new();
        let region = lexer.span();
        loop {
            let token = lexer.next();
            if token == Some(Lexeme::RNAngleBrace) && lexer.slice().len() == count {
                break;
            } else if token.is_some() {
                text.push_str(lexer.slice());
            } else {
                return Err(Error::ParsingError {
                    message: format!(
                        "Failed to find closing {:?} for this {:?}",
                        Lexeme::RNAngleBrace,
                        Lexeme::LNAngleBrace
                    ),
                    region,
                    source: lexer.source().to_string(),
                });
            }
        }
        Ok(DocumentNode::Text(text))
    }
}
