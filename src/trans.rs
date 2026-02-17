use crate::ast::{Ast, AstNode};

pub fn translate(ast: Ast, source: &str) -> String {

    let nodes = ast.0;
    let mut transbuf = String::new();

    for node in nodes {
        match node {
            AstNode::Paragraph(contents) => {
                transbuf.push_str("<p>");
                for node in contents {
                    translate_node(node, source, &mut transbuf);
                }
                transbuf.push_str("</p>");
            }
            _ => unreachable!(),
        }
    }
    return transbuf;
}

fn translate_node(node: AstNode, source: &str, transbuf: &mut String) {
    match node {
        AstNode::Paragraph(_) => unreachable!(),
        AstNode::Text(range) => transbuf.push_str(&source[range]),
        AstNode::Cursive(nodes) => {
            transbuf.push_str("<em>");
            for node in nodes {
                translate_node(node, source, transbuf);
            }
            transbuf.push_str("</em>");
        },
        AstNode::Bold(nodes) => {
            transbuf.push_str("<strong>");
            for node in nodes {
                translate_node(node, source, transbuf);
            }
            transbuf.push_str("</strong>");
        },
        AstNode::Underline(nodes) => {
            transbuf.push_str("<u>");
            for node in nodes {
                translate_node(node, source, transbuf);
            }
            transbuf.push_str("</u>");
        },
        AstNode::Deemphasised(nodes) => {
            transbuf.push_str("<small>");
            for node in nodes {
                translate_node(node, source, transbuf);
            }
            transbuf.push_str("</small>");
        },
        AstNode::Space => transbuf.push(' '),
        _ => todo!(),
    }
}
