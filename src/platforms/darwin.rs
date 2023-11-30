use tokio::fs::{File, create_dir_all, remove_file};
use tokio::io::{AsyncWriteExt, Error, ErrorKind};
use std::path::Path;
use utils::font::FontData;
use crate::utils;

const FONT_DIR: &str = "/Library/Fonts";

pub async fn install(font: &FontData) -> Result<(), Error> {
    let target = Path::new(FONT_DIR).join(&font.file_name);
    match target.parent() {
        Some(parent) => {
            create_dir_all(parent).await?;
            let mut file = File::create(&target).await?;
            file.write_all(&font.data).await?;
            Ok(())
        },
        None => Err(Error::new(ErrorKind::Other, "No parent directory")),
    }
}

pub async fn uninstall(font: &FontData) -> Result<(), Error> {
    let target = Path::new(FONT_DIR).join(&font.file_name);
    if target.exists() {
        remove_file(&target).await?;
    }
    Ok(())
}
