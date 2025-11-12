use std::{
    fs::read,
    io::{BufRead, BufReader, Read},
};

use crate::{escape, parser::parse_text};

pub fn parse_blockqoute<T>(reader: &mut BufReader<T>, mut line: String) -> (String, String)
where
    T: Read,
{
    let mut output = String::new();
    output.push_str(escape!("<"));
    output.push_str("figure");
    output.push_str(escape!(">"));
    output.push_str(escape!("<"));
    output.push_str("blockqoute");
    output.push_str(escape!(">"));
    dbg!(&line[1..]);
    output.push_str(&parse_text(&line[1..]));
    line.clear();
    while let Ok(length) = reader.read_line(&mut line) {
        dbg!(&line);
        if length == 0 {
            break;
        }
        if line.starts_with('>') {
            output.push_str(&parse_text(&line));
        } else if line.starts_with('-') {
            output.push_str(escape!("<"));
            output.push_str("/blockqoute");
            output.push_str(escape!(">"));
            output.push_str(escape!("<"));
            output.push_str("figcaption");
            output.push_str(escape!(">"));
            output.push_str(&parse_text(&line[1..]));
            output.push_str(escape!("<"));
            output.push_str("/figcaption");
            output.push_str(escape!(">"));
            output.push_str(escape!("<"));
            output.push_str("/figure");
            output.push_str(escape!(">"));
        } else {
            output.push_str(escape!("<"));
            output.push_str("/blockqoute");
            output.push_str(escape!(">"));
            output.push_str(escape!("<"));
            output.push_str("/figure");
            output.push_str(escape!(">"));
            break;
        }
        line.clear();
    }
    (output, line)
}
