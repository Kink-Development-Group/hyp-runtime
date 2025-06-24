# scripts/ - Build- und Paketierungsskripte

## Windows (winget)

- **build_winget.ps1**: Baut das self-contained Windows-Binary und erstellt ein ZIP für winget.
- **winget-manifest.yaml**: Beispiel für das winget-Manifest. SHA256 muss nach jedem Release angepasst werden.

**Veröffentlichung:**

1. Release-ZIP auf GitHub hochladen
2. SHA256 berechnen und im Manifest eintragen
3. Manifest als Pull Request im [winget-pkgs](https://github.com/microsoft/winget-pkgs) Repository einreichen

## Linux (APT)

- **build_deb.sh**: Baut das self-contained Linux-Binary und erzeugt ein .deb-Paket (benötigt `fpm`).
- **debian/**: Beispielstruktur für ein Debian-Paket (control, postinst, prerm)

**Veröffentlichung:**

1. .deb-Paket auf GitHub Releases hochladen oder eigenes APT-Repo einrichten
2. Optional: Repository mit `apt-add-repository` bereitstellen
3. Nutzer können mit `sudo apt install hypnoscript` installieren

---

**Hinweis:** Für beide Plattformen werden self-contained Binaries verwendet, sodass keine separate .NET-Installation notwendig ist.
