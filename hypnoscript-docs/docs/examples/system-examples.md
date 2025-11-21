---
sidebar_position: 2
---

# Examples: System Functions

This page shows practical examples for system functions in HypnoScript. The examples are commented and can be used directly or adapted.

## File Operations: Read, Write, Backup

```hyp
Focus {
    entrance {
        // Write file
        WriteFile("beispiel.txt", "Hallo HypnoScript!");
        // Read file
        induce content = ReadFile("beispiel.txt");
        observe "File content: " + content;
        // Create backup
        induce backupName = "beispiel_backup_" + Timestamp() + ".txt";
        CopyFile("beispiel.txt", backupName);
        observe "Backup created: " + backupName;
    }
} Relax;
```

## Directories and File Lists

```hyp
Focus {
    entrance {
        // Create directory
        if (!DirectoryExists("daten")) CreateDirectory("daten");
        // List files
        induce files = ListFiles(".");
        observe "Files in current directory: " + files;
    }
} Relax;
```

## Automated File Processing

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
            observe "Processed: " + file;
        }
    }
} Relax;
```

## Process Management: Execute System Commands

```hyp
Focus {
    entrance {
        induce result = ExecuteCommand("echo Hallo von der Shell!");
        observe "Shell output: " + result;
    }
} Relax;
```

## Reading and Setting Environment Variables

```hyp
Focus {
    entrance {
        SetEnvironmentVariable("MEIN_VAR", "Testwert");
        induce value = GetEnvironmentVariable("MEIN_VAR");
        observe "MEIN_VAR: " + value;
    }
} Relax;
```

## System Information and Monitoring

```hyp
Focus {
    entrance {
        induce sys = GetSystemInfo();
        induce mem = GetMemoryInfo();
        observe "OS: " + sys.os;
        observe "RAM: " + mem.used + "/" + mem.total + " MB used";
    }
} Relax;
```

## Network: HTTP Request and Download

```hyp
Focus {
    entrance {
        induce url = "https://example.com";
        induce response = HttpGet(url);
        observe "HTTP response: " + Substring(response, 0, 100) + "...";
        DownloadFile(url + "/file.txt", "local.txt");
        observe "File downloaded as local.txt";
    }
} Relax;
```

## Error Handling for File Operations

```hyp
Focus {
    suggestion safeRead(path) {
        try {
            awaken ReadFile(path);
        } catch (error) {
            return "Error reading: " + error;
        }
    }
    entrance {
        observe safeRead("nicht_existierend.txt");
    }
} Relax;
```

## Combined System Workflows

```hyp
Focus {
    entrance {
        // Combined backup and monitoring
        induce file = "daten.txt";
        if (FileExists(file)) {
            induce backup = file + ".bak";
            CopyFile(file, backup);
            observe "Backup created: " + backup;
        }
        induce sys = GetSystemInfo();
        observe "System: " + sys.os + " (" + sys.architecture + ")";
    }
} Relax;
```

---

**See also:**

- [System Functions Reference](../builtins/system-functions)
- [Utility Functions Examples](./utility-examples)

