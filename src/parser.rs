use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::{Captures, Regex};

struct LineFormatRegexes {
    code: Regex,
    bold: Regex,
    italicised: Regex,
    underlined: Regex,
}

// TODO: replace regexes witha cutal code cause this isnt working

pub fn parse(reader: &mut BufReader<File>) -> String {
    let regexes = LineFormatRegexes {
        code: Regex::new(r"<>(?<contents>.*?(\\\\|[^\\]))<\/>").unwrap(),
        italicised: Regex::new(r"\/\/(?<contents>.+?)\/\/").unwrap(),
        bold: Regex::new(r"!!(?<contents>.+?)!!").unwrap(),
        underlined: Regex::new(r"__(?<contents>.+?)__").unwrap(),
    };
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
            parse_title(line, &mut output, &regexes);
            last_line_was_title = true;
        } else if line.is_empty() && !last_line_was_title {
            output += "\u{E000}br/\u{E001}";
        } else {
            parse_text(line, &mut output, &regexes);
            last_line_was_title = false;
        }
        lnbuf.clear();
    }
    as_escaped_html(&output)
}

fn parse_title(line: &str, output: &mut String, regexes: &LineFormatRegexes) {
    let (prefix, _) = line.split_once(' ').unwrap();
    let header_size = prefix.len();
    // header syntax is complete

    if line.ends_with(&str::repeat("=", header_size)) {
        output.push_str("\u{E000}h");
        output.push_str(&header_size.to_string());
        output.push('\u{E001}');
        let header_contents = &line[header_size..line.len() - 1 - header_size].trim();
        parse_text(header_contents, output, regexes);
        output.push_str("\u{E000}/h");
        output.push_str(&header_size.to_string());
        output.push('\u{E001}');
    } else {
        // parse the text normally
        parse_text(line, output, regexes);
    }
}

fn parse_text(line: &str, output: &mut String, regexes: &LineFormatRegexes) {
    let with_codeblocks_parsed = regexes.code.replace_all(line, |caps: &Captures| {
        format!(
            "\u{E000}code\u{E001}{}\u{E000}/code\u{E001}",
            as_escaped_fxg(&caps["contents"])
        )
    });
    let with_bold_parsed = regexes.bold.replace_all(
        &with_codeblocks_parsed,
        "\u{E000}strong\u{E001}$contents\u{E000}/strong\u{E001}",
    );
    let with_italicised_parsed = regexes.italicised.replace_all(
        &with_bold_parsed,
        "\u{E000}em\u{E001}$contents\u{E000}/em\u{E001}",
    );
    let with_underline_parsed = regexes.underlined.replace_all(
        &with_italicised_parsed,
        "\u{E000}u\u{E001}$contents\u{E000}/u\u{E001}",
    );

    output.push_str(&with_underline_parsed);
    output.push(' ');
}

fn as_escaped_fxg(input: &str) -> String {
    //println!("escaping {input}");
    input
        .replace("\\\\\\</>", "\u{E009}")
        .replace("\\</>", "\u{E008}")
        .replace('\\', "\u{E002}")
        .replace('<', "\u{E003}")
        .replace('>', "\u{E004}")
        .replace('/', "\u{E005}")
        .replace('_', "\u{E006}")
        .replace('!', "\u{E007}")
}

fn as_escaped_html(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace("\\<", "&lt;")
        .replace(">", "&gt;")
        .replace("\\>", "&gt;")
        .replace("\"", "&qout;")
        .replace("\\\\", "\\")
        .replace("\\!", "!")
        .replace("\\_", "_")
        .replace("\\/", "/")
        .replace("\u{E000}", "<")
        .replace("\u{E001}", ">")
        .replace("\u{E002}", "\\")
        .replace("\u{E003}", "&lt;")
        .replace("\u{E004}", "&gt;")
        .replace("\u{E005}", "/")
        .replace("\u{E006}", "_")
        .replace("\u{E007}", "!")
        .replace("\u{E008}", "&lt;/&gt;")
        .replace("\u{E009}", "\\&lt;/&gt;")
}
