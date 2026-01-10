# NewGAN Manager - FM26 Edition (Fork)

> ⚠️ **CURRENT STATUS: ON HOLD / WAITING FOR GAME UPDATE** ⚠️
>
> **English:** Currently, this tool is **NOT functional with Football Manager 26**. The transition to the Unity Engine removed the "Print Screen" (Ctrl+P) data export feature, which is required to extract Player IDs.
>
> **Français :** Actuellement, cet outil n'est **PAS fonctionnel sur Football Manager 26**. Le passage au moteur Unity a supprimé la fonctionnalité d'export de données (Ctrl+P), indispensable pour récupérer les IDs des joueurs.

---

## 🚧 The Issue (Unity Engine & Data Export)

For NewGAN to work, we need to export the list of players (UIDs, Names, Ethnicities) from the game to a text/XML file. In previous versions, this was done via a Custom View and Ctrl+P.

**In FM26 (Unity), this export feature is currently missing.**
Without this data, the tool cannot map faces to newgens.

### 📢 Official Word from Sports Interactive

On the official SI Forums, the development team has acknowledged the issue regarding the missing data export. 
**Remy Boicherot (SI Staff)** stated:

> *"Trust me, we do care, and log relevant requests [...] from the volume of requests for this specific feature, it's not that "niche", so we're taking your comments and upvotes into account."*

## 📅 Roadmap / Next Steps

1.  **Wait for an FM26 Update:** We are monitoring patch notes. As soon as SI restores the ability to export view data (CSV/XML/HTML), I will update the parser.
2.  **Alternative Methods:** Unless a safe API is discovered, we will not use memory injection (like FMRTE) to keep this tool safe and open-source.

---

<p align="center">
  <img src="src-tauri/icons/icon.png" alt="NewGAN Manager 26" width="128">
</p>

<h1 align="center">NewGAN Manager 26</h1>

<h4 align="center">📖 Leggi nella tua lingua</h4>

<p align="center">
  <a href="README.md"><img src="https://hatscripts.github.io/circle-flags/flags/gb.svg" width="32" alt="English"></a>&nbsp;
  <a href="README.fr.md"><img src="https://hatscripts.github.io/circle-flags/flags/fr.svg" width="32" alt="Français"></a>&nbsp;
  <a href="README.es.md"><img src="https://hatscripts.github.io/circle-flags/flags/es.svg" width="32" alt="Español"></a>&nbsp;
  <a href="README.de.md"><img src="https://hatscripts.github.io/circle-flags/flags/de.svg" width="32" alt="Deutsch"></a>&nbsp;
  <a href="README.it.md"><img src="https://hatscripts.github.io/circle-flags/flags/it.svg" width="32" alt="Italiano"></a>&nbsp;
  <a href="README.pt.md"><img src="https://hatscripts.github.io/circle-flags/flags/pt.svg" width="32" alt="Português"></a>&nbsp;
  <a href="README.ua.md"><img src="https://hatscripts.github.io/circle-flags/flags/ua.svg" width="32" alt="Українська"></a>&nbsp;
  <a href="README.kr.md"><img src="https://hatscripts.github.io/circle-flags/flags/kr.svg" width="32" alt="한국어"></a>&nbsp;
  <a href="README.cn.md"><img src="https://hatscripts.github.io/circle-flags/flags/cn.svg" width="32" alt="中文"></a>
</p>

---

![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Tauri](https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)

**Un generatore di volti moderno per i newgen di Football Manager 26.**

Questa è una riscrittura completa del [NewGAN-Manager](https://github.com/Maradonna90/NewGAN-Manager) originale di Maradonna90, ricostruito da zero usando **Tauri + Svelte 5** per prestazioni migliori e un'interfaccia moderna.

<p align="center">
  <img src="assets/screenshot-it.png" alt="NewGAN Manager 26 Screenshot" width="600">
</p>

---

## 🎮 Funzionalità

- **8 Lingue**: Inglese, Francese, Tedesco, Spagnolo, Italiano, Coreano, Cinese, Ucraino
- **Interfaccia Moderna**: Interfaccia scura con design pulito
- **Leggero**: Costruito con Tauri (Rust + WebView) invece di Electron
- **Validazione Intelligente**: Il pulsante di generazione rimane disabilitato finché non vengono selezionati un file RTF valido e una cartella immagini
- **3 Modalità**: Genera (Generate), Preserva (Preserve), Sovrascrivi (Overwrite)

---

## 📥 Installazione

### Windows
1. Scarica l'ultimo `.exe` (portable) o l'installer `.msi` dalle [Releases](https://github.com/AnthonyMahe/NewGAN-Manager-26/releases)
2. Esegui l'applicazione
3. Sposta le cartelle `views/` e `filters/` nella tua cartella utente di Football Manager:
   ```
   Documents\Sports Interactive\Football Manager 2026\
   ```
4. Avvia NewGAN Manager 26

### Linux
1. Scarica l'`.AppImage` o `.deb` dalle [Releases](https://github.com/AnthonyMahe/NewGAN-Manager-26/releases)
2. Per AppImage: `chmod +x *.AppImage` poi eseguilo
3. Per .deb: `sudo dpkg -i *.deb`
4. Sposta le cartelle `views/` e `filters/` nella tua cartella utente di Football Manager:
   ```
   ~/.local/share/Sports Interactive/Football Manager 2026/
   ```
5. Avvia NewGAN Manager 26

---

## 🚀 Utilizzo

### Prerequisiti
- Usa **English (UK)** come lingua in FM (altre lingue potrebbero avere codici paese diversi)
- Usa la **skin predefinita di FM** (le skin personalizzate potrebbero usare bandiere invece dei codici paese)

### Passaggi
1. In Football Manager, vai alla ricerca giocatori/vista rosa
2. Seleziona tutti i giocatori (Ctrl+A)
3. Clic destro → **Esporta come pagina web** → Salva come `.rtf`
4. Apri NewGAN Manager 26
5. Seleziona il file `.rtf` esportato
6. Seleziona la tua cartella dei volti (organizzata per cartelle etnie)
7. Scegli una modalità:
   - **Generate**: Assegna volti ai giocatori senza volti esistenti
   - **Preserve**: Mantiene le assegnazioni esistenti, assegna solo ai nuovi giocatori
   - **Overwrite**: Sostituisce tutte le assegnazioni di volti esistenti
8. Clicca su **GENERATE FACES**

---

## 👥 Crediti

### NewGAN-Manager Originale
- **[Maradonna](https://github.com/Maradonna90)**: Creatore & Sviluppatore Principale
- **Samaroy**: Coordinamento, Generazione immagini
- **[HRiddick](https://sortitoutsi.net/user/profile/137954)**: Pulizia immagini
- **[Krysler76](https://community.sigames.com/profile/157461-krysler76/)**: Hacking viste FM
- **Ayal, Zealand, ZeBurgs**: Generazione immagini

### NewGAN Manager 26
- **[AnthonyMahe](https://github.com/AnthonyMahe)**: Riscrittura con Tauri + Svelte 5

---

## 📄 Licenza

Distribuito sotto la GNU General Public License v3.0. Vedere `LICENSE` per maggiori informazioni.

---

## 🐛 Risoluzione Problemi

Se riscontri problemi, per favore [apri una issue](https://github.com/AnthonyMahe/NewGAN-Manager-26/issues) con:
- Il tuo sistema operativo
- I passaggi per riprodurre il problema
- Qualsiasi messaggio di errore o screenshot

---

<p align="center">
  <a href="https://liberapay.com/TonyBoySUPER/donate">
    <img alt="Donate using Liberapay" src="https://liberapay.com/assets/widgets/donate.svg">
  </a>
</p>


