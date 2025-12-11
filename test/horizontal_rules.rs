use crate::parser;

parser_tests! {
    [horizonal_rule] { "---" -> "<hr>" };
    [horizonal_rule_not_enough] { "--" -> "--" };
    [horizonal_rule_excess] { "------" -> "<hr>" };
    [horizonal_rule_ends_with] { "------ff" -> "------ff" };
    [horizonal_rule_starts_with] { "ff------" -> "ff------" };

}
