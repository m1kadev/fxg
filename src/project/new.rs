use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::error::Error;

use super::ProjectMeta;

#[allow(dead_code)] // ????????
#[cfg(feature = "developer")]
pub const TEMPLATE_FXG: &str = include_str!("index.fxg");

#[cfg(feature = "developer")]
pub const TEMPLATE_HTML: &str = include_str!("template.html");

#[cfg(feature = "developer")]
pub const DOG_IMAGE: &[u8] = include_bytes!("dog.png");

#[cfg(feature = "developer")]
pub fn new(root_folder: PathBuf) -> Result<(), Error> {
    let mut path = root_folder.clone();
    fs::create_dir(root_folder)?;

    // write config.yml
    path.push("config.yml");
    let mut config = File::create(&path)?;
    config.write_all(serde_yaml::to_string(&ProjectMeta::default())?.as_bytes())?;
    path.pop();
    drop(config);

    // write template.html
    path.push("template.html");
    let mut dog_image = File::create(&path)?;
    dog_image.write_all(TEMPLATE_HTML.as_bytes())?;
    path.pop();
    drop(dog_image);

    // create static/
    path.push("static");
    fs::create_dir(&path)?;

    // write static/dog.png
    path.push("dog.png");
    let mut dog_image = File::create(&path)?;
    dog_image.write_all(DOG_IMAGE)?;
    drop(dog_image);
    path.pop();
    path.pop();

    // create src/
    path.push("src");
    fs::create_dir(&path)?;

    // write src/index.png
    path.push("index.fxg");
    let mut index = File::create(&path)?;
    index.write_all(TEMPLATE_FXG.as_bytes())?;
    drop(index);
    path.pop();
    path.pop();

    // create out/
    path.push("out");
    fs::create_dir(&path)?;
    Ok(())
}
