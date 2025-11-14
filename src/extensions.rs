use crate::escape;

pub trait HtmlWriting {
    fn write_tag(&mut self, tag: &str, contents: &str);
    fn write_opening_tag(&mut self, tag: &str);
    fn write_closing_tag(&mut self, tag: &str);
    fn write_opening_tag_class(&mut self, tag: &str, class: &str);
}

impl HtmlWriting for String {
    #[inline(always)]
    fn write_opening_tag(&mut self, tag: &str) {
        self.push_str(escape!("<"));
        self.push_str(tag);
        self.push_str(escape!(">"));
    }

    #[inline(always)]
    fn write_opening_tag_class(&mut self, tag: &str, class: &str) {
        self.push_str(escape!("<"));
        self.push_str(tag);
        self.push_str(" class=\"");
        self.push_str(class);
        self.push_str("\"");

        self.push_str(escape!(">"));
    }

    #[inline(always)]
    fn write_closing_tag(&mut self, tag: &str) {
        self.push_str(escape!("<"));
        self.push_str("/");
        self.push_str(tag);
        self.push_str(escape!(">"));
    }

    #[inline(always)]
    fn write_tag(&mut self, tag: &str, contents: &str) {
        self.write_opening_tag(tag);
        self.push_str(contents);
        self.write_closing_tag(tag);
    }
}
