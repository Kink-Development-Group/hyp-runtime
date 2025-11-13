---
title: Performance Functions
---

# Performance Functions

HypnoScript bietet umfangreiche Performance-Funktionen für die Überwachung und Optimierung von Skripten.

## Übersicht

Performance-Funktionen ermöglichen es Ihnen, die Ausführungszeit, Speichernutzung und andere Performance-Metriken Ihrer HypnoScript-Programme zu überwachen und zu optimieren.

## Grundlegende Performance-Funktionen

### Benchmark

Misst die Ausführungszeit einer Funktion über mehrere Iterationen.

```hyp
induce result = Benchmark(function() {
    // Code zum Messen
    return someValue;
}, 1000); // 1000 Iterationen

observe "Durchschnittliche Ausführungszeit: " + result + " ms";
```

**Parameter:**

- `function`: Die zu messende Funktion
- `iterations`: Anzahl der Iterationen

**Rückgabewert:** Durchschnittliche Ausführungszeit in Millisekunden

### GetPerformanceMetrics

Sammelt umfassende Performance-Metriken des aktuellen Systems.

```hyp
induce metrics = GetPerformanceMetrics();
observe "CPU-Auslastung: " + metrics.cpuUsage + "%";
observe "Speichernutzung: " + metrics.memoryUsage + " MB";
observe "Verfügbarer Speicher: " + metrics.availableMemory + " MB";
```

**Rückgabewert:** Dictionary mit Performance-Metriken

### GetExecutionTime

Misst die Ausführungszeit eines Code-Blocks.

```hyp
induce startTime = GetCurrentTime();
// Code zum Messen
induce endTime = GetCurrentTime();
induce executionTime = (endTime - startTime) * 1000; // in ms
observe "Ausführungszeit: " + executionTime + " ms";
```

## Speicher-Management

### GetMemoryUsage

Gibt die aktuelle Speichernutzung zurück.

```hyp
induce memoryUsage = GetMemoryUsage();
observe "Aktuelle Speichernutzung: " + memoryUsage + " MB";
```

**Rückgabewert:** Speichernutzung in Megabyte

### GetAvailableMemory

Gibt den verfügbaren Speicher zurück.

```hyp
induce availableMemory = GetAvailableMemory();
observe "Verfügbarer Speicher: " + availableMemory + " MB";
```

**Rückgabewert:** Verfügbarer Speicher in Megabyte

### ForceGarbageCollection

Erzwingt eine Garbage Collection.

```hyp
ForceGarbageCollection();
observe "Garbage Collection durchgeführt";
```

## CPU-Monitoring

### GetCPUUsage

Gibt die aktuelle CPU-Auslastung zurück.

```hyp
induce cpuUsage = GetCPUUsage();
observe "CPU-Auslastung: " + cpuUsage + "%";
```

**Rückgabewert:** CPU-Auslastung in Prozent

### GetProcessorCount

Gibt die Anzahl der verfügbaren Prozessoren zurück.

```hyp
induce processorCount = GetProcessorCount();
observe "Anzahl Prozessoren: " + processorCount;
```

**Rückgabewert:** Anzahl der Prozessoren

## Profiling-Funktionen

### StartProfiling

Startet das Performance-Profiling.

```hyp
StartProfiling("my-profile");
// Code zum Profilen
StopProfiling();
induce profileData = GetProfileData("my-profile");
observe "Profil-Daten: " + profileData;
```

**Parameter:**

- `profileName`: Name des Profils

### StopProfiling

Stoppt das Performance-Profiling.

```hyp
StartProfiling("test");
// Code
StopProfiling();
```

### GetProfileData

Gibt die Profil-Daten zurück.

```hyp
induce profileData = GetProfileData("my-profile");
observe "Funktionsaufrufe: " + profileData.functionCalls;
observe "Ausführungszeit: " + profileData.executionTime;
```

**Parameter:**

- `profileName`: Name des Profils

**Rückgabewert:** Dictionary mit Profil-Daten

## Optimierungs-Funktionen

### OptimizeMemory

Führt Speicheroptimierungen durch.

```hyp
OptimizeMemory();
observe "Speicheroptimierung durchgeführt";
```

### OptimizeCPU

Führt CPU-Optimierungen durch.

```hyp
OptimizeCPU();
observe "CPU-Optimierung durchgeführt";
```

## Monitoring-Funktionen

### StartMonitoring

Startet das kontinuierliche Performance-Monitoring.

```hyp
StartMonitoring(5000); // Alle 5 Sekunden
// Code
StopMonitoring();
```

**Parameter:**

- `interval`: Intervall in Millisekunden

### StopMonitoring

Stoppt das Performance-Monitoring.

```hyp
StartMonitoring(1000);
// Code
StopMonitoring();
```

### GetMonitoringData

Gibt die Monitoring-Daten zurück.

```hyp
induce monitoringData = GetMonitoringData();
observe "Durchschnittliche CPU-Auslastung: " + monitoringData.avgCpuUsage;
observe "Maximale Speichernutzung: " + monitoringData.maxMemoryUsage;
```

**Rückgabewert:** Dictionary mit Monitoring-Daten

## Erweiterte Performance-Funktionen

### GetSystemInfo

Gibt detaillierte System-Informationen zurück.

```hyp
induce systemInfo = GetSystemInfo();
observe "Betriebssystem: " + systemInfo.os;
observe "Architektur: " + systemInfo.architecture;
observe "Framework-Version: " + systemInfo.frameworkVersion;
```

**Rückgabewert:** Dictionary mit System-Informationen

### GetProcessInfo

Gibt Informationen über den aktuellen Prozess zurück.

```hyp
induce processInfo = GetProcessInfo();
observe "Prozess-ID: " + processInfo.processId;
observe "Arbeitsspeicher: " + processInfo.workingSet + " MB";
observe "CPU-Zeit: " + processInfo.cpuTime + " ms";
```

**Rückgabewert:** Dictionary mit Prozess-Informationen

## Best Practices

### Performance-Monitoring

```hyp
Focus {
    entrance {
        // Monitoring starten
        StartMonitoring(1000);

        // Performance-kritischer Code
        induce result = Benchmark(function() {
            // Optimierungsbedürftiger Code
            induce sum = 0;
            for (induce i = 0; i < 1000000; induce i = i + 1) {
                sum = sum + i;
            }
            return sum;
        }, 100);

        // Monitoring stoppen
        StopMonitoring();

        // Ergebnisse auswerten
        induce monitoringData = GetMonitoringData();
        if (monitoringData.avgCpuUsage > 80) {
            observe "WARNUNG: Hohe CPU-Auslastung erkannt!";
        }

        observe "Benchmark-Ergebnis: " + result + " ms";
    }
} Relax;
```

### Speicheroptimierung

```hyp
Focus {
    entrance {
        induce initialMemory = GetMemoryUsage();

        // Speicherintensive Operationen
        induce largeArray = [];
        for (induce i = 0; i < 100000; induce i = i + 1) {
            ArrayPush(largeArray, "Element " + i);
        }

        induce memoryAfterOperation = GetMemoryUsage();
        observe "Speicherzuwachs: " + (memoryAfterOperation - initialMemory) + " MB";

        // Speicheroptimierung
        ForceGarbageCollection();
        OptimizeMemory();

        induce memoryAfterOptimization = GetMemoryUsage();
        observe "Speicher nach Optimierung: " + memoryAfterOptimization + " MB";
    }
} Relax;
```

### Profiling-Workflow

```hyp
Focus {
    entrance {
        // Profiling starten
        StartProfiling("main-operation");

        // Hauptoperation
        induce result = PerformMainOperation();

        // Profiling stoppen
        StopProfiling();

        // Profil-Daten analysieren
        induce profileData = GetProfileData("main-operation");

        if (profileData.executionTime > 1000) {
            observe "WARNUNG: Operation dauert länger als 1 Sekunde!";
        }

        observe "Profil-Ergebnis: " + profileData;
    }
} Relax;
```

## Fehlerbehandlung

Performance-Funktionen können bei unerwarteten Systemzuständen Fehler werfen:

```hyp
Focus {
    entrance {
        try {
            induce metrics = GetPerformanceMetrics();
            observe "Performance-Metriken: " + metrics;
        } catch (error) {
            observe "Fehler beim Abrufen der Performance-Metriken: " + error;
        }
    }
} Relax;
```

## Nächste Schritte

- [System Functions](./system-functions) - System-spezifische Funktionen
- [Utility Functions](./utility-functions) - Allgemeine Hilfsfunktionen
- [Testing Performance](../testing/performance) - Performance-Testing-Guide

---

**Performance-Optimierung gemeistert? Dann lerne [System Functions](./system-functions) kennen!** ✅
