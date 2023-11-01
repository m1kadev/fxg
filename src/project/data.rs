use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Serialize, Deserialize)]
pub struct ProjectMeta {
    site_name: String,
    site_folder: String,
    source_folder: String,
    static_folder: String,
    output_folder: String,
    template: String,
}

impl Default for ProjectMeta {
    fn default() -> Self {
        Self {
            site_name: "https://example.com/".to_string(),
            site_folder: "/".to_string(),
            source_folder: "src".to_string(),
            static_folder: "static".to_string(),
            output_folder: "out".to_string(),
            template: "template.html".to_string(),
        }
    }
}

pub struct Project {
    metadata: ProjectMeta,
    base_path: PathBuf,
}

impl Project {
    pub fn from_dir(mut base_path: PathBuf) -> Result<Self, Error> {
        base_path.push("config.yml");
        let metadata = serde_yaml::from_reader::<File, ProjectMeta>(File::open(&base_path)?)?;
        base_path.pop();
        Ok(Self {
            metadata,
            base_path,
        })
    }

    fn collect_files(
        folder: PathBuf,
        collector: &mut Vec<PathBuf>,
        extension: &'static str,
    ) -> Result<(), crate::Error> {
        let read = folder.read_dir()?;
        for entry_r in read {
            let entry = entry_r?;
            if entry.metadata()?.is_file() {
                if let Some(v) = entry.path().extension().map(|x| x.to_string_lossy()) {
                    if v != extension {
                        continue; // the file doesn't match the extension
                    }
                } else {
                    continue; // the file didn't have an extension
                }
                collector.push(entry.path());
            } else {
                Self::collect_files(entry.path(), collector, extension)?;
            }
        }
        Ok(())
    }

    pub fn collect_documents(&self, extension: &'static str) -> Result<Vec<PathBuf>, Error> {
        let mut files = vec![];
        Self::collect_files(self.src_dir(), &mut files, extension)?;

        Ok(files)
    }

    pub fn src_dir(&self) -> PathBuf {
        let mut path = self.base_path.clone();
        path.push(&self.metadata.source_folder);
        path
    }

    pub fn dest_dir(&self) -> PathBuf {
        let mut path = self.base_path.clone();
        path.push(&self.metadata.output_folder);
        path
    }

    pub fn static_dir(&self) -> PathBuf {
        let mut path = self.base_path.clone();
        path.push(&self.metadata.static_folder);
        path
    }

    pub fn base_dir(&self) -> PathBuf {
        self.base_path.clone()
    }

    pub fn read_template(&self) -> Result<String, io::Error> {
        let mut path = self.base_path.clone();
        path.push(&self.metadata.template);
        fs::read_to_string(path)
    }
}
