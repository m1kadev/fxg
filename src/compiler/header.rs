use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DocumentHeader {
    pub ogp: OgpData,
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct OgpData {
    #[serde(rename = "type")]
    pub typ: String,
    pub description: String,

    pub image: Option<Image>,
}

#[derive(Deserialize, Serialize)]
pub struct Image {
    url: String,
    width: Option<usize>,
    height: Option<usize>,
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
            if let Some(width) = &img.width {
                header.push_str(&format!(
                    r#"<meta property="og:image:width" content="{}">"#,
                    width
                ));
            }
            if let Some(height) = &img.height {
                header.push_str(&format!(
                    r#"<meta property="og:image:height" content="{}">"#,
                    height
                ));
            }
        }

        header
    }
}
