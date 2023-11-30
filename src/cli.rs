use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Font Installer", about = "An application to install and uninstall fonts.")]
pub struct Opt {
    #[structopt(long = "install", parse(from_os_str))]
    pub install: Option<Vec<PathBuf>>,

    #[structopt(long = "uninstall", parse(from_os_str))]
    pub uninstall: Option<Vec<PathBuf>>,

    #[structopt(long = "temp")]
    pub temp: Option<bool>,

    #[structopt(long = "folder", parse(from_os_str))]
    pub folder: Option<PathBuf>,
}
