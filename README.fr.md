<div align="center">

[🇬🇧 English](README.md) • [🇫🇷 Français](README.fr.md) • [🇪🇸 Español](README.es.md) • [🇩🇪 Deutsch](README.de.md) • [🇮🇹 Italiano](README.it.md) • [🇵🇹 Português](README.pt.md) • [🇺🇦 Українська](README.ua.md) • [🇰🇷 한국어](README.kr.md) • [🇨🇳 中文](README.cn.md)

</div>

---

# NewGAN Manager 26

![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Tauri](https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)

**Un générateur de visages moderne pour les newgens de Football Manager 26.**

Ceci est une réécriture complète du [NewGAN-Manager](https://github.com/Maradonna90/NewGAN-Manager) original de Maradonna90, reconstruit de zéro en utilisant **Tauri + Svelte 5** pour de meilleures performances et une interface moderne.

---

## 🎮 Fonctionnalités

- **8 Langues** : Anglais, Français, Allemand, Espagnol, Italien, Coréen, Chinois, Ukrainien
- **Interface Moderne** : Interface sombre avec un design épuré
- **Léger** : Construit avec Tauri (Rust + WebView) au lieu d'Electron
- **Validation Intelligente** : Le bouton de génération reste désactivé tant qu'un fichier RTF valide et un dossier d'images ne sont pas sélectionnés
- **3 Modes** : Générer (Generate), Préserver (Preserve), Écraser (Overwrite)

---

## 📥 Installation

### Windows
1. Téléchargez le dernier `.exe` (portable) ou l'installateur `.msi` depuis les [Releases](https://github.com/AnthonyMahe/NewGAN-Manager-26/releases)
2. Lancez l'application
3. Déplacez les dossiers `views/` et `filters/` dans votre dossier utilisateur Football Manager :
   ```
   Documents\Sports Interactive\Football Manager 2026\
   ```
4. Lancez NewGAN Manager 26

### Linux
1. Téléchargez le `.AppImage` ou le `.deb` depuis les [Releases](https://github.com/AnthonyMahe/NewGAN-Manager-26/releases)
2. Pour AppImage : `chmod +x *.AppImage` puis lancez-le
3. Pour .deb : `sudo dpkg -i *.deb`
4. Déplacez les dossiers `views/` et `filters/` dans votre dossier utilisateur Football Manager :
   ```
   ~/.local/share/Sports Interactive/Football Manager 2026/
   ```
5. Lancez NewGAN Manager 26

---

## 🚀 Utilisation

### Prérequis
- Utilisez **English (UK)** comme langue dans FM (les autres langues peuvent avoir des codes pays différents)
- Utilisez le **thème par défaut de FM** (les thèmes personnalisés peuvent utiliser des drapeaux au lieu des codes pays)

### Étapes
1. Dans Football Manager, allez sur une vue de recherche de joueurs ou d'effectif
2. Sélectionnez tous les joueurs (Ctrl+A)
3. Clic droit → **Imprimer l'écran** (Export to Web Page) → Sauvegarder en `.rtf`
4. Ouvrez NewGAN Manager 26
5. Sélectionnez le fichier `.rtf` exporté
6. Sélectionnez votre dossier de visages (organisé par dossiers d'ethnies)
7. Choisissez un mode :
   - **Generate** : Assigne des visages aux joueurs qui n'en ont pas
   - **Preserve** : Garde les assignations existantes, n'ajoute que les nouveaux
   - **Overwrite** : Remplace toutes les assignations de visages existantes
8. Cliquez sur **GENERATE FACES**

---

## 👥 Crédits

### NewGAN-Manager Original
- **[Maradonna](https://github.com/Maradonna90)** : Créateur & Développeur Principal
- **Samaroy** : Coordination, Génération d'images
- **[HRiddick](https://sortitoutsi.net/user/profile/137954)** : Nettoyage d'images
- **[Krysler76](https://community.sigames.com/profile/157461-krysler76/)** : Hacking des vues FM
- **Ayal, Zealand, ZeBurgs** : Génération d'images

### NewGAN Manager 26
- **[AnthonyMahe](https://github.com/AnthonyMahe)** : Réécriture avec Tauri + Svelte 5

---

## 📄 Licence

Ce projet est un fork du NewGAN-Manager original. Voir le dépôt original pour les informations de licence.

---

## 🐛 Dépannage

Si vous rencontrez des problèmes, merci d'[ouvrir une issue](https://github.com/AnthonyMahe/NewGAN-Manager-26/issues) avec :
- Votre système d'exploitation
- Les étapes pour reproduire le problème
- Tout message d'erreur ou capture d'écran
