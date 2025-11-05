use std::io::BufReader;

#[path = "../src/parser.rs"]
mod parser;

#[test]
pub fn header_1() {
    parser_test! {
        "= Heading 1 ="
        =>
        "<h1>Heading 1</h1>"
    }
}

#[test]
pub fn header_2() {
    parser_test! {
        "== Heading 2 =="
        =>
        "<h2>Heading 2</h2>"
    }
}

#[test]
pub fn header_3() {
    parser_test! {
        "=== Heading 3 ==="
        =>
        "<h3>Heading 3</h3>"
    }
}

#[test]
pub fn header_4() {
    parser_test! {
        "==== Heading 4 ===="
        =>
        "<h4>Heading 4</h4>"
    }
}

#[test]
pub fn header_5() {
    parser_test! {
        "===== Heading 5 ====="
        =>
        "<h5>Heading 5</h5>"
    }
}

#[test]
pub fn heading_6() {
    parser_test! {
        "====== Heading 6 ======"
        =>
        "<h6>Heading 6</h6>"
    }
}

#[test]
pub fn excess_equals_signs() {
    parser_test! {
        "======== Heading 8 ========"
        =>
        "======== Heading 8 ========"
    }
}

#[test]
pub fn fully_unclosed_header() {
    parser_test! {
        "== Heading 2"
        =>
        "== Heading 2"
    }
}

#[test]
pub fn partially_unclosed_header() {
    parser_test! {
        "== Heading 2 ="
        =>
        "== Heading 2 ="
    }
}
