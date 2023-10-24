use crate::compiler::document_nodes::DocumentNode;
use crate::compiler::lexer::Lexeme;
use crate::compiler::Lexer;

macro_rules! parser_xtx {
    ($func:ident : $lexeme:ident => $node:ident) => {
        fn $func(lexer: &mut Lexer) -> Option<DocumentNode> {
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

    pub fn build(lexer: &mut Lexer) -> Self {
        let mut this = Self::default();
        while let Some(node) = Self::parse_node(lexer) {
            this.0.push(node);
        }
        this
    }

    fn parse_node(lexer: &mut Lexer) -> Option<DocumentNode> {
        let node = lexer.next()?;
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
            Lexeme::LNAngleBrace => Self::parse_raw(lexer),

            Lexeme::Text => {
                lexer.next();
                Some(DocumentNode::Text(lexer.slice().to_string()))
            }
            Lexeme::Newline => {
                lexer.next();
                Some(DocumentNode::LineBreak)
            }
            lexeme => panic!("Unexpected token: {lexeme:?}"),
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

    fn parse_link(lexer: &mut Lexer) -> Option<DocumentNode> {
        let mut tag_contents = String::new();
        loop {
            if lexer.peek()? == &Lexeme::RAngle {
                lexer.next();
                break;
            }
            lexer.next();
            tag_contents.push_str(lexer.slice());
        }
        let mut split = tag_contents.split(' ');
        Some(DocumentNode::Link {
            text: Document::build(&mut Lexer::lex(split.remainder()?)).0,
            href: split.next()?.to_string(),
        })
    }

    fn parse_img(lexer: &mut Lexer) -> Option<DocumentNode> {
        let mut tag_contents = String::new();
        loop {
            let lexeme = lexer.next()?;
            if lexeme == Lexeme::RAngle {
                break;
            }
            tag_contents.push_str(lexer.slice());
        }
        let mut split = tag_contents.split(' ');
        Some(DocumentNode::Image {
            src: split.next()?.to_string(),
            alt: split.remainder()?.to_string(),
        })
    }

    fn parse_raw(lexer: &mut Lexer) -> Option<DocumentNode> {
        debug_assert_eq!(lexer.next()?, Lexeme::LNAngleBrace);
        let count = lexer.slice().len();
        let mut text = String::new();
        loop {
            let token = lexer.next()?;
            if token == Lexeme::RNAngleBrace && lexer.slice().len() == count {
                break;
            }
            text.push_str(lexer.slice());
        }
        Some(DocumentNode::Text(text))
    }
}
