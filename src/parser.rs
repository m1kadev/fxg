// TODO: macro for repeated push_str calls

use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Read},
    pin,
};

use crate::{UNICODE_PLACEHOLDERS, blockqoutes::parse_blockqoute, escape, extensions::HtmlWriting};

const NUMERICS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const LOWERCASE_LETTERS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const UPPERCASE_LETTERS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const ROMAN_NUMERALS_UPPERCASE: &[char] = &['I', 'V', 'X', 'L', 'D', 'M'];
const ROMAN_NUMERALS_LOWERCASE: &[char] = &['i', 'v', 'x', 'l', 'd', 'm'];

#[derive(Debug, Clone, Copy)]
enum OrderedListMarker {
    Numerical,
    LowercaseLetters,
    UppercaseLetters,
    LowercaseNumerals,
    UppercaseNumerals,
}

impl OrderedListMarker {
    fn charset(&self) -> &[char] {
        match self {
            Self::LowercaseLetters => LOWERCASE_LETTERS,
            Self::LowercaseNumerals => ROMAN_NUMERALS_LOWERCASE,
            Self::Numerical => NUMERICS,
            Self::UppercaseLetters => UPPERCASE_LETTERS,
            Self::UppercaseNumerals => ROMAN_NUMERALS_UPPERCASE,
        }
    }

    fn html_type(&self) -> &str {
        match self {
            Self::LowercaseLetters => "a",
            Self::Numerical => "1",
            Self::UppercaseLetters => "A",
            Self::LowercaseNumerals => "i",
            Self::UppercaseNumerals => "I",
        }
    }
}

#[inline(always)]
fn string_consists_of(input: &str, set: &[char]) -> bool {
    input.chars().all(|c| set.contains(&c))
}

pub fn parse<T>(reader: &mut BufReader<T>) -> String
where
    T: std::io::Read,
{
    let mut output = String::new();
    let mut lnbuf = String::new();

    // output.write_opening_tag("div", &[("class", "fxg-content")]);

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
        } else if lnbuf.starts_with('-') {
            if &lnbuf[1..2] == " " {
                output.push_str(&parse_ul(reader, lnbuf.clone()));
                lnbuf.clear();
                continue;
            }
            let chars = line.chars().collect::<HashSet<char>>();
            if chars.len() == 1 && line.len() >= 3 {
                output.push_str(escape!("<"));
                output.push_str("hr");
                output.push_str(escape!(">"));
            } else {
                output.push_str(&parse_text(line));
                output.push(' ');
                last_line_was_title = false;
            }
        } else if lnbuf.starts_with('>') {
            let (blockqoute, _) = parse_blockqoute(reader, lnbuf.clone()); // ? elegance
            output.push_str(&blockqoute);
            last_line_was_title = false;
        } else if line.starts_with('<')
            && line.ends_with('>')
            && line[1..line.len() - 1].chars().all(char::is_alphabetic)
        {
            parse_codeblock(reader, &mut output, lnbuf.clone(), line);
        } else if lnbuf.starts_with('|') && line.ends_with('|') {
            let mut table = String::new();
            while let Ok(length) = reader.read_line(&mut lnbuf) {
                if length == 0 {
                    break;
                }
                let line = &lnbuf[lnbuf.len() - length..lnbuf.len()].trim();
                if line.starts_with('|') && line.ends_with('|') {
                    table.push_str(&lnbuf);
                } else {
                    break;
                }
                lnbuf.clear();
            }
            output.push_str(&parse_table(&table));
        } else {
            if let Some(pos) = lnbuf.find('.') {
                let marker = &lnbuf[..pos];
                if string_consists_of(marker, NUMERICS) {
                    output.push_str(&parse_ol(
                        reader,
                        lnbuf.clone(),
                        OrderedListMarker::Numerical,
                        1,
                    ));
                } else if string_consists_of(marker, ROMAN_NUMERALS_LOWERCASE) {
                    output.push_str(&parse_ol(
                        reader,
                        lnbuf.clone(),
                        OrderedListMarker::LowercaseNumerals,
                        1,
                    ));
                } else if string_consists_of(marker, ROMAN_NUMERALS_UPPERCASE) {
                    output.push_str(&parse_ol(
                        reader,
                        lnbuf.clone(),
                        OrderedListMarker::UppercaseNumerals,
                        1,
                    ));
                } else if string_consists_of(marker, LOWERCASE_LETTERS) {
                    output.push_str(&parse_ol(
                        reader,
                        lnbuf.clone(),
                        OrderedListMarker::LowercaseLetters,
                        1,
                    ));
                } else if string_consists_of(marker, UPPERCASE_LETTERS) {
                    output.push_str(&parse_ol(
                        reader,
                        lnbuf.clone(),
                        OrderedListMarker::UppercaseLetters,
                        1,
                    ));
                } else {
                    output.push_str(&parse_text(line));
                    output.push(' ');
                    last_line_was_title = false;
                }
            }
        }
        lnbuf.clear();
    }

    // output.write_closing_tag("div");

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

fn parse_codeblock<T>(reader: &mut BufReader<T>, output: &mut String, mut lnbuf: String, line: &str)
where
    T: Read,
{
    let lang = &line[1..line.len() - 1];
    if lang.is_empty() {
        output.write_opening_tag("pre", &[]);
        output.write_opening_tag("code", &[]);
    } else {
        output.write_opening_tag("pre", &[]);
        output.write_opening_tag("code", &[("class", &format!("language-{lang}"))]);
    }
    lnbuf.clear();
    while let Ok(length) = reader.read_line(&mut lnbuf) {
        if length == 0 {
            break;
        }
        if &lnbuf[lnbuf.len() - length..lnbuf.len() - 1] == "</>" {
            break;
        }
    }

    let least_indent = lnbuf
        .lines()
        .rev()
        .skip(1)
        .map(|line| line.find(|c: char| !c.is_whitespace()))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .min()
        .unwrap();

    let mut lines = lnbuf.lines().peekable();
    while let Some(line) = lines.next() {
        if let None = lines.peek() {
            break;
        }
        if line.is_empty() {
            output.push('\n');
        } else {
            output.push_str(&line[least_indent..]);
            output.push('\n');
        }
    }
    output.write_closing_tag("code");
    output.write_closing_tag("pre");
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

pub fn parse_text(line: &str) -> String {
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

fn parse_table(table: &str) -> String {
    let mut output = String::new();
    output.write_opening_tag("table", &[]);
    let mut is_header = true;
    let mut last_was_header = true;
    let mut rowbuf = String::new();
    for line in table.lines() {
        output.write_opening_tag("tr", &[]);
        for table_entry in line.split("|") {
            if table_entry.is_empty() {
                continue;
            }
            let entry_trimmed = table_entry.trim();
            if entry_trimmed.chars().all(|c| c == '-') && !entry_trimmed.is_empty() {
                is_header = false;
            }
            if is_header {
                rowbuf.write_tag("th", table_entry, &[]);
            } else {
                rowbuf.write_tag("td", table_entry, &[]);
            }
        }
        if !is_header && last_was_header {
            last_was_header = false;
            rowbuf.clear();
            continue;
        } else {
            if is_header {
                last_was_header = true;
            }
            output.push_str(&rowbuf);
        }
        output.write_closing_tag("tr");
        rowbuf.clear();
    }

    if is_header {
        output = output.replace("th", "td"); // all is data
    }
    output.write_closing_tag("table");

    output
}

fn parse_ul<T>(reader: &mut BufReader<T>, mut lnbuf: String) -> String
where
    T: Read,
{
    let mut output = String::new();

    while let Ok(length) = reader.read_line(&mut lnbuf) {
        let line = &lnbuf[lnbuf.len() - length..];

        if !line.starts_with("- ") || length == 0 {
            break;
        }
    }
    let list = lnbuf.trim().lines();
    let mut current_depth = 0;
    for item in list {
        let pre = item.find(|c| !&['-', ' '].contains(&c)).unwrap();
        let depth = item[..pre].chars().filter(|c| *c == '-').count();
        if current_depth < depth {
            let delta = depth - current_depth;
            for _ in 0..delta {
                // w3c fight me
                //output.write_opening_tag("li", &[]);
                output.write_opening_tag("ul", &[]);
            }
            current_depth = depth;
        } else if current_depth > depth {
            let delta = current_depth - depth;
            for _ in 0..delta {
                output.write_closing_tag("ul");
                //output.write_closing_tag("li");
            }
            current_depth = depth;
        }
        output.write_tag("li", &parse_li(&item[pre..]), &[]);
    }
    for _ in 0..current_depth {
        output.write_closing_tag("ul");
    }

    output
}

// ! REWRITE: remove code for other indexers, only allow numbered lists
fn parse_ol<T>(
    reader: &mut BufReader<T>,
    mut buffer: String,
    marker: OrderedListMarker,
    depth: usize,
) -> String
where
    T: Read,
{
    let mut output = String::new();
    let mut items = vec![];
    items.push(buffer.splitn(depth + 1, ".").last().unwrap().to_string());
    buffer.clear();
    while let Ok(read) = reader.read_line(&mut buffer) {
        if read == 0 {
            break;
        }
        // if let Some((indexer, item)) = buffer.split_once('.') {
        let mut splitn = buffer.splitn(depth + 1, '.').skip(depth - 1);
        let pindexer = splitn.next();
        let ptext = splitn.next();
        if let (Some(indexer), Some(item)) = (pindexer, ptext) {
            if !string_consists_of(indexer, marker.charset()) {
                break;
            } else {
                if let Some((potential_indexer, _)) = item.split_once('.') {
                    let indexer = potential_indexer.trim();
                    if string_consists_of(indexer, NUMERICS) {
                        items.push(parse_ol(reader, buffer.clone(), marker, depth + 1));
                    } else if string_consists_of(indexer, ROMAN_NUMERALS_UPPERCASE) {
                        items.push(parse_ol(reader, buffer.clone(), marker, depth + 1));
                    } else if string_consists_of(indexer, ROMAN_NUMERALS_LOWERCASE) {
                        items.push(parse_ol(reader, buffer.clone(), marker, depth + 1));
                    } else if string_consists_of(indexer, LOWERCASE_LETTERS) {
                        items.push(parse_ol(reader, buffer.clone(), marker, depth + 1));
                    } else if string_consists_of(indexer, UPPERCASE_LETTERS) {
                        items.push(parse_ol(reader, buffer.clone(), marker, depth + 1));
                    }
                }
                items.push(item.to_string());
            }
        } else {
            break;
        }
        buffer.clear();
    }
    output.write_opening_tag("ol", &[("type", marker.html_type())]);
    let html_output = items.iter().map(|x| x.trim().to_string());
    for item in html_output {
        output.write_tag("li", &item, &[]);
    }
    output.write_closing_tag("ol");

    output
}

fn parse_li(rli: &str) -> String {
    let li = rli.trim();
    let prefix = &li[..3];
    match prefix {
        "[ ]" => {
            let mut output = String::new();
            output.write_tag(
                "input",
                &parse_text(&li[3..]),
                &[("type", "checkbox"), ("class", "fxg-clitem")],
            );
            output
        }
        "[-]" => {
            let mut output = String::new();
            output.write_tag(
                "input",
                &parse_text(&li[3..]),
                &[
                    ("type", "checkbox"),
                    ("class", "fxg-indeterminate fxg-clitem"),
                ],
            );
            output
        }
        "[x]" => {
            let mut output = String::new();
            output.write_tag(
                "input",
                &parse_text(&li[3..]),
                &[
                    ("type", "checkbox"),
                    ("class", "fxg-indeterminate fxg-clitem"),
                    ("checked", ""),
                ],
            );
            output
        }
        _ => parse_text(li),
    }
}
