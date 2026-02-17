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
    Space,
}

#[derive(Default, Debug)]
pub struct Ast(pub Vec<AstNode>);

pub fn build_ast(lex: Vec<lexer::LexData>) -> Ast {
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
        let mut text = parse_text(lexer);
        collector.append(&mut text);
        if let Some((lexer::Lexeme::Newline, _)) = lexer.peek() {
            break;
        } else if let None = lexer.peek() {
            break;
        }
        collector.push(AstNode::Space);
    }

    AstNode::Paragraph(collector)
}

fn parse_text(lexer: &mut Lexer) -> Vec<AstNode> {
    let mut collector = vec![];
    while let Some((lexeme, range)) = lexer.next() {
        match lexeme {
            lexer::Lexeme::Newline => break,
            lexer::Lexeme::Text => collector.push(AstNode::Text(range.clone())),
            lexer::Lexeme::Cursive => collector.append(&mut parse_cursive(lexer, range)),
            lexer::Lexeme::Bold => collector.append(&mut parse_bold(lexer, range)),
            lexer::Lexeme::Underline => collector.append(&mut parse_underline(lexer, range)),
            lexer::Lexeme::Deemphasised => collector.append(&mut parse_deemphasised(lexer, range)),
            _ => todo!(),
        };
    }
    return collector 
}

fn parse_cursive(lexer: &mut Lexer, cursive_pos: Range) -> Vec<AstNode> {
    let mut collector = vec![];
    while let Some((lexeme, b_range)) = lexer.peek() {
        let range = b_range.clone(); 
        let node = match lexeme {
            lexer::Lexeme::Text => collector.push(AstNode::Text(range)),
            lexer::Lexeme::Newline => {
                collector.insert(0, AstNode::Text(cursive_pos));
                lexer.next();
                return collector;
            },
            lexer::Lexeme::Cursive => {
                lexer.next();
                break;
            },
            lexer::Lexeme::Bold => collector.append(&mut parse_bold(lexer, range)),
            lexer::Lexeme::Deemphasised => collector.append(&mut parse_deemphasised(lexer, range)),
            lexer::Lexeme::Underline => collector.append(&mut parse_underline(lexer, range)),
            _ => todo!(),
        };
        lexer.next();
    }
    if let None = lexer.peek() {
        collector.insert(0, AstNode::Text(cursive_pos));
        return collector;
    }
    vec![AstNode::Cursive(collector)]
}

fn parse_bold(lexer: &mut Lexer, cursive_pos: Range) -> Vec<AstNode> {
    let mut collector = vec![];
    while let Some((lexeme, b_range)) = lexer.peek() {
        let range = b_range.clone(); 
        match lexeme {
            lexer::Lexeme::Text => collector.push(AstNode::Text(range)),
            lexer::Lexeme::Newline => {
                collector.insert(0, AstNode::Text(cursive_pos));
                lexer.next();
                return collector;
            },
            lexer::Lexeme::Bold => {
                lexer.next();
                break;
            },
            lexer::Lexeme::Cursive => collector.append(&mut parse_cursive(lexer, range)),
            lexer::Lexeme::Deemphasised => collector.append(&mut parse_deemphasised(lexer, range)),
            lexer::Lexeme::Underline => collector.append(&mut parse_underline(lexer, range)),
            _ => todo!(),
        };
        lexer.next();
    }
    if let None = lexer.peek() {
        collector.insert(0, AstNode::Text(cursive_pos));
        return collector;
    }
    vec![AstNode::Bold(collector)]
}

fn parse_underline(lexer: &mut Lexer, cursive_pos: Range) -> Vec<AstNode> {
    let mut collector = vec![];
    while let Some((lexeme, b_range)) = lexer.peek() {
        let range = b_range.clone();
            match lexeme {
            lexer::Lexeme::Text => collector.push(AstNode::Text(range)),
            lexer::Lexeme::Newline => {
                collector.insert(0, AstNode::Text(cursive_pos));
                lexer.next();
                return collector;
            },
            lexer::Lexeme::Underline => {
                lexer.next();
                break;
            },
            lexer::Lexeme::Cursive => collector.append(&mut parse_cursive(lexer, range)),
            lexer::Lexeme::Deemphasised => collector.append(&mut parse_deemphasised(lexer, range)),
            lexer::Lexeme::Bold => collector.append(&mut parse_bold(lexer, range)),
            _ => todo!(),
        };
        lexer.next();
    }
    if let None = lexer.peek() {
        collector.insert(0, AstNode::Text(cursive_pos));
        return collector;
    }
    vec![AstNode::Underline(collector)]
}

fn parse_deemphasised(lexer: &mut Lexer, cursive_pos: Range) -> Vec<AstNode> {
    let mut collector = vec![];
    while let Some((lexeme, b_range)) = lexer.peek() {
        let range = b_range.clone();
        match lexeme {
            lexer::Lexeme::Text => collector.push(AstNode::Text(range)),
            lexer::Lexeme::Newline => {
                collector.insert(0, AstNode::Text(cursive_pos));
                lexer.next();
                return collector;
            },
            lexer::Lexeme::Deemphasised => {
                lexer.next();
                break;
            },
            lexer::Lexeme::Cursive => collector.append(&mut parse_cursive(lexer, range)),
            lexer::Lexeme::Underline => collector.append(&mut parse_underline(lexer, range)),
            lexer::Lexeme::Bold => collector.append(&mut parse_bold(lexer, range)),
            _ => todo!(),
        };
        lexer.next();
    }
    if let None = lexer.peek() {
        collector.insert(0, AstNode::Text(cursive_pos));
        return collector;
    }
    vec![AstNode::Deemphasised(collector)]
}
