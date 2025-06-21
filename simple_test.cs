using System;
using System.IO;

class Program
{
    static int Main(string[] args)
    {
        Console.WriteLine("=== Simple HypnoScript Test ===");

        if (args.Length == 0)
        {
            Console.WriteLine("Verwendung: dotnet run simple_test.cs <datei.hyp>");
            return 1;
        }

        var filePath = args[0];

        try
        {
            if (!File.Exists(filePath))
            {
                Console.WriteLine($"Datei nicht gefunden: {filePath}");
                return 1;
            }

            var content = File.ReadAllText(filePath);
            Console.WriteLine($"Datei gelesen: {content.Length} Zeichen");
            Console.WriteLine($"Inhalt: {content}");

            if (content.TrimStart().StartsWith("Focus"))
            {
                Console.WriteLine("✓ Datei beginnt mit 'Focus'");
            }
            else
            {
                Console.WriteLine("⚠ Datei beginnt nicht mit 'Focus'");
            }

            return 0;
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Fehler: {ex.Message}");
            return 1;
        }
    }
}
