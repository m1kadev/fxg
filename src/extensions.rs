use crate::escape;

type TagData<'a> = &'a [(&'a str, &'a str)];

pub trait HtmlWriting {
    fn write_tag(&mut self, tag: &str, contents: &str, tag_data: TagData);
    fn write_opening_tag(&mut self, tag: &str, tag_data: TagData);
    fn write_closing_tag(&mut self, tag: &str);
}

impl HtmlWriting for String {
    #[inline(always)]
    fn write_opening_tag(&mut self, tag: &str, tag_data: TagData) {
        self.push_str(escape!("<"));
        self.push_str(tag);
        for data in tag_data {
            self.push(' ');
            self.push_str(data.0);
            self.push('=');
            self.push_str(escape!("\""));
            self.push_str(data.1);
            self.push_str(escape!("\""));
        }
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
    fn write_tag(&mut self, tag: &str, contents: &str, tag_data: TagData) {
        self.write_opening_tag(tag, tag_data);
        self.push_str(contents);
        self.write_closing_tag(tag);
    }
}
