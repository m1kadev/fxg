#[path = "../src/parser.rs"]
mod parser;

parser_tests! {
    [code] { "<>code</>" -> "<code>code</code>" };
    [code_with_escape] { "<>code \\</> with escape</>" -> "<code>code &lt;/&gt; with escape</code>" };

    [code_only_tag] { "<>\\</></>" -> "<code>&lt;/&gt;</code>" };
    [code_only_backslash] { "<>\\\\</>" -> "<code>\\</code>" };
    // this is an edge case
    [code_only_escaped_tag] { "<>\\\\\\</></>" -> "<code>\\&lt;/&gt;</code>" };
    // ususally, this suffices
    [code_escaped_tag_in_context] { "<>\\ test \\</> yayay</>" -> "<code>\\ test &lt;/&gt; yayay</code>" };

}
