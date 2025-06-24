using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// Stellt Datei- und Verzeichnisfunktionen für HypnoScript bereit.
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

        /// <summary>
        /// Copies a file
        /// </summary>
        public static void FileCopy(string source, string dest)
        {
            try
            {
                File.Copy(source, dest, true);
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to copy file: {ex.Message}");
            }
        }

        /// <summary>
        /// Moves a file
        /// </summary>
        public static void FileMove(string source, string dest)
        {
            try
            {
                File.Move(source, dest);
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to move file: {ex.Message}");
            }
        }

        /// <summary>
        /// Deletes a file
        /// </summary>
        public static void FileDelete(string path)
        {
            try
            {
                if (File.Exists(path))
                {
                    File.Delete(path);
                }
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to delete file: {ex.Message}");
            }
        }

        /// <summary>
        /// Gets file information
        /// </summary>
        public static Dictionary<string, object> GetFileInfo(string path)
        {
            try
            {
                var fileInfo = new FileInfo(path);
                return new Dictionary<string, object>
                {
                    ["name"] = fileInfo.Name,
                    ["fullName"] = fileInfo.FullName,
                    ["size"] = fileInfo.Length,
                    ["creationTime"] = fileInfo.CreationTime.ToString("yyyy-MM-dd HH:mm:ss"),
                    ["lastWriteTime"] = fileInfo.LastWriteTime.ToString("yyyy-MM-dd HH:mm:ss"),
                    ["extension"] = fileInfo.Extension,
                    ["exists"] = fileInfo.Exists
                };
            }
            catch
            {
                return new Dictionary<string, object>
                {
                    ["exists"] = false
                };
            }
        }

        /// <summary>
        /// Checks if file is read-only
        /// </summary>
        public static bool IsFileReadOnly(string path) => (File.GetAttributes(path) & FileAttributes.ReadOnly) != 0;

        /// <summary>
        /// Sets file read-only attribute
        /// </summary>
        public static void SetFileReadOnly(string path, bool readOnly)
        {
            try
            {
                var attributes = File.GetAttributes(path);
                if (readOnly)
                    attributes |= FileAttributes.ReadOnly;
                else
                    attributes &= ~FileAttributes.ReadOnly;
                File.SetAttributes(path, attributes);
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to set file attributes: {ex.Message}");
            }
        }

        /// <summary>
        /// Gets file creation time
        /// </summary>
        public static string GetFileCreationTime(string path) => File.GetCreationTime(path).ToString("yyyy-MM-dd HH:mm:ss");

        /// <summary>
        /// Gets file last write time
        /// </summary>
        public static string GetFileLastWriteTime(string path) => File.GetLastWriteTime(path).ToString("yyyy-MM-dd HH:mm:ss");

        /// <summary>
        /// Gets file size in MB
        /// </summary>
        public static double GetFileSizeMB(string path) => new FileInfo(path).Length / (1024.0 * 1024.0);

        /// <summary>
        /// Gets file name without extension
        /// </summary>
        public static string GetFileNameWithoutExtension(string path) => Path.GetFileNameWithoutExtension(path);

        /// <summary>
        /// Combines path components
        /// </summary>
        public static string CombinePath(string path1, string path2) => Path.Combine(path1, path2);

        /// <summary>
        /// Gets current directory
        /// </summary>
        public static string GetCurrentDirectory() => Environment.CurrentDirectory;

        /// <summary>
        /// Gets temporary path
        /// </summary>
        public static string GetTempPath() => Path.GetTempPath();
    }
}
