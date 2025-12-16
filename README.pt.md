<div align="center">

[🇬🇧 English](README.md) • [🇫🇷 Français](README.fr.md) • [🇪🇸 Español](README.es.md) • [🇩🇪 Deutsch](README.de.md) • [🇮🇹 Italiano](README.it.md) • [🇵🇹 Português](README.pt.md) • [🇺🇦 Українська](README.ua.md) • [🇰🇷 한국어](README.kr.md) • [🇨🇳 中文](README.cn.md)

</div>

---

# NewGAN Manager 26

![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Tauri](https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)

**Um gerador de faces moderno para newgens do Football Manager 26.**

Esta é uma reescrita completa do [NewGAN-Manager](https://github.com/Maradonna90/NewGAN-Manager) original de Maradonna90, reconstruído do zero usando **Tauri + Svelte 5** para melhor desempenho e uma interface moderna.

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

Este projeto é um fork do NewGAN-Manager original. Veja o repositório original para informações de licença.

---

## 🐛 Solução de Problemas

Se encontrar problemas, por favor [abra uma issue](https://github.com/AnthonyMahe/NewGAN-Manager-26/issues) com:
- Seu sistema operacional
- Passos para reproduzir
- Qualquer mensagem de erro ou capturas de tela
