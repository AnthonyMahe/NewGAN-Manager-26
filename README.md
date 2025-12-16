<div align="center">

[🇬🇧 English](README.md) • [🇫🇷 Français](README.fr.md) • [🇪🇸 Español](README.es.md) • [🇩🇪 Deutsch](README.de.md) • [🇮🇹 Italiano](README.it.md) • [🇵🇹 Português](README.pt.md) • [🇺🇦 Українська](README.ua.md) • [🇰🇷 한국어](README.kr.md) • [🇨🇳 中文](README.cn.md)

</div>

---

# NewGAN Manager 26

![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Tauri](https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)

**A modern face generator for Football Manager 26 newgens.**

This is a complete rewrite of the original [NewGAN-Manager](https://github.com/Maradonna90/NewGAN-Manager) by Maradonna90, rebuilt from scratch using **Tauri + Svelte 5** for improved performance and a modern UI.

---

## 🎮 Features

- **8 Languages**: English, French, German, Spanish, Italian, Korean, Chinese, Ukrainian
- **Modern UI**: Dark interface with clean design
- **Lightweight**: Built with Tauri (Rust + WebView) instead of Electron
- **Smart Validation**: Disabled generation button until valid RTF file and image folder are selected
- **3 Modes**: Generate, Preserve, Overwrite

---

## 📥 Installation

### Windows
1. Download the latest `.exe` (portable) or `.msi` installer from [Releases](https://github.com/AnthonyMahe/NewGAN-Manager-26/releases)
2. Run the application
3. Move the `views/` and `filters/` folders to your Football Manager user folder:
   ```
   Documents\Sports Interactive\Football Manager 2026\
   ```
4. Run NewGAN Manager 26

### Linux
1. Download the `.AppImage` or `.deb` from [Releases](https://github.com/AnthonyMahe/NewGAN-Manager-26/releases)
2. For AppImage: `chmod +x *.AppImage` then run it
3. For .deb: `sudo dpkg -i *.deb`
4. Move the `views/` and `filters/` folders to your Football Manager user folder:
   ```
   ~/.local/share/Sports Interactive/Football Manager 2026/
   ```
5. Run NewGAN Manager 26

---

## 🚀 Usage

### Prerequisites
- Use **English (UK)** as FM Language (other languages may have different country codes)
- Use **FM default skin** (custom skins may use flags instead of country codes)

### Steps
1. In Football Manager, go to a player search/squad view
2. Select all players (Ctrl+A)
3. Right-click → **Export to Web Page** → Save as `.rtf`
4. Open NewGAN Manager 26
5. Select the exported `.rtf` file
6. Select your faces directory (organized by ethnicity folders)
7. Choose a mode:
   - **Generate**: Assign faces to players without existing faces
   - **Preserve**: Keep existing mappings, only assign to new players
   - **Overwrite**: Replace all existing face mappings
8. Click **GENERATE FACES**

---

## 👥 Credits

### Original NewGAN-Manager
- **[Maradonna](https://github.com/Maradonna90)**: Creator & Lead Developer
- **Samaroy**: Coordination, Image Generation
- **[HRiddick](https://sortitoutsi.net/user/profile/137954)**: Image Cleaning
- **[Krysler76](https://community.sigames.com/profile/157461-krysler76/)**: FM View Hacking
- **Ayal, Zealand, ZeBurgs**: Image Generation

### NewGAN Manager 26
- **[AnthonyMahe](https://github.com/AnthonyMahe)**: Rewrite with Tauri + Svelte 5

---

## 📄 License

This project is a fork of the original NewGAN-Manager. See the original repository for license information.

---

## 🐛 Troubleshooting

If you encounter issues, please [open an issue](https://github.com/AnthonyMahe/NewGAN-Manager-26/issues) with:
- Your operating system
- Steps to reproduce
- Any error messages or screenshots
