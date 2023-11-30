extern crate winapi;
use tokio::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use winapi::um::winuser::{SendMessageW, HWND_BROADCAST, WM_FONTCHANGE};
use winapi::um::wingdi::{AddFontResourceW, RemoveFontResourceW};
use winreg::enums::*;
use winreg::RegKey;
use utils::font::FontData;
use crate::utils;

const FONT_DIR: &str = "C:\\Windows\\Fonts";

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

fn add_font_resource(file_path: &str) -> Result<(), &'static str> {
    let wide_file_path = to_wide(file_path);
    unsafe {
        if AddFontResourceW(wide_file_path.as_ptr()) == 0 {
            return Err("Failed to add font resource");
        }
    }
    Ok(())
}

fn remove_font_resource(file_path: &str) -> Result<(), &'static str> {
    let wide_file_path = to_wide(file_path);
    unsafe {
        if RemoveFontResourceW(wide_file_path.as_ptr()) == 0 {
            return Err("Failed to remove font resource");
        }
    }
    Ok(())
}

fn broadcast_font_change() -> Result<(), &'static str> {
    unsafe {
        if SendMessageW(HWND_BROADCAST, WM_FONTCHANGE, 0, 0) == 0 {
            return Err("Failed to broadcast font change");
        }
    }
    Ok(())
}

pub async fn install(font: &FontData) -> Result<(), Error> {
    let file_path = Path::new(FONT_DIR).join(&font.file_name);
    let reg_value = format!("{} {}", font.font_name, font.mime_type);

    if !file_path.exists() {
        match fs::write(&file_path, &font.data).await {
            Ok(_) => (),
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Failed to write file: {}", e))),
        }
    }    

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts", KEY_ALL_ACCESS)?;

    match subkey.get_value::<String, _>(reg_value.clone()) {
        Ok(_) => (),
        Err(_) => match subkey.set_value(reg_value, &font.file_name) {
            Ok(_) => (),
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Failed to set registry value: {}", e))),
        },
    }
    
    match add_font_resource(&font.file_path) {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }
    
    match broadcast_font_change() {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }

    Ok(())
}

pub async fn uninstall(font: &FontData) -> Result<(), Error> {
    let file_path = Path::new(FONT_DIR).join(&font.file_name);
    let reg_value = format!("{} {}", font.font_name, font.mime_type);

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts", KEY_ALL_ACCESS)?;

    let value_result = subkey.get_value::<String, _>(reg_value.clone());
    if let Err(_) = value_result {
        return Err(Error::new(ErrorKind::Other, "Error getting registry value"));
    }

    match subkey.delete_value(reg_value) {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("Failed to delete registry value: {}", e))),
    }
    
    if file_path.exists() {
        match fs::remove_file(&file_path).await {
            Ok(_) => (),
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Failed to remove file: {}", e))),
        }
    }    

    match remove_font_resource(&font.file_path) {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }
    
    match broadcast_font_change() {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }

    Ok(())
}

pub async fn temp_install(font: &FontData) -> Result<(), Error> {
    match add_font_resource(&font.file_path) {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }
    
    match broadcast_font_change() {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }

    Ok(())
}

pub async fn temp_uninstall(font: &FontData) -> Result<(), Error> {
    match remove_font_resource(&font.file_path) {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }
    
    match broadcast_font_change() {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }

    Ok(())
}
