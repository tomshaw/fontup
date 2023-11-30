# 🚀 FontUp 🚀

![Build](https://github.com/tomshaw/fontup/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/d/fontup.svg)](https://crates.io/crates/fontup)
[![Crates.io](https://img.shields.io/crates/v/fontup.svg)](https://crates.io/crates/fontup)

**FontUp** is a cross-platform command line application that makes installing and uninstalling fonts a blast! 💥

## 🌟 Features 🌟

- Install and uninstall fonts on both Unix-like and Windows systems. 🖥️
- Temporary font installation and uninstallation on Windows. ⏳
- Runs asynchronously and is lightning fast. 🏃
- Error checking to ensure fonts are installed and uninstalled correctly. ✔️
- Prints neat colorized table of installed fonts sorted by duration. 🦋

## 🛠️ Usage 🛠️

First, add FontUp to your `Cargo.toml`:

```toml
[dependencies]
fontup = "0.1.2"
```

### 📥 Install Fonts 📥

Use the `--install` argument to specify one or more font files that you want to install. You can provide multiple files by separating them with spaces.

> 📝 Note: Use quotes if your file paths include spaces.

```shell
fontup --install /path/to/font1.ttf /path/to/font2.ttf
```

This will install the fonts located at `/path/to/font1.ttf` and `/path/to/font2.ttf`.

### 📤 Uninstall Fonts 📤

The `--uninstall` argument is used to specify one or more font files that you want to uninstall. You can provide multiple files by separating them with spaces.

```shell
fontup --uninstall /path/to/font1.ttf /path/to/font2.ttf
```

This will uninstall the fonts located at `/path/to/font1.ttf` and `/path/to/font2.ttf`.

### ⏱️ Temporary Installation ⏱️

> 📝 Note: This is a Windows only feature.

The `--temp` is a boolean flag that indicates whether font installation is temporary. Once your system reboots the font will be automagically removed.

```shell
fontup --install /path/to/font1.ttf --temp
```

```shell
fontup --uninstall /path/to/font1.ttf --temp
```

### 📁 Font Folders 📁

Installs all the fonts found in specified folder

```shell
fontup --folder /path/to/folder
```

## 📦 Dependencies 📦

This utility depends on the following Rust crates:

- `chrono`: 0.4 - Comprehensive support for working with date and time.
- `colored`: 2.0.4 - Coloring terminal text output.
- `cli-table`: 0.4.7 - Creating and displaying tables in the command line.
- `dirs`: 3.0.2 - Finding platform-specific, user-accessible directories.
- `structopt`: 0.3.26 - Handling command line arguments by defining a struct.
- `ttf-parser`: 0.20.0 - High-level, safe, zero-allocation TrueType font parser.
- `tokio`: 1 (with full features) - Event-driven, non-blocking I/O platform for writing asynchronous applications.

### Windows Specific Dependencies

- `winapi`: 0.3 (with winuser feature) - Raw FFI bindings to all of Windows API.
- `winreg`: 0.52.0 - Rust bindings to the Windows Registry API.

## 🙏 Acknowledgements 🙏

This project uses the [Fira Code](https://github.com/tonsky/FiraCode) font, which is licensed under the SIL Open Font License, Version 1.1.

## 📜 License 📜

The MIT License (MIT). See [License File](LICENSE) for more information.