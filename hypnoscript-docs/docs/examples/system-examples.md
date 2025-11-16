---
sidebar_position: 2
---

# Examplee: System-Functionen

Diese Seite zeigt praxisnahe Examplee für System-Functionen in HypnoScript. Die Examplee sind kommentiert und können direkt übernommen oder angepasst werden.

## Fileoperationen: Lesen, Schreiben, Backup

```hyp
Focus {
    entrance {
        // Datei schreiben
        WriteFile("beispiel.txt", "Hallo HypnoScript!");
        // Datei lesen
        induce content = ReadFile("beispiel.txt");
        observe "Datei-Inhalt: " + content;
        // Backup anlegen
        induce backupName = "beispiel_backup_" + Timestamp() + ".txt";
        CopyFile("beispiel.txt", backupName);
        observe "Backup erstellt: " + backupName;
    }
} Relax;
```

## Directories und Filelisten

```hyp
Focus {
    entrance {
        // Verzeichnis anlegen
        if (!DirectoryExists("daten")) CreateDirectory("daten");
        // Dateien auflisten
        induce files = ListFiles(".");
        observe "Dateien im aktuellen Verzeichnis: " + files;
    }
} Relax;
```

## Automatisierte Fileverarbeitung

```hyp
Focus {
    entrance {
        induce inputDir = "input";
        induce outputDir = "output";
        if (!DirectoryExists(outputDir)) CreateDirectory(outputDir);
        induce files = ListFiles(inputDir);
        for (induce i = 0; i < ArrayLength(files); induce i = i + 1) {
            induce file = ArrayGet(files, i);
            induce content = ReadFile(inputDir + "/" + file);
            induce processed = ToUpper(content);
            WriteFile(outputDir + "/" + file, processed);
            observe "Verarbeitet: " + file;
        }
    }
} Relax;
```

## Prozessmanagement: Systembefehle ausführen

```hyp
Focus {
    entrance {
        induce result = ExecuteCommand("echo Hallo von der Shell!");
        observe "Shell-Ausgabe: " + result;
    }
} Relax;
```

## environment variablen lesen und setzen

```hyp
Focus {
    entrance {
        SetEnvironmentVariable("MEIN_VAR", "Testwert");
        induce value = GetEnvironmentVariable("MEIN_VAR");
        observe "MEIN_VAR: " + value;
    }
} Relax;
```

## Systeminformationen und Monitoring

```hyp
Focus {
    entrance {
        induce sys = GetSystemInfo();
        induce mem = GetMemoryInfo();
        observe "OS: " + sys.os;
        observe "RAM: " + mem.used + "/" + mem.total + " MB verwendet";
    }
} Relax;
```

## Netzwerk: HTTP-Request und Download

```hyp
Focus {
    entrance {
        induce url = "https://example.com";
        induce response = HttpGet(url);
        observe "HTTP-Response: " + Substring(response, 0, 100) + "...";
        DownloadFile(url + "/file.txt", "local.txt");
        observe "Datei heruntergeladen als local.txt";
    }
} Relax;
```

## Fehlerbehandlung bei Fileoperationen

```hyp
Focus {
    suggestion safeRead(path) {
        try {
            awaken ReadFile(path);
        } catch (error) {
            return "Fehler beim Lesen: " + error;
        }
    }
    entrance {
        observe safeRead("nicht_existierend.txt");
    }
} Relax;
```

## Kombinierte System-Workflows

```hyp
Focus {
    entrance {
        // Backup und Monitoring kombiniert
        induce file = "daten.txt";
        if (FileExists(file)) {
            induce backup = file + ".bak";
            CopyFile(file, backup);
            observe "Backup erstellt: " + backup;
        }
        induce sys = GetSystemInfo();
        observe "System: " + sys.os + " (" + sys.architecture + ")";
    }
} Relax;
```

---

**See auch:**

- [System-Functionen Referenz](../builtins/system-functions)
- [Utility-Functionen Examplee](./utility-examples)
