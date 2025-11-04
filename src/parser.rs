use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

// ! ONLY VALID WITH parse_text IN SCOPE
macro_rules! markup {
    ($input:ident, $markup:literal - $markup_end:literal ! $escape:literal -> <$html_tag:ident>) => {{
        let mut tag_contents = String::new();
        while let Some(idx) = $input.find($markup_end) {
            // not escaped; end of input
            if &$input[idx - 1..idx] != "\\" {
                tag_contents.push_str(&$input[..idx]);
                break;
            } else {
                // character is escaped, so
                $input = $input.replacen($escape, "\u{E000}", 1);
            }
        }
        // parse the contents again for tested markup
        tag_contents = tag_contents.trim().to_string();
        let parsed = parse_text(&tag_contents);
        format!(
            "<{}>{}</{}>",
            stringify!($html_tag),
            parsed.replace("\u{E000}", $markup),
            stringify!($html_tag)
        )
    }};
}

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
            output += "\u{E001}br/\u{E002}";
        } else {
            output.push_str(&parse_text(line));
            output.push(' ');
            last_line_was_title = false;
        }
        lnbuf.clear();
    }

    output.replace("\u{E001}", "<").replace("\u{E002}", ">")
}

fn parse_title(line: &str) -> String {
    let mut output = String::new();
    let (prefix, _) = line.split_once(' ').unwrap();
    let header_size = prefix.len();
    // header syntax is complete

    if line.ends_with(&str::repeat("=", header_size)) {
        output.push_str("\u{E001}h");
        output.push_str(&header_size.to_string());
        output.push('\u{E002}');
        let header_contents = &line[header_size..line.len() - 1 - header_size].trim();
        output.push_str(&parse_text(header_contents));
        output.push_str("\u{E001}/h");
        output.push_str(&header_size.to_string());
        output.push('\u{E002}');
    } else {
        // parse the text normally
        output.push_str(&parse_text(line));
    }
    return output;
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
        if smallest == cursive {
            let idx = smallest.unwrap();
            output.push_str(&line[..idx]);
            let mut fmt_text = line[idx + 2..].to_string();
            let compiled = markup!(fmt_text, "//" - "//" ! "\\//" -> <em>);
            output.push_str(&compiled);
            let text_length = compiled.len() - 9;
            output.push_str(&parse_text(&fmt_text[text_length + 2..]));
        } else if smallest == bold {
            let idx = smallest.unwrap();
            output.push_str(&line[..idx]);
            let mut fmt_text = line[idx + 2..].to_string();
            let compiled = markup!(fmt_text, "!!" - "!!" ! "\\!!" -> <strong>);
            output.push_str(&compiled);
            let text_length = compiled.len() - 17;
            output.push_str(&parse_text(&fmt_text[text_length + 2..]));
        } else if smallest == underline {
            let idx = smallest.unwrap();
            output.push_str(&line[..idx]);
            let mut fmt_text = line[idx + 2..].to_string();
            let compiled = markup!(fmt_text, "__" - "__" ! "\\__" -> <u>);
            output.push_str(&compiled);
            let text_length = compiled.len() - 7;
            output.push_str(&parse_text(&fmt_text[text_length + 2..]));
        } else if smallest == code {
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
        } else {
        }
    }

    // if let Some(idx) = line.find("//") {
    //     // anything before is unformatted

    // } else if let Some(idx) = line.find("!!") {
    //     output.push_str(&line[..idx]);
    //     let mut fmt_text = line[idx + 2..].to_string();
    //     output.push_str(&markup!(fmt_text, "!!" - "!!" ! "\\!!" -> <strong>));
    // } else if let Some(idx) = line.find("__") {
    //     output.push_str(&line[..idx]);
    //     let mut fmt_text = line[idx + 2..].to_string();
    //     output.push_str(&markup!(fmt_text, "__" - "__" ! "\\__" -> <u>));
    // } else if let Some(idx) = line.find("<>") {
    //     // code is a special case
    //     output.push_str(&line[..idx]);
    //     let mut fmt_text = line[idx + 2..].to_string();
    //     while let Some(idx) = fmt_text.find("</>") {
    //         dbg!(&fmt_text[..idx + 3]);
    //         if &fmt_text[idx - 1..idx] != "\\" {
    //             break;
    //         } else {
    //             fmt_text = fmt_text.replacen("\\</>", "\u{e000}", 1);
    //         }
    //     }

    //     output.push_str(&format!("<code>{}</code>", fmt_text[..idx].to_string()));
    // } else {
    //     output.push_str(line);
    // }
    output
}
