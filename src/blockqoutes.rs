use std::io::{BufRead, BufReader, Read};

use crate::{extensions::HtmlWriting, parser::parse_text};

struct QouteData<'lines>(isize, bool, &'lines str);

pub fn parse_blockqoute<T>(reader: &mut BufReader<T>, line: String) -> (String, String)
where
    T: Read,
{
    parse_blockqoute_internal(reader, line)
}

fn parse_blockqoute_internal<T>(reader: &mut BufReader<T>, line: String) -> (String, String)
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
    let mut qoute_data: Vec<QouteData> = vec![];
    for line in qoute.lines() {
        let qoutes_idx = line
            .find(|c: char| c != '>' && c != '-' && !c.is_whitespace())
            .unwrap();
        let qoutes = line[..qoutes_idx]
            .chars()
            .filter(|x| *x == '>')
            .count()
            .try_into()
            .unwrap();
        if line[..qoutes_idx].trim().ends_with('-') {
            qoute_data.push(QouteData(qoutes, true, &line[qoutes_idx..]));
        } else {
            qoute_data.push(QouteData(qoutes, false, &line[qoutes_idx..]));
        }
    }

    let mut current_depth = 0isize;
    let mut output = String::new();
    for data in qoute_data {
        if data.0 != current_depth {
            if data.1 {
                output.write_closing_tag("blockqoute");
                output.write_tag("figcaption", data.2);
                output.write_closing_tag("figure");
                continue;
            }
            let delta = data.0 - current_depth;
            if delta < 0 {
                for _ in 0..delta.abs() {
                    output.write_closing_tag("blockqoute");
                    output.write_closing_tag("figure");
                }
                current_depth = data.0;
            } else {
                for _ in 0..delta {
                    output.write_opening_tag("figure", &[]);
                    output.write_opening_tag("blockqoute", &[]);
                }
                current_depth = data.0;
            }
        }
        output.push_str(&parse_text(data.2));
        output.write_opening_tag("br", &[]);
    }

    for _ in 0..current_depth {
        output.write_closing_tag("blockqoute");
        output.write_closing_tag("figure");
    }

    (output, line)
}
