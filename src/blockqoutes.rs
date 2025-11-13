use std::{
    fs::read,
    io::{BufRead, BufReader, Read},
};

use crate::{escape, parser::parse_text};

pub fn parse_blockqoute<T>(reader: &mut BufReader<T>, mut line: String) -> (String, String)
where
    T: Read,
{
    parse_blockqoute_internal(reader, line, 1)
}

fn parse_blockqoute_internal<T>(
    reader: &mut BufReader<T>,
    line: String,
    depth: usize,
) -> (String, String)
where
    T: Read,
{
    let mut qoute = line;
    let mut line = String::new();
    while let Ok(len) = reader.read_line(&mut line) {
        if len == 0 {
            break;
        }
        if line.starts_with(&['>', '-']) {
            qoute.push_str(&line);
        } else {
            break;
        }
        line.clear();
    }
    let mut qoute_data: Vec<(usize, &str)> = vec![];
    for line in qoute.lines() {
        let qoutes = line
            .find(|c: char| !c.is_whitespace() && !['>', '-'].contains(&c))
            .unwrap();
    }
    (qoute, line)
}
