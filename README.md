# 🚀 FontUp 🚀

![Build](https://github.com/tomshaw/fontup/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/d/fontup.svg)](https://crates.io/crates/fontup)
[![Crates.io](https://img.shields.io/crates/v/fontup.svg)](https://crates.io/crates/fontup)

**FontUp** is a cross-platform command line application that makes installing and uninstalling fonts a blast! 🌬️

## 🌟 Features 🌟

- Install and uninstall fonts on both Unix-like and Windows systems. 🖥️
- Temporary font installation and uninstallation on Windows. ⏳
- Error checking to ensure fonts are installed and uninstalled correctly. ✔️

## 🛠️ Usage 🛠️

First, add FontUp to your `Cargo.toml`:

```toml
[dependencies]
fontup = "0.1.0"
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

## 🙏 Acknowledgements 🙏

This project uses the [Fira Code](https://github.com/tonsky/FiraCode) font, which is licensed under the SIL Open Font License, Version 1.1.

## 📜 License 📜

The MIT License (MIT). See [License File](LICENSE) for more information.