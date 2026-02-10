use crate::ast::{Ast, AstNode};

pub fn translate(ast: Ast, source: &str) {

    let nodes = ast.0;

    for node in nodes {
        match node {
            AstNode::Paragraph(contents) => {
                print!("<p>");
                for node in contents {
                    translate_node(node, source);
                }
                print!("</p>");
            }
            _ => unreachable!(),
        }
    }
}

fn translate_node(node: AstNode, source: &str) {
    match node {
        AstNode::Paragraph(_) => unreachable!(),
        AstNode::Text(range) => print!("{}", &source[range]),
        _ => todo!(),
    }
}
