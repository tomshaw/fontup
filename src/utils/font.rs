use tokio::fs as async_fs;
use std::path::Path;
use ttf_parser::Face;

#[derive(Clone)]
pub struct FontData {
    pub font_name: String,
    pub file_name: String,
    pub file_path: String,
    pub mime_type: String,
    pub data: Vec<u8>,
}

impl FontData {
    #[allow(dead_code)]
    pub fn get_parent_folder(&self) -> Option<&str> {
        let path = Path::new(&self.file_path);
        path.parent()
            .and_then(|p| p.to_str())
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("Font Name: {}", self.font_name);
        println!("File Name: {}", self.file_name);
        println!("File Path: {}", self.file_path);
        println!("MIME Type: {}", self.mime_type);
    }
}

fn is_font_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => {
            let ext = ext.to_string_lossy().to_lowercase();
            ext == "ttf" || ext == "otf" || ext == "woff"
        }
        None => false,
    }
}

async fn is_font_file_content(path: &Path) -> bool {
    let data = async_fs::read(path).await.expect("Unable to read file");
    Face::parse(&data, 0).is_ok()
}

pub async fn create_font_data(path: &Path) -> Option<FontData> {
    if is_font_file(path) && is_font_file_content(path).await {
        let data = async_fs::read(path).await.expect("Unable to read file");
        let font_name = path.file_stem()?.to_string_lossy().into_owned();
        let file_name = path.file_name()?.to_string_lossy().into_owned();
        let file_path = path.to_string_lossy().into_owned();
        let mime_type = match path.extension()?.to_string_lossy().to_lowercase().as_str() {
            "ttf" => "font/ttf",
            "otf" => "font/otf",
            "woff" => "font/woff",
            _ => "application/octet-stream",
        }
        .to_string();

        Some(FontData {
            font_name,
            file_name,
            file_path,
            mime_type,
            data,
        })
    } else {
        None
    }
}
