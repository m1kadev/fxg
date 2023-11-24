use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct DocumentHeader {
    pub ogp: OgpData,
    pub title: String,
    pub tags: Vec<String>,
    pub date: SystemTime,
    pub summary: String,
    pub author: String,
}

impl Default for DocumentHeader {
    fn default() -> Self {
        Self {
            date: SystemTime::now(),
            title: "".into(),
            tags: vec![],
            ogp: OgpData {
                typ: "".into(),
                description: "".into(),
                image: None,
            },
            summary: "".into(),
            author: "".into(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct OgpData {
    #[serde(rename = "type")]
    pub typ: String,
    pub description: String,

    pub image: Option<Image>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Image {
    url: String,
    width: usize,
    height: usize,
    alt: Option<String>,
}

impl OgpData {
    pub fn build_ogp(&self, title: &str, site_name: &str, url: &str) -> String {
        let mut header = format!(
            r#"
<meta property="og:type" content="{}">
<meta property="og:title" content="{}">
<meta property="og:site_name" content="{}">
<meta property="og:description" content="{}">
<meta property="og:url" content="{}">
    "#,
            self.typ, title, site_name, self.description, url
        );

        if let Some(img) = &self.image {
            header.push_str(&format!(
                r#"<meta property="og:image" content="{}">"#,
                img.url
            ));
            if let Some(alt) = &img.alt {
                header.push_str(&format!(
                    r#"<meta property="og:image:alt" content="{}">"#,
                    alt
                ));
            }
            header.push_str(&format!(
                r#"<meta property="og:image:width" content="{}">"#,
                img.width
            ));
            header.push_str(&format!(
                r#"<meta property="og:image:height" content="{}">"#,
                img.height
            ));
        }

        header
    }
}
