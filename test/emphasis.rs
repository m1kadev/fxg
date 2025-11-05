#[path = "../src/parser.rs"]
mod parser;

// ? NOTE: the space at the end is intended

#[test]
pub fn cursive() {
    parser_test! {
        "//cursive//"
        =>
        "<em>cursive</em> "
    }
}

#[test]
pub fn bold() {
    parser_test! {
        "!!bold!!"
        =>
        "<strong>bold</strong> "
    }
}

#[test]
pub fn underline() {
    parser_test! {
        "__underlined__"
        =>
        "<u>underlined</u> "
    }
}

#[test]
pub fn cursive_with_escape() {
    parser_test! {
        "//cursive \\// escaped//"
        =>
        "<em>cursive // escaped</em> "
    }
}

#[test]
pub fn bold_with_escape() {
    parser_test! {
        "!!bold \\!! escaped!!"
        =>
        "<strong>bold !! escaped</strong> "
    }
}

#[test]
pub fn underline_with_escape() {
    parser_test! {
        "__underline \\__ escaped__"
        =>
        "<u>underline __ escaped</u> "
    }
}

#[test]
pub fn cursive_no_format() {
    parser_test! {
        "// cursive unformat"
        =>
        "// cursive unformat "
    }
}

#[test]
pub fn bold_no_format() {
    parser_test! {
        "!! bold unformat"
        =>
        "!! bold unformat "
    }
}

#[test]
pub fn underline_no_format() {
    parser_test! {
        "__ underline unformat"
        =>
        "__ underline unformat "
    }
}

#[test]
pub fn cursive_with_escaped_final_tag() {
    parser_test! {
        "//cursive unformat \\//"
        =>
        "//cursive unformat // "
    }
}

#[test]
pub fn bold_with_escaped_final_tag() {
    parser_test! {
        "!!bold unformat \\!!"
        =>
        "!!bold unformat !! "
    }
}

#[test]
pub fn underline_with_escaped_final_tag() {
    parser_test! {
        "__underline unformat \\__"
        =>
        "__underline unformat __ "
    }
}
