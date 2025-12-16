<div align="center">

[🇬🇧 English](README.md) • [🇫🇷 Français](README.fr.md) • [🇪🇸 Español](README.es.md) • [🇩🇪 Deutsch](README.de.md) • [🇮🇹 Italiano](README.it.md) • [🇺🇦 Українська](README.ua.md) • [🇰🇷 한국어](README.kr.md) • [🇨🇳 中文](README.cn.md)

</div>

---

# NewGAN Manager 26

![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Tauri](https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)

**Un generador de caras moderno para los newgens de Football Manager 26.**

Esta es una reescritura completa del [NewGAN-Manager](https://github.com/Maradonna90/NewGAN-Manager) original de Maradonna90, reconstruido desde cero usando **Tauri + Svelte 5** para un mejor rendimiento y una interfaz moderna.

---

## 🎮 Características

- **8 Idiomas**: Inglés, Francés, Alemán, Español, Italiano, Coreano, Chino, Ucraniano
- **Interfaz Moderna**: Interfaz oscura con diseño limpio
- **Ligero**: Construido con Tauri (Rust + WebView) en lugar de Electron
- **Validación Inteligente**: El botón de generación permanece desactivado hasta que se seleccione un archivo RTF válido y una carpeta de imágenes
- **3 Modos**: Generar (Generate), Preservar (Preserve), Sobrescribir (Overwrite)

---

## 📥 Instalación

### Windows
1. Descarga el último `.exe` (portable) o el instalador `.msi` desde [Releases](https://github.com/AnthonyMahe/NewGAN-Manager-26/releases)
2. Ejecuta la aplicación
3. Mueve las carpetas `views/` y `filters/` a tu carpeta de usuario de Football Manager:
   ```
   Documents\Sports Interactive\Football Manager 2026\
   ```
4. Ejecuta NewGAN Manager 26

### Linux
1. Descarga el `.AppImage` o `.deb` desde [Releases](https://github.com/AnthonyMahe/NewGAN-Manager-26/releases)
2. Para AppImage: `chmod +x *.AppImage` y luego ejecútalo
3. Para .deb: `sudo dpkg -i *.deb`
4. Mueve las carpetas `views/` y `filters/` a tu carpeta de usuario de Football Manager:
   ```
   ~/.local/share/Sports Interactive/Football Manager 2026/
   ```
5. Ejecuta NewGAN Manager 26

---

## 🚀 Uso

### Requisitos previos
- Usa **English (UK)** como idioma en FM (otros idiomas pueden tener códigos de país diferentes)
- Usa el **skin por defecto de FM** (los skins personalizados pueden usar banderas en lugar de códigos de país)

### Pasos
1. En Football Manager, ve a una vista de búsqueda de jugadores o plantilla
2. Selecciona todos los jugadores (Ctrl+A)
3. Clic derecho → **Exportar a página web** → Guardar como `.rtf`
4. Abre NewGAN Manager 26
5. Selecciona el archivo `.rtf` exportado
6. Selecciona tu carpeta de caras (organizada por carpetas de etnicidades)
7. Elige un modo:
   - **Generate**: Asigna caras a jugadores sin caras existentes
   - **Preserve**: Mantiene las asignaciones existentes, solo asigna a nuevos jugadores
   - **Overwrite**: Reemplaza todas las asignaciones de caras existentes
8. Haz clic en **GENERATE FACES**

---

## 👥 Créditos

### NewGAN-Manager Original
- **[Maradonna](https://github.com/Maradonna90)**: Creador y Desarrollador Principal
- **Samaroy**: Coordinación, Generación de imágenes
- **[HRiddick](https://sortitoutsi.net/user/profile/137954)**: Limpieza de imágenes
- **[Krysler76](https://community.sigames.com/profile/157461-krysler76/)**: Hacking de vistas FM
- **Ayal, Zealand, ZeBurgs**: Generación de imágenes

### NewGAN Manager 26
- **[AnthonyMahe](https://github.com/AnthonyMahe)**: Reescritura con Tauri + Svelte 5

---

## 📄 Licencia

Este proyecto es un fork del NewGAN-Manager original. Consulta el repositorio original para información de licencia.

---

## 🐛 Solución de problemas

Si encuentras problemas, por favor [abre un issue](https://github.com/AnthonyMahe/NewGAN-Manager-26/issues) con:
- Tu sistema operativo
- Pasos para reproducir el problema
- Cualquier mensaje de error o captura de pantalla
