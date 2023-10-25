use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::error::Error;

use super::Project;

#[allow(dead_code)] // ????????
pub const TEMPLATE: &str = r#"---
---
title: "Index"
ogp:
  type: website
  description: "New website"
---


= Heading 1 =
== Heading 2 ==
=== Heading 3 ===
==== Heading 4 ====
===== Heading 5 =====
====== Heading 6 ======

//italic// !!bold!! __underline__

this is an image:
<!/static/dog.png>

this is an image with alt text:
<!/static/dog.png Dog>

This is a <https://github.com/zTags/fxg link>!

[[ No formatting here! ]]

This text will be on

different lines.

This text will be on
the same line.
"#;

pub const DOG_IMAGE: &[u8] = include_bytes!("../../dog.png");

pub fn new(root_folder: PathBuf) -> Result<(), Error> {
    let mut path = root_folder.clone();
    fs::create_dir(root_folder)?;
    path.push("config.yml");

    let mut config = File::create(&path)?;
    config.write_all(serde_yaml::to_string(&Project::default())?.as_bytes())?;
    drop(config);

    path.pop();
    path.push("static");
    fs::create_dir(&path)?;
    path.push("dog.png");
    let mut dog_image = File::create(&path)?;
    dog_image.write_all(DOG_IMAGE)?;
    drop(dog_image);
    path.pop();
    path.pop();

    path.push("src");
    fs::create_dir(&path)?;
    path.push("index.fxg");
    let mut index = File::create(&path)?;
    index.write_all(TEMPLATE.as_bytes())?;
    drop(index);

    Ok(())
}
