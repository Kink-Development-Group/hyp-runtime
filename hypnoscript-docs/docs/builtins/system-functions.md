---
sidebar_position: 6
---

# System Functions

System functions enable interaction with the operating system, filesystem, processes, and environment variables.

## Filesystem Operations

### ReadFile(path)

Reads the contents of a file as a string.

```hyp
induce content = ReadFile("config.txt");
observe content;
```

### WriteFile(path, content)

Writes content to a file.

```hyp
WriteFile("output.txt", "Hallo Welt!");
```

### AppendFile(path, content)

Appends content to an existing file.

```hyp
AppendFile("log.txt", "Neuer Eintrag: " + Now());
```

### FileExists(path)

Checks if a file exists.

```hyp
if (FileExists("config.json")) {
    induce config = ReadFile("config.json");
    // Verarbeite Konfiguration
}
```

### DeleteFile(path)

Deletes a file.

```hyp
if (FileExists("temp.txt")) {
    DeleteFile("temp.txt");
}
```

### CopyFile(source, destination)

Copies a file.

```hyp
CopyFile("source.txt", "backup.txt");
```

### MoveFile(source, destination)

Moves a file.

```hyp
MoveFile("old.txt", "new.txt");
```

### GetFileSize(path)

Returns the size of a file in bytes.

```hyp
induce size = GetFileSize("large.txt");
observe "Dateigröße: " + size + " Bytes";
```

### GetFileInfo(path)

Returns information about a file.

```hyp
induce info = GetFileInfo("document.txt");
observe "Erstellt: " + info.created;
observe "Geändert: " + info.modified;
observe "Größe: " + info.size + " Bytes";
```

## Directory Operations

### CreateDirectory(path)

Creates a directory.

```hyp
CreateDirectory("logs");
```

### DirectoryExists(path)

Checks if a directory exists.

```hyp
if (!DirectoryExists("output")) {
    CreateDirectory("output");
}
```

### ListFiles(path)

Lists all files in a directory.

```hyp
induce files = ListFiles(".");
for (induce i = 0; i < ArrayLength(files); induce i = i + 1) {
    observe ArrayGet(files, i);
}
```

### ListDirectories(path)

Lists all subdirectories.

```hyp
induce dirs = ListDirectories(".");
observe "Unterverzeichnisse: " + dirs;
```

### DeleteDirectory(path, recursive)

Deletes a directory.

```hyp
DeleteDirectory("temp", true); // Recursively delete
```

### GetCurrentDirectory()

Returns the current working directory.

```hyp
induce cwd = GetCurrentDirectory();
observe "Aktuelles Verzeichnis: " + cwd;
```

### ChangeDirectory(path)

Changes the working directory.

```hyp
ChangeDirectory("../data");
```

## Process Management

### ExecuteCommand(command)

Executes a system command.

```hyp
induce result = ExecuteCommand("dir");
observe result;
```

### ExecuteCommandAsync(command)

Executes a system command asynchronously.

```hyp
induce process = ExecuteCommandAsync("ping google.com");
// Process runs in background
```

### KillProcess(processId)

Terminates a process.

```hyp
induce pid = 1234;
KillProcess(pid);
```

### GetProcessList()

Returns a list of all running processes.

```hyp
induce processes = GetProcessList();
for (induce i = 0; i < ArrayLength(processes); induce i = i + 1) {
    induce proc = ArrayGet(processes, i);
    observe proc.name + " (PID: " + proc.id + ")";
}
```

### GetCurrentProcessId()

Returns the process ID of the current script.

```hyp
induce pid = GetCurrentProcessId();
observe "Aktuelle PID: " + pid;
```

## Environment Variables

### GetEnvironmentVariable(name)

Reads an environment variable.

```hyp
induce path = GetEnvironmentVariable("PATH");
induce user = GetEnvironmentVariable("USERNAME");
```

### SetEnvironmentVariable(name, value)

Sets an environment variable.

```hyp
SetEnvironmentVariable("MY_VAR", "mein_wert");
```

### GetAllEnvironmentVariables()

Returns all environment variables.

```hyp
induce env = GetAllEnvironmentVariables();
for (induce key in env) {
    observe key + " = " + env[key];
}
```

## System Information

### GetSystemInfo()

Returns general system information.

```hyp
induce sysInfo = GetSystemInfo();
observe "Betriebssystem: " + sysInfo.os;
observe "Architektur: " + sysInfo.architecture;
observe "Prozessoren: " + sysInfo.processors;
```

### GetMemoryInfo()

Returns memory information.

```hyp
induce memInfo = GetMemoryInfo();
observe "Gesamter RAM: " + memInfo.total + " MB";
observe "Verfügbarer RAM: " + memInfo.available + " MB";
observe "Verwendeter RAM: " + memInfo.used + " MB";
```

### GetDiskInfo()

Returns disk information.

```hyp
induce diskInfo = GetDiskInfo();
for (induce drive in diskInfo) {
    observe "Laufwerk " + drive.letter + ":";
    observe "  Gesamt: " + drive.total + " GB";
    observe "  Verfügbar: " + drive.free + " GB";
}
```

### GetNetworkInfo()

Returns network information.

```hyp
induce netInfo = GetNetworkInfo();
observe "Hostname: " + netInfo.hostname;
observe "IP-Adresse: " + netInfo.ipAddress;
```

## Network Operations

### DownloadFile(url, destination)

Downloads a file from a URL.

```hyp
DownloadFile("https://example.com/file.txt", "downloaded.txt");
```

### UploadFile(url, filePath)

Uploads a file to a URL.

```hyp
UploadFile("https://example.com/upload", "local.txt");
```

### HttpGet(url)

Executes an HTTP GET request.

```hyp
induce response = HttpGet("https://api.example.com/data");
induce data = ParseJSON(response);
```

### HttpPost(url, data)

Executes an HTTP POST request.

```hyp
induce postData = StringifyJSON({"name": "Max", "age": 30});
induce response = HttpPost("https://api.example.com/users", postData);
```

## Registry Operations (Windows)

### ReadRegistryValue(key, valueName)

Reads a registry value.

```hyp
induce version = ReadRegistryValue("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion", "ProductName");
```

### WriteRegistryValue(key, valueName, value)

Writes a registry value.

```hyp
WriteRegistryValue("HKEY_CURRENT_USER\\Software\\MyApp", "Version", "1.0");
```

### DeleteRegistryValue(key, valueName)

Deletes a registry value.

```hyp
DeleteRegistryValue("HKEY_CURRENT_USER\\Software\\MyApp", "TempValue");
```

## System Events

### OnSystemEvent(eventType, callback)

Registers an event handler for system events.

```hyp
OnSystemEvent("fileChanged", function(path) {
    observe "Datei geändert: " + path;
});
```

### TriggerSystemEvent(eventType, data)

Triggers a system event.

```hyp
TriggerSystemEvent("customEvent", {"message": "Hallo Welt!"});
```

## Practical Examples

### File Backup System

```hyp
Focus {
    suggestion createBackup(sourcePath, backupDir) {
        if (!FileExists(sourcePath)) {
            observe "Quelldatei existiert nicht: " + sourcePath;
            awaken false;
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

### Automatisierte Fileverarbeitung

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
suggestion safeFileOperation(operation) {
    try {
        awaken operation();
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
// Automatically delete temporary files
induce tempFile = "temp_" + Timestamp() + ".txt";
WriteFile(tempFile, "Temporäre Daten");

// Verarbeitung...

// Cleanup
if (FileExists(tempFile)) {
    DeleteFile(tempFile);
}
```

### Sicherheit

```hyp
// Pfad-Validierung
suggestion isValidPath(path) {
    if (Contains(path, "..")) awaken false;
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

## Next Steps

- [Examples](../examples/system-examples) - Practical system examples
- [CLI Extensions](../cli/advanced-commands) - Advanced CLI features
- [Runtime Features](../enterprise/features) - Runtime functions

---

**System functions mastered? Then check out the [Examples](../examples/system-examples)!** 🚀
