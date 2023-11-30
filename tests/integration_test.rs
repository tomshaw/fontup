use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_install_uninstall() {
    let mut font_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    font_path.push("tests");
    font_path.push("FiraCode-Regular.otf");

    // Test installation
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--install")
        .arg(font_path.to_str().unwrap())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Installation failed: {}", String::from_utf8_lossy(&output.stderr));

    // Test uninstallation
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--uninstall")
        .arg(font_path.to_str().unwrap())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Uninstallation failed: {}", String::from_utf8_lossy(&output.stderr));
}

