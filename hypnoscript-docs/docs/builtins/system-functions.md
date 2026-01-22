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
WriteFile("output.txt", "Hello World!");
```

### AppendFile(path, content)

Appends content to an existing file.

```hyp
AppendFile("log.txt", "New entry: " + Now());
```

### FileExists(path)

Checks if a file exists.

```hyp
if (FileExists("config.json")) {
    induce config = ReadFile("config.json");
    // Process configuration
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
observe "File size: " + size + " bytes";
```

### GetFileInfo(path)

Returns information about a file.

```hyp
induce info = GetFileInfo("document.txt");
observe "Created: " + info.created;
observe "Modified: " + info.modified;
observe "Size: " + info.size + " bytes";
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
observe "Subdirectories: " + dirs;
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
observe "Current directory: " + cwd;
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
observe "Current PID: " + pid;
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
SetEnvironmentVariable("MY_VAR", "my_value");
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
observe "Operating system: " + sysInfo.os;
observe "Architecture: " + sysInfo.architecture;
observe "Processors: " + sysInfo.processors;
```

### GetMemoryInfo()

Returns memory information.

```hyp
induce memInfo = GetMemoryInfo();
observe "Total RAM: " + memInfo.total + " MB";
observe "Available RAM: " + memInfo.available + " MB";
observe "Used RAM: " + memInfo.used + " MB";
```

### GetDiskInfo()

Returns disk information.

```hyp
induce diskInfo = GetDiskInfo();
for (induce drive in diskInfo) {
    observe "Drive " + drive.letter + ":";
    observe "  Total: " + drive.total + " GB";
    observe "  Free: " + drive.free + " GB";
}
```

### GetNetworkInfo()

Returns network information.

```hyp
induce netInfo = GetNetworkInfo();
observe "Hostname: " + netInfo.hostname;
observe "IP address: " + netInfo.ipAddress;
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
    observe "File changed: " + path;
});
```

### TriggerSystemEvent(eventType, data)

Triggers a system event.

```hyp
TriggerSystemEvent("customEvent", {"message": "Hello World!"});
```

## Practical Examples

### File Backup System

```hyp
Focus {
    suggestion createBackup(sourcePath, backupDir) {
        if (!FileExists(sourcePath)) {
            observe "Source file does not exist: " + sourcePath;
            awaken false;
        }

        if (!DirectoryExists(backupDir)) {
            CreateDirectory(backupDir);
        }

        induce timestamp = Timestamp();
        induce backupPath = backupDir + "/backup_" + timestamp + ".txt";

        CopyFile(sourcePath, backupPath);
        observe "Backup created: " + backupPath;
        return true;
    }

    entrance {
        induce sourceFile = "important.txt";
        induce backupDirectory = "backups";

        if (createBackup(sourceFile, backupDirectory)) {
            induce backupFiles = ListFiles(backupDirectory);
            observe "Number of backups: " + ArrayLength(backupFiles);
        }
    }
} Relax;
```

### System Monitoring

```hyp
Focus {
    entrance {
        // Collect system information
        induce sysInfo = GetSystemInfo();
        induce memInfo = GetMemoryInfo();
        induce diskInfo = GetDiskInfo();

        observe "=== System Status ===";
        observe "OS: " + sysInfo.os;
        observe "RAM: " + memInfo.used + "/" + memInfo.total + " MB";

        // Disk status
        for (induce drive in diskInfo) {
            induce usagePercent = (drive.total - drive.free) / drive.total * 100;
            observe "Drive " + drive.letter + ": " + Round(usagePercent, 1) + "% used";
        }

        // Process list (top 5)
        induce processes = GetProcessList();
        induce sortedProcesses = Sort(processes, function(a, b) {
            return b.memory - a.memory;
        });

        observe "Top 5 processes (by memory):";
        for (induce i = 0; i < Min(5, ArrayLength(sortedProcesses)); induce i = i + 1) {
            induce proc = ArrayGet(sortedProcesses, i);
            observe "  " + proc.name + ": " + proc.memory + " MB";
        }
    }
} Relax;
```

### Automated File Processing

```hyp
Focus {
    entrance {
        induce inputDir = "input";
        induce outputDir = "output";
        induce processedDir = "processed";

        // Create directories
        if (!DirectoryExists(outputDir)) CreateDirectory(outputDir);
        if (!DirectoryExists(processedDir)) CreateDirectory(processedDir);

        // Process all files in the input directory
        induce files = ListFiles(inputDir);

        for (induce i = 0; i < ArrayLength(files); induce i = i + 1) {
            induce file = ArrayGet(files, i);
            induce inputPath = inputDir + "/" + file;
            induce outputPath = outputDir + "/processed_" + file;
            induce processedPath = processedDir + "/" + file;

            // Process file
            induce content = ReadFile(inputPath);
            induce processedContent = ToUpper(content); // Example processing

            WriteFile(outputPath, processedContent);
            MoveFile(inputPath, processedPath);

            observe "Processed: " + file;
        }

        observe "Processing complete. " + ArrayLength(files) + " files processed.";
    }
} Relax;
```

### Network Monitoring

```hyp
Focus {
    entrance {
        induce hosts = ["google.com", "github.com", "stackoverflow.com"];

        observe "=== Network Status ===";

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
                observe host + ": Ping error";
            }
        }
    }
} Relax;
```

### Configuration Management

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

        // Load configuration or create default
        if (FileExists(configFile)) {
            induce configContent = ReadFile(configFile);
            induce config = ParseJSON(configContent);
            observe "Configuration loaded";
        } else {
            induce config = defaultConfig;
            WriteFile(configFile, StringifyJSON(config));
            observe "Default configuration created";
        }

        // Use configuration
        observe "Server: " + config.server + ":" + config.port;
        observe "Timeout: " + config.timeout + " seconds";
        observe "Debug mode: " + config.debug;

        // Update configuration
        config.timeout = 60;
        WriteFile(configFile, StringifyJSON(config));
        observe "Configuration updated";
    }
} Relax;
```

## Best Practices

### Error Handling

```hyp
suggestion safeFileOperation(operation) {
    try {
        awaken operation();
    } catch (error) {
        observe "Error: " + error;
        return false;
    }
}

// Usage
safeFileOperation(function() {
    return ReadFile("nonexistent.txt");
});
```

### Resource Management

```hyp
// Automatically delete temporary files
induce tempFile = "temp_" + Timestamp() + ".txt";
WriteFile(tempFile, "Temporary data");

// Processing...

// Cleanup
if (FileExists(tempFile)) {
    DeleteFile(tempFile);
}
```

### Security

```hyp
// Path validation
suggestion isValidPath(path) {
    if (Contains(path, "..")) awaken false;
    if (Contains(path, "\\")) return false;
    return true;
}

// Safe file operation
if (isValidPath(userInput)) {
    ReadFile(userInput);
} else {
    observe "Invalid path!";
}
```

## Next Steps

- [Examples](../examples/system-examples) - Practical system examples
- [CLI Extensions](../cli/advanced-commands) - Advanced CLI features
- [Runtime Features](../enterprise/features) - Runtime functions

---

**System functions mastered? Then check out the [Examples](../examples/system-examples)!** 🚀
