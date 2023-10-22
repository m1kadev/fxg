use logos::Logos;

use crate::lexer::{Lexeme, Lexer};

// fn parse_eq1(lexer: &mut Lexer) -> Option<DocumentNode> {
//     assert_eq!(lexer.next()?, Lexeme::Eq1);
//     let mut node = DocumentNode::Header1(vec![]);
//     loop {
//         match lexer.peek() {
//             Some(Lexeme::Eq1) => break,
//             Some(_) => {
//                 if let DocumentNode::Header1(data) = &mut node {
//                     data.push(Self::parse_node(lexer)?)
//                 }
//             }
//             None => unreachable!(),
//         }
//     }
//     Some(node)
// }

macro_rules! parser_xtx {
    ($func:ident : $lexeme:ident => $node:ident) => {
        fn $func(lexer: &mut Lexer) -> Option<DocumentNode> {
            assert_eq!(lexer.next()?, Lexeme::$lexeme);
            let mut nodes = vec![];
            loop {
                match lexer.peek() {
                    Some(Lexeme::$lexeme) => {
                        lexer.next();
                        break;
                    }
                    Some(_) => nodes.push(Self::parse_node(lexer)?),
                    None => unreachable!(),
                }
            }
            Some(DocumentNode::$node(nodes))
        }
    };
}

// rust doesnt allow macros inside of enum decls so this is what were doing now
#[derive(Debug)]
pub enum DocumentNode {
    Header1(Vec<DocumentNode>),
    Header2(Vec<DocumentNode>),
    Header3(Vec<DocumentNode>),
    Header4(Vec<DocumentNode>),
    Header5(Vec<DocumentNode>),
    Header6(Vec<DocumentNode>),

    Bold(Vec<DocumentNode>),
    Italic(Vec<DocumentNode>),
    Underline(Vec<DocumentNode>),

    Link {
        text: Vec<DocumentNode>,
        href: String,
    },
    Image {
        alt: String,
        src: String,
    },

    Text(String),

    LineBreak,
}

#[inline]
fn tag_helper_fn(output: &mut String, tag_name: &'static str, tag_contents: &[DocumentNode]) {
    output.push('<');
    output.push_str(tag_name);
    output.push('>');

    for node in tag_contents {
        node.as_html(output);
    }

    output.push_str("</");
    output.push_str(tag_name);
    output.push('>');
}

impl DocumentNode {
    fn as_html(&self, output: &mut String) {
        match self {
            Self::Header1(nodes) => tag_helper_fn(output, "h1", nodes),
            Self::Header2(nodes) => tag_helper_fn(output, "h2", nodes),
            Self::Header3(nodes) => tag_helper_fn(output, "h3", nodes),
            Self::Header4(nodes) => tag_helper_fn(output, "h4", nodes),
            Self::Header5(nodes) => tag_helper_fn(output, "h5", nodes),
            Self::Header6(nodes) => tag_helper_fn(output, "h6", nodes),

            Self::Bold(nodes) => tag_helper_fn(output, "b", nodes),
            Self::Italic(nodes) => tag_helper_fn(output, "i", nodes),
            Self::Underline(nodes) => tag_helper_fn(output, "u", nodes),

            Self::LineBreak => output.push_str("<br/>"),
            Self::Text(text) => output.push_str(text),

            Self::Link { text, href: to } => {
                output.push_str("<a href=\"");
                output.push_str(to);
                output.push('>');

                for node in text {
                    node.as_html(output);
                }

                output.push_str("</a>");
            }

            Self::Image { alt, src } => {
                output.push_str("<img src=\"");
                output.push_str(src);
                output.push_str("\" alt=\"");
                output.push_str(alt);
                output.push_str("\">");
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Document {
    nodes: Vec<DocumentNode>,
}

impl Document {
    pub fn as_html(&self) -> String {
        let mut output = String::new();
        for node in &self.nodes {
            node.as_html(&mut output);
        }
        output
    }

    pub fn build(lexer: &mut Lexer) -> Self {
        let mut this = Self::default();
        while let Some(node) = Self::parse_node(lexer) {
            this.nodes.push(node);
        }
        this
    }

    fn parse_node(lexer: &mut Lexer) -> Option<DocumentNode> {
        let node = lexer.peek()?;
        match node {
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
            Lexeme::LDoubleAngleBrace => Self::parse_raw(lexer),

            Lexeme::Text => {
                lexer.next();
                Some(DocumentNode::Text(lexer.slice().to_string()))
            }
            Lexeme::Newline => {
                lexer.next();
                Some(DocumentNode::LineBreak)
            }
            _ => unreachable!(),
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

    // <https://tagsdev.nl/ My website!>
    fn parse_link(lexer: &mut Lexer) -> Option<DocumentNode> {
        assert_eq!(lexer.next()?, Lexeme::LAngle);
        if let Some(Lexeme::Text) = lexer.next() {
            let slice = lexer.slice();
            let mut split = slice.split(' ');
            let href = split.next()?.to_string();
            let text_raw = split.remainder()?;
            let text = Document::build(&mut Lexer::lex(text_raw)).nodes;
            Some(DocumentNode::Link { text, href })
        } else {
            None
        }
    }

    // <!/dog.png Dog>
    fn parse_img(lexer: &mut Lexer) -> Option<DocumentNode> {
        assert_eq!(lexer.next()?, Lexeme::LAngleBang);
        if let Some(Lexeme::Text) = lexer.next() {
            let slice = lexer.slice();
            let mut split = slice.split(' ');
            let src = split.next()?.to_string();
            let alt = split.remainder()?.to_string();
            Some(DocumentNode::Image { alt, src })
        } else {
            None
        }
    }

    fn parse_raw(lexer: &mut Lexer) -> Option<DocumentNode> {
        None
    }
}
