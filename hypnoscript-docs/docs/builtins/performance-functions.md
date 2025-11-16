---
title: Performance Functions
---

# Performance Functions

HypnoScript bietet umfangreiche Performance-Functionen für die Überwachung und Optimierung von Skripten.

## Overview

Performance-Functionen ermöglichen es Ihnen, die Ausführungszeit, Speichernutzung und andere Performance-Metriken Ihrer HypnoScript-Programme zu überwachen und zu optimieren.

## Basic Performance-Functionen

### Benchmark

Misst die Ausführungszeit einer Function über mehrere Iterationen.

```hyp
induce result = Benchmark(function() {
    // Code zum Messen
    return someValue;
}, 1000); // 1000 Iterationen

observe "Durchschnittliche Ausführungszeit: " + result + " ms";
```

**Parameters:**

- `function`: Die zu messende Function
- `iterations`: Anzahl der Iterationen

**Return value:** Durchschnittliche Ausführungszeit in Millisekunden

### GetPerformanceMetrics

Sammelt umfassende Performance-Metriken des aktuellen Systems.

```hyp
induce metrics = GetPerformanceMetrics();
observe "CPU-Auslastung: " + metrics.cpuUsage + "%";
observe "Speichernutzung: " + metrics.memoryUsage + " MB";
observe "Verfügbarer Speicher: " + metrics.availableMemory + " MB";
```

**Return value:** Dictionary mit Performance-Metriken

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

Returns die aktuelle Speichernutzung .

```hyp
induce memoryUsage = GetMemoryUsage();
observe "Aktuelle Speichernutzung: " + memoryUsage + " MB";
```

**Return value:** Speichernutzung in Megabyte

### GetAvailableMemory

Returns den availableen Speicher .

```hyp
induce availableMemory = GetAvailableMemory();
observe "Verfügbarer Speicher: " + availableMemory + " MB";
```

**Return value:** Verfügbarer Speicher in Megabyte

### ForceGarbageCollection

Erzwingt eine Garbage Collection.

```hyp
ForceGarbageCollection();
observe "Garbage Collection durchgeführt";
```

## CPU-Monitoring

### GetCPUUsage

Returns die aktuelle CPU-Auslastung .

```hyp
induce cpuUsage = GetCPUUsage();
observe "CPU-Auslastung: " + cpuUsage + "%";
```

**Return value:** CPU-Auslastung in Prozent

### GetProcessorCount

Returns die Anzahl der availableen Prozessoren .

```hyp
induce processorCount = GetProcessorCount();
observe "Anzahl Prozessoren: " + processorCount;
```

**Return value:** Anzahl der Prozessoren

## Profiling-Functionen

### StartProfiling

Startet das Performance-Profiling.

```hyp
StartProfiling("my-profile");
// Code zum Profilen
StopProfiling();
induce profileData = GetProfileData("my-profile");
observe "Profil-Daten: " + profileData;
```

**Parameters:**

- `profileName`: Name des Profils

### StopProfiling

Stoppt das Performance-Profiling.

```hyp
StartProfiling("test");
// Code
StopProfiling();
```

### GetProfileData

Returns die Profil-Daten .

```hyp
induce profileData = GetProfileData("my-profile");
observe "Funktionsaufrufe: " + profileData.functionCalls;
observe "Ausführungszeit: " + profileData.executionTime;
```

**Parameters:**

- `profileName`: Name des Profils

**Return value:** Dictionary mit Profil-Daten

## Optimierungs-Functionen

### OptimizeMemory

Executes Speicheroptimierungen durch.

```hyp
OptimizeMemory();
observe "Speicheroptimierung durchgeführt";
```

### OptimizeCPU

Executes CPU-Optimierungen durch.

```hyp
OptimizeCPU();
observe "CPU-Optimierung durchgeführt";
```

## Monitoring-Functionen

### StartMonitoring

Startet das kontinuierliche Performance-Monitoring.

```hyp
StartMonitoring(5000); // Alle 5 Sekunden
// Code
StopMonitoring();
```

**Parameters:**

- `interval`: Intervall in Millisekunden

### StopMonitoring

Stoppt das Performance-Monitoring.

```hyp
StartMonitoring(1000);
// Code
StopMonitoring();
```

### GetMonitoringData

Returns die Monitoring-Daten .

```hyp
induce monitoringData = GetMonitoringData();
observe "Durchschnittliche CPU-Auslastung: " + monitoringData.avgCpuUsage;
observe "Maximale Speichernutzung: " + monitoringData.maxMemoryUsage;
```

**Return value:** Dictionary mit Monitoring-Daten

## Advanced Performance-Functionen

### GetSystemInfo

Returns detaillierte System-Informationen .

```hyp
induce systemInfo = GetSystemInfo();
observe "Betriebssystem: " + systemInfo.os;
observe "Architektur: " + systemInfo.architecture;
observe "Framework-Version: " + systemInfo.frameworkVersion;
```

**Return value:** Dictionary mit System-Informationen

### GetProcessInfo

Returns Informationen über den aktuellen Prozess .

```hyp
induce processInfo = GetProcessInfo();
observe "Prozess-ID: " + processInfo.processId;
observe "Arbeitsspeicher: " + processInfo.workingSet + " MB";
observe "CPU-Zeit: " + processInfo.cpuTime + " ms";
```

**Return value:** Dictionary mit Prozess-Informationen

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

Performance-Functionen können bei unerwarteten Systemzuständen Fehler werfen:

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

## Next Steps

- [System Functions](./system-functions) - System-spezifische Functionen
- [Utility Functions](./utility-functions) - Allgemeine Hilfsfunktionen
- [Testing Performance](../testing/performance) - Performance-Testing-Guide

---

**Performance-Optimierung gemeistert? Dann lerne [System Functions](./system-functions) kennen!** ✅
