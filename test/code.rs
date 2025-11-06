#[path = "../src/parser.rs"]
mod parser;

parser_tests! {
    [code] { "<>code</>" -> "<code>code</code>" };
    [code_with_escape] { "<>code \\</> with escape</>" -> "<code>code &lt;/&gt; with escape</code>" };
}
