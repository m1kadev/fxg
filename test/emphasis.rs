use crate::parser;

// ? NOTE: the space at the end is intended

parser_tests! {
    [cursive] { "//cursive//" -> "<em>cursive</em>" };
    [bold] { "!!bold!!" -> "<strong>bold</strong>" };
    [underline] { "__underline__" -> "<u>underline</u>" };

    [cursive_with_escape] { "//cursive \\// still cursive//" -> "<em>cursive // still cursive</em>" };
    [bold_with_escape] { "!!bold \\!! still bold!!" -> "<strong>bold !! still bold</strong>" };
    [underline_with_escape] { "__underline \\__ still underline__" -> "<u>underline __ still underline</u>" };

    [cursive_no_format] { "//cursive" -> "//cursive" };
    [bold_no_format] { "!!bold" -> "!!bold" };
    [underline_no_format] { "__underline" -> "__underline" };

    [cursive_with_escaped_final_tag] { "//cursive \\//" -> "//cursive //" };
    [bold_with_escaped_final_tag] { "!!bold \\!!" -> "!!bold !!" };
    [underline_with_escaped_final_tag] { "__underline \\__" -> "__underline __" };
}
