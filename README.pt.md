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

<h4 align="center">📖 Leia no seu idioma</h4>

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

**Um gerador de faces moderno para newgens do Football Manager 26.**

Esta é uma reescrita completa do [NewGAN-Manager](https://github.com/Maradonna90/NewGAN-Manager) original de Maradonna90, reconstruído do zero usando **Tauri + Svelte 5** para melhor desempenho e uma interface moderna.

<p align="center">
  <img src="assets/screenshot-pt.png" alt="NewGAN Manager 26 Screenshot" width="600">
</p>

---

## 🎮 Funcionalidades

- **9 Idiomas**: Inglês, Francês, Alemão, Espanhol, Italiano, Português, Coreano, Chinês, Ucraniano
- **Interface Moderna**: Interface escura com design limpo
- **Leve**: Construído com Tauri (Rust + WebView) em vez de Electron
- **Validação Inteligente**: Botão de geração desativado até que um arquivo RTF válido e pasta de imagens sejam selecionados
- **3 Modos**: Gerar (Generate), Preservar (Preserve), Sobrescrever (Overwrite)

---

## 📥 Instalação

### Windows
1. Baixe o último `.exe` (portátil) ou instalador `.msi` das [Releases](https://github.com/AnthonyMahe/NewGAN-Manager-26/releases)
2. Execute a aplicação
3. Mova as pastas `views/` e `filters/` para sua pasta de usuário do Football Manager:
   ```
   Documents\Sports Interactive\Football Manager 2026\
   ```
4. Execute NewGAN Manager 26

### Linux
1. Baixe o `.AppImage` ou `.deb` das [Releases](https://github.com/AnthonyMahe/NewGAN-Manager-26/releases)
2. Para AppImage: `chmod +x *.AppImage` e execute
3. Para .deb: `sudo dpkg -i *.deb`
4. Mova as pastas `views/` e `filters/` para sua pasta de usuário do Football Manager:
   ```
   ~/.local/share/Sports Interactive/Football Manager 2026/
   ```
5. Execute NewGAN Manager 26

---

## 🚀 Uso

### Pré-requisitos
- Use **English (UK)** como idioma do FM (outros idiomas podem ter códigos de país diferentes)
- Use a **skin padrão do FM** (skins personalizadas podem usar bandeiras em vez de códigos de país)

### Passos
1. No Football Manager, vá para uma visualização de busca de jogadores/elenco
2. Selecione todos os jogadores (Ctrl+A)
3. Clique com botão direito → **Exportar para Página Web** → Salvar como `.rtf`
4. Abra NewGAN Manager 26
5. Selecione o arquivo `.rtf` exportado
6. Selecione seu diretório de faces (organizado por pastas de etnias)
7. Escolha um modo:
   - **Generate**: Atribui faces a jogadores sem faces existentes
   - **Preserve**: Mantém mapeamentos existentes, apenas atribui a novos jogadores
   - **Overwrite**: Substitui todos os mapeamentos de face existentes
8. Clique em **GENERATE FACES**

---

## 👥 Créditos

### NewGAN-Manager Original
- **[Maradonna](https://github.com/Maradonna90)**: Criador e Desenvolvedor Principal
- **Samaroy**: Coordenação, Geração de Imagens
- **[HRiddick](https://sortitoutsi.net/user/profile/137954)**: Limpeza de Imagens
- **[Krysler76](https://community.sigames.com/profile/157461-krysler76/)**: Hacking de Views do FM
- **Ayal, Zealand, ZeBurgs**: Geração de Imagens

### NewGAN Manager 26
- **[AnthonyMahe](https://github.com/AnthonyMahe)**: Reescrita com Tauri + Svelte 5

---

## 📄 Licença

Distribuído sob a GNU General Public License v3.0. Veja `LICENSE` para mais informações.

---

## 🐛 Solução de Problemas

Se encontrar problemas, por favor [abra uma issue](https://github.com/AnthonyMahe/NewGAN-Manager-26/issues) com:
- Seu sistema operacional
- Passos para reproduzir
- Qualquer mensagem de erro ou capturas de tela

---

<p align="center">
  <a href="https://liberapay.com/TonyBoySUPER/donate">
    <img alt="Donate using Liberapay" src="https://liberapay.com/assets/widgets/donate.svg">
  </a>
</p>


