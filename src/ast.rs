use std::iter::Peekable;
use std::vec::IntoIter;

use crate::lexer;

type Range = std::ops::Range<usize>;
type Lexer<'a> = Peekable<IntoIter<lexer::LexData>>;

#[derive(Debug)]
pub enum AstNode {
    Paragraph(Vec<AstNode>),
    Text(Range),

    Cursive(Vec<AstNode>),
    Bold(Vec<AstNode>),
    Underline(Vec<AstNode>),
    Deemphasised(Vec<AstNode>),
}

#[derive(Default, Debug)]
pub struct Ast(pub Vec<AstNode>);

pub fn build_ast(lex: Vec<lexer::LexData>, source: &str) -> Ast {
    let mut ast = Ast::default();

    let mut peekable = lex.into_iter().peekable();

    while let Some(_) = peekable.peek() {
        ast.0.push(parse_paragraph(&mut peekable));
    }

    ast
}

fn parse_paragraph(lexer: &mut Lexer)  -> AstNode {
    let mut collector = vec![];
    loop {
        collector.append(&mut parse_text(lexer));
        if let Some((lexer::Lexeme::Newline, _)) = lexer.next() {
            break;
        } else if let None = lexer.next() {
            break;
        }
    }

    AstNode::Paragraph(collector)
}

fn parse_text(lexer: &mut Lexer) -> Vec<AstNode> {
    let mut collector = vec![];
    while let Some((lexeme, range)) = lexer.next() {
        let node = match lexeme {
            lexer::Lexeme::Newline => break,
            lexer::Lexeme::Text => AstNode::Text(range.clone()),
            _ => todo!(),
        };
        collector.push(node);
    }
    collector
}

