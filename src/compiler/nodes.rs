use const_format::formatc;

#[derive(Debug, PartialEq, Eq)]
pub enum DocumentNode {
    Header1(Vec<DocumentNode>),
    Header2(Vec<DocumentNode>),
    Header3(Vec<DocumentNode>),
    Header4(Vec<DocumentNode>),
    Header5(Vec<DocumentNode>),
    Header6(Vec<DocumentNode>),

    Bold(Vec<DocumentNode>),
    Italic(Vec<DocumentNode>),
    Underline(Vec<DocumentNode>),

    Link { text: String, href: String },
    Image { alt: String, src: String },

    Text(String),

    LineBreak,
    Eof,
}

impl DocumentNode {
    pub fn as_html(&self, output: &mut String) {
        macro_rules! html_tag {
            (<$opening:ident $($attr:ident),* > { $nodes:ident } </$closing:ident>) => {{
                output.push_str(formatc!("<{}", stringify!($opening)));
                $( output.push_str(&format!(" {}={:?} ", stringify!($attr), $attr)); )*
                output.push('>');

                for node in $nodes {
                    node.as_html(output);
                }

                output.push_str(formatc!("</{}>", stringify!($closing)));
            }};

            (<$opening:ident $($attr:ident),* > {{ $node:ident }} </$closing:ident>) => {{
                output.push_str(formatc!("<{}", stringify!($opening)));
                $( output.push_str(&format!(" {}={:?} ", stringify!($attr), $attr)); )*
                output.push('>');
                output.push_str(&$node);

                output.push_str(formatc!("</{}>", stringify!($closing)));
            }};

            (<$opening:ident $($attr:ident),* />) => {{
                output.push_str(formatc!("<{}", stringify!($opening)));
                $( output.push_str(&format!(" {}={:?} ", stringify!($attr), $attr)); )*
                output.push_str("/>");
            }};

            (<$tag_name:ident/>) => {
                output.push_str(formatc!("<{}/>", stringify!($tag_name)))
            }
        }

        match self {
            Self::Header1(nodes) => html_tag!(<h1>{nodes}</h1>),
            Self::Header2(nodes) => html_tag!(<h2>{nodes}</h2>),
            Self::Header3(nodes) => html_tag!(<h3>{nodes}</h3>),
            Self::Header4(nodes) => html_tag!(<h4>{nodes}</h4>),
            Self::Header5(nodes) => html_tag!(<h5>{nodes}</h5>),
            Self::Header6(nodes) => html_tag!(<h6>{nodes}</h6>),

            Self::Bold(nodes) => html_tag!(<b>{nodes}</b>),
            Self::Italic(nodes) => html_tag!(<i>{nodes}</i>),
            Self::Underline(nodes) => html_tag!(<u>{nodes}</u>),

            Self::Image { alt, src } => html_tag!(<img src, alt />),
            Self::Link { text, href } => html_tag!(<a href> {{text}} </a>),

            Self::LineBreak => html_tag!(<br/>),

            Self::Text(text) => output.push_str(text),

            Self::Eof => {}
        }
    }
}
