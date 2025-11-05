#[path = "../src/parser.rs"]
mod parser;

#[test]
pub fn code() {
    parser_test! {
        "<>code</>"
        =>
        "<code>code</code> "
    }
}

#[test]
pub fn code_with_escape() {
    parser_test! {
        "<>code \\</> morecode</>"
        =>
        "<code>code &lt;.&gt;</code> "
    }
}
