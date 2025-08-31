<h1 align="center">ThemeClipper</h1>

<h4 align="center">Generate backdrops from movies and TV shows for Jellyfin. Built on <a href="https://rust-lang.org" target="_blank">Rust</a>.</h4><br>

ThemeClipper is a **lightweight Blazing fast CLI tool** that automatically generates **theme clips** for your movies and series using **FFmpeg**.

---

## Features

- Generate theme clips for **Movies**, **TV Shows / Series**
- **Random method** for selecting clips ( currently supported )
- Option to **delete all Backdrops folders**
- Cross platform ( **Linux, Mac, Windows** )

---

## Upcoming Features

- [ ] **Audio-based** clip detection
- [ ] **Visual-based** scene analysis
- [ ] **Music-driven** theme clips

---

## Demo

![Demo](docs/assets/demo.gif)

[Watch Full Demo Video](docs/assets/demo.mp4)

## Usage

Run ThemeClipper with your desired options:

```bash
theme_clipper --help
```

Example:

```bash
theme_clipper --movies-directory /path/to/movies --clip-length 15
```

To clean up all Backdrops:

```bash
theme_clipper --shows-directory /path/to/shows --delete
```

### support config file

example config.json:

```json
{
  "movies_directory": "./data/movies/",
  "shows_directory": "./data/shows/",
  "clip_length": 10,
  "method": "random"
}
```

using config:

```bash
theme_clipper --config /path/to/config.json
```

---

### Installation

**Pre-requisite:**  
You must have [FFmpeg](https://ffmpeg.org/download.html) installed and available in your system's PATH.

```bash
# Linux (Debian/Ubuntu)
sudo apt update && sudo apt install ffmpeg -y

# Linux (Arch/Manjaro)
sudo pacman -S ffmpeg

# macOS (with Homebrew)
brew install ffmpeg

# Windows (with Chocolatey)
choco install ffmpeg

# Windows (with Scoop)
scoop install ffmpeg
```

1. Download the latest release from [🔗 Releases](https://github.com/kumarvivek1752/ThemeClipper/releases).

2. Make it executable:

```bash
# Linux
chmod +x theme_clipper-0.1.0-x86_64-unknown-linux-gnu

# macOS
chmod +x theme_clipper-0.1.0-aarch64-apple-darwin
# or if Intel Mac
chmod +x theme_clipper-0.1.0-x86_64-apple-darwin

# Windows (PowerShell)
# No need for chmod, just use the .exe directly
```

3. (Recommended) Rename and move for easier use:

```bash
# Linux
sudo mv theme_clipper-0.1.0-x86_64-unknown-linux-gnu /usr/local/bin/themeclipper

# macOS (Apple Silicon)
sudo mv theme_clipper-0.1.0-aarch64-apple-darwin /usr/local/bin/themeclipper
# macOS (Intel)
sudo mv theme_clipper-0.1.0-x86_64-apple-darwin /usr/local/bin/themeclipper

# Windows (PowerShell, run as Administrator)
Move-Item -Path .\theme_clipper-0.1.0-x86_64-pc-windows-msvc.exe -Destination "C:\Program Files\themeclipper\themeclipper.exe"
# Then add "C:\Program Files\themeclipper" to your PATH environment variable
```

4. Run it anywhere:

```bash
# Linux
themeclipper

# macOS
themeclipper

# Windows (PowerShell or CMD)
themeclipper
```

---

## ❤️ Support & Contributions

- 💡 Created with love by **Vivek ([@kumarvivek1752](https://github.com/kumarvivek1752))**
- 💝 Donate via [PayPal](https://paypal.me/kumarvivek1752)
- 🐙 Source Code on [GitHub](https://github.com/kumarvivek1752/ThemeClipper)
- 🙏 If you find ThemeClipper useful, consider **supporting or starring ⭐ the repo** to keep the project alive!

---

⚡ **ThemeClipper makes your Jellyfin experience more cinematic!** 🎬

---
