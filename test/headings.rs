use crate::parser;

parser_tests! {
    [header_1] { "= Heading 1 ="           -> "<h1>Heading 1</h1>" };
    [header_2] { "== Heading 2 =="         -> "<h2>Heading 2</h2>" };
    [header_3] { "=== Heading 3 ==="       -> "<h3>Heading 3</h3>" };
    [header_4] { "==== Heading 4 ===="     -> "<h4>Heading 4</h4>" };
    [header_5] { "===== Heading 5 ====="   -> "<h5>Heading 5</h5>" };
    [header_6] { "====== Heading 6 ======" -> "<h6>Heading 6</h6>" };

    [excess_equals_signs] { "======== Heading 8 ========" ->  "======== Heading 8 ========" };
    [fully_unclosed_header] { "== Heading 2" -> "== Heading 2" };
    [partially_unclosed_header] { "== Heading 2 =" -> "== Heading 2 =" };

    [empty_header] { "= =" -> "<h1></h1>" };
    [empty_header_3] { "=== ===" -> "<h3></h3>" };

}
