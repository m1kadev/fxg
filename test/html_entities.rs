use crate::parser;

parser_tests! {
    [html_entities_amp] { "&" -> "&amp;" };
    // [html_entities_qout] { "\"" -> "&qout;" };
    [html_entities_lt] { "<" -> "&lt;" };
    [html_entities_gt] { " >" -> "&gt;" };
}
