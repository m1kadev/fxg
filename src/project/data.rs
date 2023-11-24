use std::{borrow::Cow, fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectMeta {
    pub site_name: String,
    pub site_folder: String,
    pub source_folder: String,
    pub static_folder: String,
    pub output_folder: String,
    pub template: String,
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
        let metadata = serde_yaml::from_reader::<File, ProjectMeta>(
            File::open(&base_path)
                .map_err(|_| Error::Nice("No FXG project file was found.".into()))?,
        )?;
        base_path.pop();
        Ok(Self {
            metadata,
            base_path,
        })
    }

    #[inline]
    pub fn metadata(&self) -> &ProjectMeta {
        &self.metadata
    }

    fn collect_files(
        folder: PathBuf,
        collector: &mut Vec<PathBuf>,
        filter: &impl Fn(&Cow<'_, str>) -> bool,
    ) -> Result<(), crate::Error> {
        let read = folder.read_dir()?;
        for entry_r in read {
            let entry = entry_r?;
            if entry.metadata()?.is_file() {
                if let Some(v) = entry.path().extension().map(|x| x.to_string_lossy()) {
                    if filter(&v) {
                        collector.push(entry.path());
                    }
                } else {
                    continue; // the file didn't have an extension
                }
                continue; // the file doesn't match the extension
            } else {
                Self::collect_files(entry.path(), collector, filter)?;
            }
        }
        Ok(())
    }

    pub fn collect_documents(&self) -> Result<Vec<PathBuf>, Error> {
        let mut files = vec![];
        Self::collect_files(self.src_dir(), &mut files, &|ext| ext == "fxg")?;
        Ok(files)
    }

    pub fn collect_misc(&self) -> Result<Vec<PathBuf>, Error> {
        let mut files = vec![];
        Self::collect_files(self.src_dir(), &mut files, &|ext| ext != "fxg")?;
        Ok(files)
    }

    // pub fn collect_documents(&self) -> Result<Vec<PathBuf>, Error> {
    //     let mut files = vec![];
    //     Self::collect_files(self.src_dir(), &mut files, "fxg")?;
    //     Ok(files)
    // }

    #[inline]
    pub fn src_dir(&self) -> PathBuf {
        let mut path = self.base_path.clone();
        path.push(&self.metadata.source_folder);
        path
    }

    #[inline]
    pub fn dest_dir(&self) -> PathBuf {
        let mut path = self.base_path.clone();
        path.push(&self.metadata.output_folder);
        path
    }

    #[inline]
    pub fn static_dir(&self) -> PathBuf {
        let mut path = self.base_path.clone();
        path.push(&self.metadata.static_folder);
        path
    }

    #[inline]
    pub fn base_dir(&self) -> PathBuf {
        self.base_path.clone()
    }

    #[inline]
    pub fn template(&self) -> PathBuf {
        let mut path = self.base_path.clone();
        path.push(&self.metadata.template);
        path
    }
}
