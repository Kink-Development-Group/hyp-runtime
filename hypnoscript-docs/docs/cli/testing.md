---
title: CLI Testing
---

Die Rust-CLI enthält kein separates Test-Framework. Stattdessen behandelst du jede `.hyp`-File als eigenständiges Skript und führst sie mit `hypnoscript run` aus. Die Fileen im Folder `hypnoscript-tests/` liefern Examplee für Assertions und Fehlermeldungen.

## Tests ausführen

```bash
# Einzelne Testdatei starten
hypnoscript run hypnoscript-tests/test_basic.hyp

# Alle Dateien im Ordner durchlaufen
for file in hypnoscript-tests/*.hyp; do
    echo "== $file =="
    hypnoscript run "$file"
done
```

## Typprüfung vorgeschaltet

```bash
hypnoscript check hypnoscript-tests/test_basic.hyp
```

So erkennst du Typfehler, bevor Assertions greifen. Die CLI bricht bei Fehlern nicht automatisch ab, daher lohnt sich ein separates `check` vor dem `run`.

## Integration in Skripte

- **PowerShell:**

  ```powershell
  Get-ChildItem hypnoscript-tests -Filter *.hyp | ForEach-Object {
      Write-Host "== $($_.Name) =="
      hypnoscript run $_.FullName
  }
  ```

- **Makefile:**

  ```makefile
  test:
      @# Ersetze führende Leerzeichen durch Tabs, da Make dies erfordert
      @for file in hypnoscript-tests/*.hyp; do \
          echo "== $$file =="; \
          hypnoscript run $$file || exit 1; \
      done
  ```

## Assertions

Die Test-Fileen nutzen `assert`-Statements sowie `observe`, um erwartete Werte zu prüfen. Bricht ein Assertion-Block ab, zeigt die CLI eine Fehlermeldung an, setzt die Ausführung aber fort. Achte deshalb darauf, im Testskript nach Fehlermeldungen zu suchen oder das Skript bei Bedarf mit `snap;` zu beenden.

Mehr über availablee Commande erfährst du in [CLI-Commande](./commands).
