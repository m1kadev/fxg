use serde::{Deserialize, Serialize};

mod new;

pub use new::new;

#[derive(Serialize, Deserialize)]
pub struct Project {
    site_name: String,
    site_folder: String,
    source_folder: String,
    static_folder: String,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            site_name: "https://example.com".to_string(),
            site_folder: "/".to_string(),
            source_folder: "src".to_string(),
            static_folder: "static".to_string(),
        }
    }
}
