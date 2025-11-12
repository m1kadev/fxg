use crate::parser;

parser_tests! {
    [link] { "<#https://example.com/ Example>" -> "<a href=\"https://example.com/\">Example</a>" };
    [image] { "<!https://example.com/img.jpg Example>" -> "<img src=\"https://example.com/img.jpg\" alt=\"Example\">" };

    [link_unclosed] { "<#not a link" -> "&lt;#not a link" };
    [image_unclosed] { "<!not an image" -> "&lt;!not an image" };

    [link_no_contents] { "<#https://google.com/>" -> "<a href=\"https://google.com/\">https://google.com/</a>" };

    [image_no_alt] { "<!https://example.com/>" -> "<img src=\"https://example.com/\">" };
}
