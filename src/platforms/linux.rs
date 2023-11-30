use crate::utils;
use dirs::home_dir;
use tokio::fs::{create_dir_all, remove_file, File};
use tokio::io::{AsyncWriteExt, Error, ErrorKind};
use std::path::Path;
use utils::font::FontData;

const FONT_DIR_TTF: &str = "/usr/share/fonts/truetype";
const FONT_DIR_OTF: &str = "/usr/share/fonts/opentype";
const HOME_FONT_DIR_TTF: &str = ".fonts/truetype";
const HOME_FONT_DIR_OTF: &str = ".fonts/opentype";

pub async fn install(font: &FontData, user_dir: bool, timestamp: String) -> Result<(), Error> {
    let font_dir = match font.mime_type.as_str() {
        "font/ttf" => {
            if user_dir {
                home_dir().unwrap().join(HOME_FONT_DIR_TTF).join(timestamp)
            } else {
                Path::new(FONT_DIR_TTF).to_path_buf().join(timestamp)
            }
        },
        "font/otf" => {
            if user_dir {
                home_dir().unwrap().join(HOME_FONT_DIR_OTF).join(timestamp)
            } else {
                Path::new(FONT_DIR_OTF).to_path_buf().join(timestamp)
            }
        },
        _ => return Err(Error::new(ErrorKind::Other, "Unsupported font type")),
    };

    let target = Path::new(&font_dir).join(&font.file_name);
    match target.parent() {
        Some(parent) => {
            create_dir_all(parent).await?;
            let mut file = File::create(&target).await?;
            file.write_all(&font.data).await?;
            Ok(())
        }
        None => Err(Error::new(ErrorKind::Other, "No parent directory")),
    }
}

pub async fn uninstall(font: &FontData) -> Result<(), Error> {
    let target = Path::new(&font.file_path);
    if target.exists() {
        remove_file(&target).await?;
    }
    Ok(())
}
