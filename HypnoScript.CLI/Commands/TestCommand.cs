using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using HypnoScript.CLI;

namespace HypnoScript.CLI.Commands
{
    public static class TestCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== TEST MODE ===");
            AppLogger.Info("🧪 Running HypnoScript Tests...");

            List<string> testFiles;
            if (string.IsNullOrEmpty(filePath))
            {
                // Alle .hyp-Dateien im Projektverzeichnis rekursiv finden
                testFiles = Directory.GetFiles(Directory.GetCurrentDirectory(), "*.hyp", SearchOption.AllDirectories)
                    .OrderBy(f => f).ToList();
            }
            else
            {
                if (!File.Exists(filePath))
                {
                    AppLogger.Error($"File not found: {filePath}");
                    return 2;
                }
                testFiles = new List<string> { filePath };
            }

            if (testFiles.Count == 0)
            {
                AppLogger.Warn("[WARN] No .hyp test files found.");
                return 0;
            }

            int passed = 0, failed = 0;
            var results = new List<(string file, bool ok, TimeSpan duration, string? error)>();

            foreach (var testFile in testFiles)
            {
                var sw = System.Diagnostics.Stopwatch.StartNew();
                try
                {
                    int exitCode = RunSingleTest(testFile, debug, verbose);
                    sw.Stop();
                    if (exitCode == 0)
                    {
                        results.Add((testFile, true, sw.Elapsed, null));
                        passed++;
                    }
                    else
                    {
                        results.Add((testFile, false, sw.Elapsed, $"Exit code: {exitCode}"));
                        failed++;
                    }
                }
                catch (Exception ex)
                {
                    sw.Stop();
                    string errorMessage = ex.Message;
                    bool isAssertionFailure = errorMessage.Contains("Assertion failed") || errorMessage.StartsWith("Assertion failed");

                    if (isAssertionFailure)
                    {
                        // Assertion-Fehler speziell hervorheben
                        errorMessage = $"ASSERTION FAILED: {errorMessage}";
                    }

                    results.Add((testFile, false, sw.Elapsed, errorMessage));
                    failed++;
                }
            }

            // Testreport
            AppLogger.Info("\n=== Test Results ===");
            foreach (var (file, ok, duration, error) in results)
            {
                if (ok)
                {
                    AppLogger.Info($"[OK]     {System.IO.Path.GetFileName(file),-30} ({duration.TotalMilliseconds:F0} ms)");
                }
                else
                {
                    if (error?.Contains("ASSERTION FAILED") == true)
                    {
                        AppLogger.Error($"[ASSERT] {System.IO.Path.GetFileName(file),-30} ({duration.TotalMilliseconds:F0} ms)");
                        AppLogger.Error($"         └─ {error}");
                    }
                    else
                    {
                        AppLogger.Error($"[FAIL]   {System.IO.Path.GetFileName(file),-30} ({duration.TotalMilliseconds:F0} ms)  {error}");
                    }
                }
            }

            AppLogger.Info($"\nSummary: {passed} passed, {failed} failed, {testFiles.Count} total");
            if (failed > 0)
            {
                AppLogger.Warn($"⚠️  {failed} test(s) failed. Check the output above for details.");
            }

            return failed == 0 ? 0 : 1;
        }

        private static int RunSingleTest(string filePath, bool debug, bool verbose)
        {
            // Die Run-Logik aus RunCommand wiederverwenden
            return RunCommand.Execute(filePath, debug, verbose);
        }
    }
}
