using System;
using System.IO;

class Program
{
    static int Main(string[] args)
    {
        Console.WriteLine("Minimale CLI Test");
        Console.WriteLine($"Args: {string.Join(" ", args)}");

        if (args.Length > 0)
        {
            var filePath = args[0];
            if (File.Exists(filePath))
            {
                var content = File.ReadAllText(filePath);
                Console.WriteLine($"Datei gelesen: '{content}'");
                return 0;
            }
            else
            {
                Console.WriteLine($"Datei nicht gefunden: {filePath}");
                return 1;
            }
        }

        Console.WriteLine("Keine Datei angegeben");
        return 1;
    }
}
