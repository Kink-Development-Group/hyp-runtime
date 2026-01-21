---
title: Performance Functions
---

# Performance Functions

HypnoScript provides extensive performance functions for monitoring and optimizing scripts.

## Overview

Performance functions let you monitor and optimize execution time, memory usage, and other performance metrics for HypnoScript programs.

## Basic Performance Functions

### Benchmark

Measures execution time for a function over multiple iterations.

```hyp
induce result = Benchmark(function() {
    // Code to measure
    return someValue;
}, 1000); // 1000 iterations

observe "Average execution time: " + result + " ms";
```

**Parameters:**

- `function`: Function to measure
- `iterations`: Number of iterations

**Return value:** Average execution time in milliseconds

### GetPerformanceMetrics

Collects comprehensive performance metrics for the current system.

```hyp
induce metrics = GetPerformanceMetrics();
observe "CPU usage: " + metrics.cpuUsage + "%";
observe "Memory usage: " + metrics.memoryUsage + " MB";
observe "Available memory: " + metrics.availableMemory + " MB";
```

**Return value:** Dictionary with performance metrics

### GetExecutionTime

Measures execution time of a code block.

```hyp
induce startTime = GetCurrentTime();
// Code to measure
induce endTime = GetCurrentTime();
induce executionTime = (endTime - startTime) * 1000; // in ms
observe "Execution time: " + executionTime + " ms";
```

## Memory Management

### GetMemoryUsage

Returns current memory usage.

```hyp
induce memoryUsage = GetMemoryUsage();
observe "Current memory usage: " + memoryUsage + " MB";
```

**Return value:** Memory usage in megabytes

### GetAvailableMemory

Returns available memory.

```hyp
induce availableMemory = GetAvailableMemory();
observe "Available memory: " + availableMemory + " MB";
```

**Return value:** Available memory in megabytes

### ForceGarbageCollection

Forces garbage collection.

```hyp
ForceGarbageCollection();
observe "Garbage collection completed";
```

## CPU Monitoring

### GetCPUUsage

Returns current CPU usage.

```hyp
induce cpuUsage = GetCPUUsage();
observe "CPU usage: " + cpuUsage + "%";
```

**Return value:** CPU usage in percent

### GetProcessorCount

Returns the number of available processors.

```hyp
induce processorCount = GetProcessorCount();
observe "Processor count: " + processorCount;
```

**Return value:** Processor count

## Profiling Functions

### StartProfiling

Starts performance profiling.

```hyp
StartProfiling("my-profile");
// Code to profile
StopProfiling();
induce profileData = GetProfileData("my-profile");
observe "Profile data: " + profileData;
```

**Parameters:**

- `profileName`: Profile name

### StopProfiling

Stops performance profiling.

```hyp
StartProfiling("test");
// Code
StopProfiling();
```

### GetProfileData

Returns profile data.

```hyp
induce profileData = GetProfileData("my-profile");
observe "Function calls: " + profileData.functionCalls;
observe "Execution time: " + profileData.executionTime;
```

**Parameters:**

- `profileName`: Profile name

**Return value:** Dictionary with profile data

## Optimization Functions

### OptimizeMemory

Executes memory optimizations.

```hyp
OptimizeMemory();
observe "Memory optimization completed";
```

### OptimizeCPU

Executes CPU optimizations.

```hyp
OptimizeCPU();
observe "CPU optimization completed";
```

## Monitoring Functions

### StartMonitoring

Starts continuous performance monitoring.

```hyp
StartMonitoring(5000); // Every 5 seconds
// Code
StopMonitoring();
```

**Parameters:**

- `interval`: Interval in milliseconds

### StopMonitoring

Stops performance monitoring.

```hyp
StartMonitoring(1000);
// Code
StopMonitoring();
```

### GetMonitoringData

Returns monitoring data.

```hyp
induce monitoringData = GetMonitoringData();
observe "Average CPU usage: " + monitoringData.avgCpuUsage;
observe "Peak memory usage: " + monitoringData.maxMemoryUsage;
```

**Return value:** Dictionary with monitoring data

## Advanced Performance Functions

### GetSystemInfo

Returns detailed system information.

```hyp
induce systemInfo = GetSystemInfo();
observe "Operating system: " + systemInfo.os;
observe "Architecture: " + systemInfo.architecture;
observe "Framework version: " + systemInfo.frameworkVersion;
```

**Return value:** Dictionary with system information

### GetProcessInfo

Returns information about the current process.

```hyp
induce processInfo = GetProcessInfo();
observe "Process ID: " + processInfo.processId;
observe "Working set: " + processInfo.workingSet + " MB";
observe "CPU time: " + processInfo.cpuTime + " ms";
```

**Return value:** Dictionary with process information

## Best Practices

### Performance Monitoring

```hyp
Focus {
    entrance {
        // Start monitoring
        StartMonitoring(1000);

        // Performance-critical code
        induce result = Benchmark(function() {
            // Code needing optimization
            induce sum = 0;
            for (induce i = 0; i < 1000000; induce i = i + 1) {
                sum = sum + i;
            }
            return sum;
        }, 100);

        // Stop monitoring
        StopMonitoring();

        // Evaluate results
        induce monitoringData = GetMonitoringData();
        if (monitoringData.avgCpuUsage > 80) {
            observe "WARNING: High CPU usage detected!";
        }

        observe "Benchmark result: " + result + " ms";
    }
} Relax;
```

### Memory Optimization

```hyp
Focus {
    entrance {
        induce initialMemory = GetMemoryUsage();

        // Memory-intensive operations
        induce largeArray = [];
        for (induce i = 0; i < 100000; induce i = i + 1) {
            ArrayPush(largeArray, "Item " + i);
        }

        induce memoryAfterOperation = GetMemoryUsage();
        observe "Memory increase: " + (memoryAfterOperation - initialMemory) + " MB";

        // Memory optimization
        ForceGarbageCollection();
        OptimizeMemory();

        induce memoryAfterOptimization = GetMemoryUsage();
        observe "Memory after optimization: " + memoryAfterOptimization + " MB";
    }
} Relax;
```

### Profiling Workflow

```hyp
Focus {
    entrance {
        // Start profiling
        StartProfiling("main-operation");

        // Main operation
        induce result = PerformMainOperation();

        // Stop profiling
        StopProfiling();

        // Analyze profile data
        induce profileData = GetProfileData("main-operation");

        if (profileData.executionTime > 1000) {
            observe "WARNING: Operation takes longer than 1 second!";
        }

        observe "Profile result: " + profileData;
    }
} Relax;
```

## Error Handling

Performance functions can throw errors under unexpected system conditions:

```hyp
Focus {
    entrance {
        try {
            induce metrics = GetPerformanceMetrics();
            observe "Performance metrics: " + metrics;
        } catch (error) {
            observe "Error retrieving performance metrics: " + error;
        }
    }
} Relax;
```

## Next Steps

- [System Functions](./system-functions) - System-specific functions
- [Utility Functions](./utility-functions) - General helper functions
- [Testing Performance](../testing/performance) - Performance testing guide

---

**Mastered performance optimization? Then explore [System Functions](./system-functions)!** ✅
