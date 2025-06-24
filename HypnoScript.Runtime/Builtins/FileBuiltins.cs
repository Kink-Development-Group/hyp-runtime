using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// File/Directory Builtins für HypnoScript (ausgelagert aus HypnoBuiltins)
    /// </summary>
    public static class FileBuiltins
    {
        /// <summary>Prüft, ob eine Datei existiert.</summary>
        public static bool FileExists(string path) => File.Exists(path);

        /// <summary>Liest den gesamten Inhalt einer Datei als String.</summary>
        public static string ReadFile(string path)
        {
            try { return File.ReadAllText(path); }
            catch (Exception ex) { return $"[File Error] {ex.Message}"; }
        }

        /// <summary>Schreibt einen String in eine Datei (überschreibt).</summary>
        public static void WriteFile(string path, string content)
        {
            try { File.WriteAllText(path, content); }
            catch (Exception ex) { HypnoBuiltins.Observe($"[File Error] {ex.Message}"); }
        }

        /// <summary>Hängt einen String an eine Datei an.</summary>
        public static void AppendFile(string path, string content)
        {
            try { File.AppendAllText(path, content); }
            catch (Exception ex) { HypnoBuiltins.Observe($"[File Error] {ex.Message}"); }
        }

        /// <summary>Liest alle Zeilen einer Datei als Array.</summary>
        public static string[] ReadLines(string path)
        {
            try { return File.ReadAllLines(path); }
            catch (Exception ex) { HypnoBuiltins.Observe($"[File Error] {ex.Message}"); return Array.Empty<string>(); }
        }

        /// <summary>Schreibt ein Array von Zeilen in eine Datei.</summary>
        public static void WriteLines(string path, string[] lines)
        {
            try { File.WriteAllLines(path, lines); }
            catch (Exception ex) { HypnoBuiltins.Observe($"[File Error] {ex.Message}"); }
        }

        /// <summary>Gibt die Dateigröße in Bytes zurück.</summary>
        public static long GetFileSize(string path)
        {
            try { return new FileInfo(path).Length; }
            catch (Exception ex) { HypnoBuiltins.Observe($"[File Error] {ex.Message}"); return -1; }
        }

        /// <summary>Gibt die Dateiendung zurück.</summary>
        public static string GetFileExtension(string path) => Path.GetExtension(path);

        /// <summary>Gibt den Dateinamen zurück.</summary>
        public static string GetFileName(string path) => Path.GetFileName(path);

        /// <summary>Gibt den Verzeichnisnamen zurück.</summary>
        public static string GetDirectoryName(string path) => Path.GetDirectoryName(path) ?? string.Empty;

        /// <summary>Prüft, ob ein Verzeichnis existiert.</summary>
        public static bool DirectoryExists(string path) => Directory.Exists(path);

        /// <summary>Erstellt ein Verzeichnis (rekursiv).</summary>
        public static void CreateDirectory(string path)
        {
            try { Directory.CreateDirectory(path); }
            catch (Exception ex) { HypnoBuiltins.Observe($"[Directory Error] {ex.Message}"); }
        }

        /// <summary>Gibt alle Dateien im Verzeichnis zurück (optional mit Suchmuster).</summary>
        public static string[] GetFiles(string path, string searchPattern = "*")
        {
            try { return Directory.GetFiles(path, searchPattern); }
            catch (Exception ex) { HypnoBuiltins.Observe($"[Directory Error] {ex.Message}"); return Array.Empty<string>(); }
        }

        /// <summary>Gibt alle Unterverzeichnisse im Verzeichnis zurück.</summary>
        public static string[] GetDirectories(string path)
        {
            try { return Directory.GetDirectories(path); }
            catch (Exception ex) { HypnoBuiltins.Observe($"[Directory Error] {ex.Message}"); return Array.Empty<string>(); }
        }
    }
}
