// TODO: macro for repeated push_str calls

use std::io::{BufRead, BufReader};

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
    "__" => "\u{E004}",
    "!!" => "\u{E005}",
    "</>" => "\u{E006}",
    "\"" => "\u{E007}",
    "\\" => "\u{E008}"
};

pub fn parse<T>(reader: &mut BufReader<T>) -> String
where
    T: std::io::Read,
{
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

    output = output
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;");
    //.replace("\"", "&qout;");

    for (key, placeholder) in UNICODE_PLACEHOLDERS.entries() {
        if *key == "</>" {
            output = output.replace(placeholder, "&lt;/&gt;");
        } else {
            output = output.replace(placeholder, key);
        }
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
            found = false;
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
    // tags above <h6> don't exist
    if header_size > 6 {
        output.push_str(&parse_text(line));
        return output;
    }

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
    let link = line.find("<#");
    let image = line.find("<!");
    let smallest = [cursive, bold, underline, code, link, image]
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
            let idx = smallest.unwrap();
            let text = &line[idx + 2..];
            output.push_str(&line[..idx]);
            output.push_str(&parse_code(text));
        } else if smallest == link {
            let idx = smallest.unwrap();
            output.push_str(&line[..idx]);
            let data = &line[idx + 2..];
            if let Some(idx_end) = data.find(">") {
                let contents = &data[..idx_end];
                if let Some((link, desc)) = contents.split_once(" ") {
                    output.push_str(escape!("<"));
                    output.push_str("a href=");
                    output.push_str(escape!("\""));
                    output.push_str(link);
                    output.push_str(escape!("\""));
                    output.push_str(escape!(">"));
                    output.push_str(&parse_text(desc));
                    output.push_str(escape!("<"));
                    output.push_str("/a");
                    output.push_str(escape!(">"));
                    output.push_str(&parse_text(&data[idx_end + 1..]));
                } else {
                    output.push_str(escape!("<"));
                    output.push_str("a href=");
                    output.push_str(escape!("\""));
                    output.push_str(contents);
                    output.push_str(escape!("\""));
                    output.push_str(escape!(">"));
                    output.push_str(contents);
                    output.push_str(escape!("<"));
                    output.push_str("/a");
                    output.push_str(escape!(">"));
                    output.push_str(&parse_text(&data[idx_end + 1..]));
                }
            } else {
                output.push_str("<#");
                output.push_str(&parse_text(&line[idx + 2..]));
            }
        } else if smallest == image {
            let idx = smallest.unwrap();
            output.push_str(&line[..idx]);
            let data = &line[idx + 2..];
            if let Some(idx_end) = data.find(">") {
                let contents = &data[..idx_end];
                if let Some((link, alt)) = contents.split_once(" ") {
                    output.push_str(escape!("<"));
                    output.push_str("img src=");
                    output.push_str(escape!("\""));
                    output.push_str(link);
                    output.push_str(escape!("\""));
                    output.push_str(" alt=");
                    output.push_str(escape!("\""));
                    output.push_str(alt);
                    output.push_str(escape!("\""));
                    output.push_str(escape!(">"));
                    output.push_str(&parse_text(&data[idx_end + 1..]));
                } else {
                    output.push_str(escape!("<"));
                    output.push_str("img src=");
                    output.push_str(escape!("\""));
                    output.push_str(contents);
                    output.push_str(escape!("\""));
                    output.push_str(escape!(">"));
                }
            } else {
                output.push_str("<!");
                output.push_str(&parse_text(&line[idx + 2..]));
            }
        }
    }
    output
}

fn parse_code(line_dat: &str) -> String {
    let mut output = String::new();
    let mut line = line_dat.to_string();
    let mut tag_contents = "";
    let mut tag_remainder = "";
    while let Some(idx) = line.find("</>") {
        // 92 is the backslash codepoint
        if line.as_bytes()[idx - 1] == 92 {
            // if the character before THAT is also a backslash
            if idx > 1 && line.as_bytes()[idx - 2] == 92 {
                // triple backslash; edge case handling purely to allow anything to be inside code tags.
                if idx > 2 && line.as_bytes()[idx - 3] == 92 {
                    line.replace_range(idx - 3..idx - 1, escape!("\\"));
                    continue;
                } else {
                    line.replace_range(idx - 2..idx, escape!("\\"));
                }
            } else {
                line = line.replacen("\\</>", escape!("</>"), 1);
            }
        } else {
            tag_contents = &line[..idx];
            tag_remainder = &line[idx + 3..];
            break;
        }
    }

    output.push_str(escape!("<"));
    output.push_str("code");
    output.push_str(escape!(">"));
    output.push_str(tag_contents);
    output.push_str(escape!("<"));
    output.push_str("/code");
    output.push_str(escape!(">"));
    output.push_str(&parse_text(tag_remainder));
    output
}
