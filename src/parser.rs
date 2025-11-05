// TODO: macro for repeated push_str calls

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use phf_macros::phf_map;

macro_rules! escape {
    ($item:ident) => {
        UNICODE_PLACEHOLDERS.get($item).unwrap()
    };
    ($item:literal) => {
        UNICODE_PLACEHOLDERS.get($item).unwrap()
    };
}

static UNICODE_PLACEHOLDERS: phf::Map<&'static str, &'static str> = phf_map! {
    "//" => "\u{E001}",
    ">" => "\u{E002}",
    "<" => "\u{E003}",
};

pub fn parse(reader: &mut BufReader<File>) -> String {
    let mut output = String::new();
    let mut lnbuf = String::new();

    let mut last_line_was_title = false;

    while let Ok(n) = reader.read_line(&mut lnbuf) {
        let line = lnbuf.trim();
        if n == 0 {
            // EOF reached
            break;
        }
        if lnbuf.starts_with('=') {
            output.push_str(&parse_title(line));
            last_line_was_title = true;
        } else if line.is_empty() && !last_line_was_title {
            output.push_str(escape!("<"));
            output.push_str("br/");
            output.push_str(escape!(">"));
        } else {
            output.push_str(&parse_text(line));
            output.push(' ');
            last_line_was_title = false;
        }
        lnbuf.clear();
    }

    for (key, placeholder) in UNICODE_PLACEHOLDERS.entries() {
        output = output.replace(placeholder, key);
    }
    output
}

fn parse_markup(input: &str, markup: &'static str, html_tag: &'static str) -> String {
    let mut line = input.to_string();
    let mut output = String::new();
    let mut found = false;
    let mut tag_contents = "";
    let mut tag_remainder = "";
    while let Some(idx) = line.find(markup) {
        found = true;
        if &input[idx - 1..idx] == "\\" {
            line = line.replacen(&format!("\\{markup}"), escape!(markup), 1); // ? can we remove this format! call
        } else {
            tag_contents = &line[..idx];
            tag_remainder = &line[idx + markup.len()..];
            break;
        }
    }

    if found {
        output.push_str(escape!("<"));
        output.push_str(html_tag);
        output.push_str(escape!(">"));
        output.push_str(&parse_text(tag_contents));
        output.push_str(escape!("<"));
        output.push_str("/");
        output.push_str(html_tag);
        output.push_str(escape!(">"));
        output.push_str(&parse_text(tag_remainder));
    } else {
        // insert the text without further markup
        output.push_str(markup);
        output.push_str(&parse_text(&line));
    }

    output
}

fn parse_title(line: &str) -> String {
    let mut output = String::new();
    let (prefix, _) = line.split_once(' ').unwrap();
    let header_size = prefix.len();
    // header syntax is complete

    if line.ends_with(&str::repeat("=", header_size)) {
        let header_contents = &line[header_size..line.len() - 1 - header_size].trim();
        output.push_str(escape!("<"));
        output.push('h');
        output.push_str(&header_size.to_string());
        output.push_str(escape!(">"));
        output.push_str(&parse_text(header_contents));
        output.push_str(escape!("<"));
        output.push_str("/h");
        output.push_str(&header_size.to_string());
        output.push_str(escape!(">"));
    } else {
        // parse the text normally
        output.push_str(&parse_text(line));
    }
    output
}

fn parse_text(line: &str) -> String {
    let mut output = String::new();
    // find opening tag
    let cursive = line.find("//");
    let bold = line.find("!!");
    let underline = line.find("__");
    let code = line.find("<>");
    let smallest = [cursive, bold, underline, code]
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .min();
    if smallest.is_none() {
        return line.to_string();
    } else {
        // ? lot of repeating here,, see if more efficient way is possible
        if smallest == cursive {
            let idx = smallest.unwrap();
            let text = &line[idx + 2..];
            output.push_str(&line[..idx]);
            output.push_str(&parse_markup(text, "//", "em"));
        } else if smallest == bold {
            let idx = smallest.unwrap();
            let text = &line[idx + 2..];
            output.push_str(&line[..idx]);
            output.push_str(&parse_markup(text, "!!", "strong"));
        } else if smallest == underline {
            let idx = smallest.unwrap();
            let text = &line[idx + 2..];
            output.push_str(&line[..idx]);
            output.push_str(&parse_markup(text, "__", "u"));
        } else if smallest == code {
            parse_code(line, &mut output, smallest);
        } else {
        }
    }
    output
}

fn parse_code(line: &str, output: &mut String, smallest: Option<usize>) {
    let idx = smallest.unwrap();
    output.push_str(&line[..idx]);
    let mut code_contents = line[idx + 2..].to_string();
    let mut tag_contents = String::new();
    let mut leftovers = "";
    while let Some(idx) = code_contents.find("</>") {
        let closing_tag = &code_contents[idx - 1..idx + 3];
        if closing_tag == "\\</>" {
            code_contents = code_contents.replacen("\\</>", "\u{E000}", 1);
        } else {
            tag_contents.push_str(&code_contents[..idx]);
            leftovers = &code_contents[idx..];
            break;
        }
    }
    output.push_str(&format!("<code>{}</code>", tag_contents));
    output.push_str(&parse_text(leftovers));
}
