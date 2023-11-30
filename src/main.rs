use structopt::StructOpt;
mod cli;
use cli::Opt;
use std::fs;
use std::path::PathBuf;
mod utils;
use utils::font::create_font_data;
use utils::helpers::print_table;
mod platforms;
use std::time::Instant;

#[cfg(target_os = "linux")]
use {
    chrono::prelude::*,
    utils::helpers::update_font_cache,
    platforms::linux,
};

#[cfg(target_os = "windows")]
use platforms::windows;

#[cfg(target_os = "macos")]
use platforms::darwin;

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    if let Some(install) = &opt.install {
        if !install.is_empty() {
            process_paths(install, &opt.temp.unwrap_or(false), true).await;
        }
    }

    if let Some(uninstall) = &opt.uninstall {
        if !uninstall.is_empty() {
            process_paths(uninstall, &opt.temp.unwrap_or(false), false).await;
        }
    }

    if let Some(folder) = &opt.folder {
        if let Ok(entries) = fs::read_dir(folder) {
            let mut fonts = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            if extension == "ttf" || extension == "otf" {
                                fonts.push(path);
                            }
                        }
                    }
                }
            }
            if !fonts.is_empty() {
                process_paths(&fonts, &opt.temp.unwrap_or(false), true).await;
            }
        }
    }
}

async fn process_paths(paths: &Vec<PathBuf>, temp: &bool, is_install: bool) {
    let mut installed = Vec::new();

    for (index, path) in paths.iter().enumerate() {
        let start = Instant::now();

        let font_data = create_font_data(&path).await;

        match font_data {
            Some(data) => {
                let duration = start.elapsed();
                installed.push((index, data.font_name.clone(), duration));

                #[cfg(target_os = "windows")]
                {
                    if *temp {
                        if is_install {
                            if let Err(e) = windows::temp_install(&data).await {
                                println!("Failed to install font: {}", e);
                            }
                        } else {
                            if let Err(e) = windows::temp_uninstall(&data).await {
                                println!("Failed to uninstall font: {}", e);
                            }
                        }
                    } else {
                        if is_install {
                            if let Err(e) = windows::install(&data).await {
                                println!("Failed to install font: {}", e);
                            }
                        } else {
                            if let Err(e) = windows::uninstall(&data).await {
                                println!("Failed to uninstall font: {}", e);
                            }
                        }
                    }
                }

                #[cfg(target_os = "linux")]
                {
                    if is_install {
                        let now = Utc::now();
                        let timestamp = now.format("%Y%m%d%H%M%S").to_string();
                        if let Err(e) = linux::install(&data, true, timestamp).await {
                            println!("Failed to install font: {}", e);
                        }
                    } else {
                        if let Err(e) = linux::uninstall(&data).await {
                            println!("Failed to uninstall font: {}", e);
                        }
                    }
                }

                #[cfg(unix)]
                {
                    if *temp {
                        println!("Temporary font installation not supported.")
                    }
                }

                #[cfg(target_os = "darwin")]
                {
                    if is_install {
                        if let Err(e) = macos::install(&data).await {
                            println!("Failed to install font: {}", e);
                        }
                    } else {
                        if let Err(e) = macos::uninstall(&data).await {
                            println!("Failed to uninstall font: {}", e);
                        }
                    }
                }
            }
            None => println!("The file is not a valid font file"),
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Err(e) = update_font_cache().await {
            eprintln!("Failed to update font cache: {}", e);
        }
    }

    // Sort installed by duration
    installed.sort_by(|a, b| a.2.cmp(&b.2));

    print_table(&installed).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_process_paths() {
        let mut font_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        font_path.push("tests");
        font_path.push("FiraCode-Regular.otf");

        let paths = vec![font_path.clone()];

        let temp = false;
        let is_install = true;

        process_paths(&paths, &temp, is_install).await;

        assert!(fs::metadata(font_path.clone()).await.is_ok());

        assert!(fs::read(font_path).await.unwrap().len() > 0);
    }
}
