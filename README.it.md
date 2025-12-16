<p align="center">
  <a href="README.md"><img src="https://flagcdn.com/48x36/gb.png" alt="English" width="40"></a>
  &nbsp;&nbsp;
  <a href="README.fr.md"><img src="https://flagcdn.com/48x36/fr.png" alt="Français" width="40"></a>
  &nbsp;&nbsp;
  <a href="README.es.md"><img src="https://flagcdn.com/48x36/es.png" alt="Español" width="40"></a>
  &nbsp;&nbsp;
  <a href="README.de.md"><img src="https://flagcdn.com/48x36/de.png" alt="Deutsch" width="40"></a>
  &nbsp;&nbsp;
  <a href="README.it.md"><img src="https://flagcdn.com/48x36/it.png" alt="Italiano" width="40"></a>
  &nbsp;&nbsp;
  <a href="README.pt.md"><img src="https://flagcdn.com/48x36/pt.png" alt="Português" width="40"></a>
  &nbsp;&nbsp;
  <a href="README.ua.md"><img src="https://flagcdn.com/48x36/ua.png" alt="Українська" width="40"></a>
  &nbsp;&nbsp;
  <a href="README.kr.md"><img src="https://flagcdn.com/48x36/kr.png" alt="한국어" width="40"></a>
  &nbsp;&nbsp;
  <a href="README.cn.md"><img src="https://flagcdn.com/48x36/cn.png" alt="中文" width="40"></a>
</p>

---

# NewGAN Manager 26

![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Tauri](https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)

**Un generatore di volti moderno per i newgen di Football Manager 26.**

Questa è una riscrittura completa del [NewGAN-Manager](https://github.com/Maradonna90/NewGAN-Manager) originale di Maradonna90, ricostruito da zero usando **Tauri + Svelte 5** per prestazioni migliori e un'interfaccia moderna.

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

Questo progetto è un fork del NewGAN-Manager originale. Vedi il repository originale per informazioni sulla licenza.

---

## 🐛 Risoluzione Problemi

Se riscontri problemi, per favore [apri una issue](https://github.com/AnthonyMahe/NewGAN-Manager-26/issues) con:
- Il tuo sistema operativo
- I passaggi per riprodurre il problema
- Qualsiasi messaggio di errore o screenshot
