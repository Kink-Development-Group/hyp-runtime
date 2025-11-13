---
sidebar_position: 6
---

# System-Funktionen

System-Funktionen ermöglichen die Interaktion mit dem Betriebssystem, Dateisystem, Prozessen und Umgebungsvariablen.

## Dateisystem-Operationen

### ReadFile(path)

Liest den Inhalt einer Datei als String.

```hyp
induce content = ReadFile("config.txt");
observe content;
```

### WriteFile(path, content)

Schreibt Inhalt in eine Datei.

```hyp
WriteFile("output.txt", "Hallo Welt!");
```

### AppendFile(path, content)

Fügt Inhalt an eine bestehende Datei an.

```hyp
AppendFile("log.txt", "Neuer Eintrag: " + Now());
```

### FileExists(path)

Prüft, ob eine Datei existiert.

```hyp
if (FileExists("config.json")) {
    induce config = ReadFile("config.json");
    // Verarbeite Konfiguration
}
```

### DeleteFile(path)

Löscht eine Datei.

```hyp
if (FileExists("temp.txt")) {
    DeleteFile("temp.txt");
}
```

### CopyFile(source, destination)

Kopiert eine Datei.

```hyp
CopyFile("source.txt", "backup.txt");
```

### MoveFile(source, destination)

Verschiebt eine Datei.

```hyp
MoveFile("old.txt", "new.txt");
```

### GetFileSize(path)

Gibt die Größe einer Datei in Bytes zurück.

```hyp
induce size = GetFileSize("large.txt");
observe "Dateigröße: " + size + " Bytes";
```

### GetFileInfo(path)

Gibt Informationen über eine Datei zurück.

```hyp
induce info = GetFileInfo("document.txt");
observe "Erstellt: " + info.created;
observe "Geändert: " + info.modified;
observe "Größe: " + info.size + " Bytes";
```

## Verzeichnis-Operationen

### CreateDirectory(path)

Erstellt ein Verzeichnis.

```hyp
CreateDirectory("logs");
```

### DirectoryExists(path)

Prüft, ob ein Verzeichnis existiert.

```hyp
if (!DirectoryExists("output")) {
    CreateDirectory("output");
}
```

### ListFiles(path)

Listet alle Dateien in einem Verzeichnis auf.

```hyp
induce files = ListFiles(".");
for (induce i = 0; i < ArrayLength(files); induce i = i + 1) {
    observe ArrayGet(files, i);
}
```

### ListDirectories(path)

Listet alle Unterverzeichnisse auf.

```hyp
induce dirs = ListDirectories(".");
observe "Unterverzeichnisse: " + dirs;
```

### DeleteDirectory(path, recursive)

Löscht ein Verzeichnis.

```hyp
DeleteDirectory("temp", true); // Rekursiv löschen
```

### GetCurrentDirectory()

Gibt das aktuelle Arbeitsverzeichnis zurück.

```hyp
induce cwd = GetCurrentDirectory();
observe "Aktuelles Verzeichnis: " + cwd;
```

### ChangeDirectory(path)

Wechselt das Arbeitsverzeichnis.

```hyp
ChangeDirectory("../data");
```

## Prozess-Management

### ExecuteCommand(command)

Führt einen Systembefehl aus.

```hyp
induce result = ExecuteCommand("dir");
observe result;
```

### ExecuteCommandAsync(command)

Führt einen Systembefehl asynchron aus.

```hyp
induce process = ExecuteCommandAsync("ping google.com");
// Prozess läuft im Hintergrund
```

### KillProcess(processId)

Beendet einen Prozess.

```hyp
induce pid = 1234;
KillProcess(pid);
```

### GetProcessList()

Gibt eine Liste aller laufenden Prozesse zurück.

```hyp
induce processes = GetProcessList();
for (induce i = 0; i < ArrayLength(processes); induce i = i + 1) {
    induce proc = ArrayGet(processes, i);
    observe proc.name + " (PID: " + proc.id + ")";
}
```

### GetCurrentProcessId()

Gibt die Prozess-ID des aktuellen Skripts zurück.

```hyp
induce pid = GetCurrentProcessId();
observe "Aktuelle PID: " + pid;
```

## Umgebungsvariablen

### GetEnvironmentVariable(name)

Liest eine Umgebungsvariable.

```hyp
induce path = GetEnvironmentVariable("PATH");
induce user = GetEnvironmentVariable("USERNAME");
```

### SetEnvironmentVariable(name, value)

Setzt eine Umgebungsvariable.

```hyp
SetEnvironmentVariable("MY_VAR", "mein_wert");
```

### GetAllEnvironmentVariables()

Gibt alle Umgebungsvariablen zurück.

```hyp
induce env = GetAllEnvironmentVariables();
for (induce key in env) {
    observe key + " = " + env[key];
}
```

## System-Informationen

### GetSystemInfo()

Gibt allgemeine Systeminformationen zurück.

```hyp
induce sysInfo = GetSystemInfo();
observe "Betriebssystem: " + sysInfo.os;
observe "Architektur: " + sysInfo.architecture;
observe "Prozessoren: " + sysInfo.processors;
```

### GetMemoryInfo()

Gibt Speicherinformationen zurück.

```hyp
induce memInfo = GetMemoryInfo();
observe "Gesamter RAM: " + memInfo.total + " MB";
observe "Verfügbarer RAM: " + memInfo.available + " MB";
observe "Verwendeter RAM: " + memInfo.used + " MB";
```

### GetDiskInfo()

Gibt Festplatteninformationen zurück.

```hyp
induce diskInfo = GetDiskInfo();
for (induce drive in diskInfo) {
    observe "Laufwerk " + drive.letter + ":";
    observe "  Gesamt: " + drive.total + " GB";
    observe "  Verfügbar: " + drive.free + " GB";
}
```

### GetNetworkInfo()

Gibt Netzwerkinformationen zurück.

```hyp
induce netInfo = GetNetworkInfo();
observe "Hostname: " + netInfo.hostname;
observe "IP-Adresse: " + netInfo.ipAddress;
```

## Netzwerk-Operationen

### DownloadFile(url, destination)

Lädt eine Datei von einer URL herunter.

```hyp
DownloadFile("https://example.com/file.txt", "downloaded.txt");
```

### UploadFile(url, filePath)

Lädt eine Datei zu einer URL hoch.

```hyp
UploadFile("https://example.com/upload", "local.txt");
```

### HttpGet(url)

Führt eine HTTP GET-Anfrage aus.

```hyp
induce response = HttpGet("https://api.example.com/data");
induce data = ParseJSON(response);
```

### HttpPost(url, data)

Führt eine HTTP POST-Anfrage aus.

```hyp
induce postData = StringifyJSON({"name": "Max", "age": 30});
induce response = HttpPost("https://api.example.com/users", postData);
```

## Registry-Operationen (Windows)

### ReadRegistryValue(key, valueName)

Liest einen Registry-Wert.

```hyp
induce version = ReadRegistryValue("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion", "ProductName");
```

### WriteRegistryValue(key, valueName, value)

Schreibt einen Registry-Wert.

```hyp
WriteRegistryValue("HKEY_CURRENT_USER\\Software\\MyApp", "Version", "1.0");
```

### DeleteRegistryValue(key, valueName)

Löscht einen Registry-Wert.

```hyp
DeleteRegistryValue("HKEY_CURRENT_USER\\Software\\MyApp", "TempValue");
```

## System-Events

### OnSystemEvent(eventType, callback)

Registriert einen Event-Handler für System-Events.

```hyp
OnSystemEvent("fileChanged", function(path) {
    observe "Datei geändert: " + path;
});
```

### TriggerSystemEvent(eventType, data)

Löst ein System-Event aus.

```hyp
TriggerSystemEvent("customEvent", {"message": "Hallo Welt!"});
```

## Praktische Beispiele

### Datei-Backup-System

```hyp
Focus {
    Trance createBackup(sourcePath, backupDir) {
        if (!FileExists(sourcePath)) {
            observe "Quelldatei existiert nicht: " + sourcePath;
            return false;
        }

        if (!DirectoryExists(backupDir)) {
            CreateDirectory(backupDir);
        }

        induce timestamp = Timestamp();
        induce backupPath = backupDir + "/backup_" + timestamp + ".txt";

        CopyFile(sourcePath, backupPath);
        observe "Backup erstellt: " + backupPath;
        return true;
    }

    entrance {
        induce sourceFile = "important.txt";
        induce backupDirectory = "backups";

        if (createBackup(sourceFile, backupDirectory)) {
            induce backupFiles = ListFiles(backupDirectory);
            observe "Anzahl Backups: " + ArrayLength(backupFiles);
        }
    }
} Relax;
```

### System-Monitoring

```hyp
Focus {
    entrance {
        // System-Informationen sammeln
        induce sysInfo = GetSystemInfo();
        induce memInfo = GetMemoryInfo();
        induce diskInfo = GetDiskInfo();

        observe "=== System-Status ===";
        observe "OS: " + sysInfo.os;
        observe "RAM: " + memInfo.used + "/" + memInfo.total + " MB";

        // Festplatten-Status
        for (induce drive in diskInfo) {
            induce usagePercent = (drive.total - drive.free) / drive.total * 100;
            observe "Laufwerk " + drive.letter + ": " + Round(usagePercent, 1) + "% belegt";
        }

        // Prozess-Liste (Top 5)
        induce processes = GetProcessList();
        induce sortedProcesses = Sort(processes, function(a, b) {
            return b.memory - a.memory;
        });

        observe "Top 5 Prozesse (nach Speicher):";
        for (induce i = 0; i < Min(5, ArrayLength(sortedProcesses)); induce i = i + 1) {
            induce proc = ArrayGet(sortedProcesses, i);
            observe "  " + proc.name + ": " + proc.memory + " MB";
        }
    }
} Relax;
```

### Automatisierte Dateiverarbeitung

```hyp
Focus {
    entrance {
        induce inputDir = "input";
        induce outputDir = "output";
        induce processedDir = "processed";

        // Verzeichnisse erstellen
        if (!DirectoryExists(outputDir)) CreateDirectory(outputDir);
        if (!DirectoryExists(processedDir)) CreateDirectory(processedDir);

        // Alle Dateien im Eingabeverzeichnis verarbeiten
        induce files = ListFiles(inputDir);

        for (induce i = 0; i < ArrayLength(files); induce i = i + 1) {
            induce file = ArrayGet(files, i);
            induce inputPath = inputDir + "/" + file;
            induce outputPath = outputDir + "/processed_" + file;
            induce processedPath = processedDir + "/" + file;

            // Datei verarbeiten
            induce content = ReadFile(inputPath);
            induce processedContent = ToUpper(content); // Beispiel-Verarbeitung

            WriteFile(outputPath, processedContent);
            MoveFile(inputPath, processedPath);

            observe "Verarbeitet: " + file;
        }

        observe "Verarbeitung abgeschlossen. " + ArrayLength(files) + " Dateien verarbeitet.";
    }
} Relax;
```

### Netzwerk-Monitoring

```hyp
Focus {
    entrance {
        induce hosts = ["google.com", "github.com", "stackoverflow.com"];

        observe "=== Netzwerk-Status ===";

        for (induce i = 0; i < ArrayLength(hosts); induce i = i + 1) {
            induce host = ArrayGet(hosts, i);
            induce startTime = Timestamp();

            try {
                induce result = ExecuteCommand("ping -n 1 " + host);
                induce endTime = Timestamp();
                induce responseTime = (endTime - startTime) * 1000; // in ms

                if (Contains(result, "TTL=")) {
                    observe host + ": Online (" + Round(responseTime, 0) + "ms)";
                } else {
                    observe host + ": Offline";
                }
            } catch {
                observe host + ": Fehler beim Ping";
            }
        }
    }
} Relax;
```

### Konfigurations-Management

```hyp
Focus {
    entrance {
        induce configFile = "config.json";
        induce defaultConfig = {
            "server": "localhost",
            "port": 8080,
            "timeout": 30,
            "debug": false
        };

        // Konfiguration laden oder Standard erstellen
        if (FileExists(configFile)) {
            induce configContent = ReadFile(configFile);
            induce config = ParseJSON(configContent);
            observe "Konfiguration geladen";
        } else {
            induce config = defaultConfig;
            WriteFile(configFile, StringifyJSON(config));
            observe "Standard-Konfiguration erstellt";
        }

        // Konfiguration verwenden
        observe "Server: " + config.server + ":" + config.port;
        observe "Timeout: " + config.timeout + " Sekunden";
        observe "Debug-Modus: " + config.debug;

        // Konfiguration aktualisieren
        config.timeout = 60;
        WriteFile(configFile, StringifyJSON(config));
        observe "Konfiguration aktualisiert";
    }
} Relax;
```

## Best Practices

### Fehlerbehandlung

```hyp
Trance safeFileOperation(operation) {
    try {
        return operation();
    } catch (error) {
        observe "Fehler: " + error;
        return false;
    }
}

// Verwendung
safeFileOperation(function() {
    return ReadFile("nonexistent.txt");
});
```

### Ressourcen-Management

```hyp
// Temporäre Dateien automatisch löschen
induce tempFile = "temp_" + Timestamp() + ".txt";
WriteFile(tempFile, "Temporäre Daten");

// Verarbeitung...

// Aufräumen
if (FileExists(tempFile)) {
    DeleteFile(tempFile);
}
```

### Sicherheit

```hyp
// Pfad-Validierung
Trance isValidPath(path) {
    if (Contains(path, "..")) return false;
    if (Contains(path, "\\")) return false;
    return true;
}

// Sichere Dateioperation
if (isValidPath(userInput)) {
    ReadFile(userInput);
} else {
    observe "Ungültiger Pfad!";
}
```

## Nächste Schritte

- [Beispiele](../examples/system-examples) - Praktische System-Beispiele
- [CLI-Erweiterungen](../cli/advanced-commands) - Erweiterte CLI-Features
- [Runtime-Features](../enterprise/features) - Runtime-Funktionen

---

**System-Funktionen gemeistert? Dann schaue dir die [Beispiele](../examples/system-examples) an!** 🚀
