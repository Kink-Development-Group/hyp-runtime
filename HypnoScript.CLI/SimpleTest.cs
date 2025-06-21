using System;
using System.IO;
using System.Threading.Tasks;

namespace HypnoScript.CLI
{
    public class SimpleTest
    {
        public static async Task<int> Main(string[] args)
        {
            Console.WriteLine("SimpleTest gestartet!");
            Console.WriteLine($"Args: {string.Join(" ", args)}");

            if (args.Length > 0)
            {
                var filePath = args[0];
                if (File.Exists(filePath))
                {
                    var content = await File.ReadAllTextAsync(filePath);
                    Console.WriteLine($"Datei gelesen: {content}");
                }
                else
                {
                    Console.WriteLine($"Datei nicht gefunden: {filePath}");
                }
            }

            Console.WriteLine("SimpleTest beendet!");
            return 0;
        }
    }
}
