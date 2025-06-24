using System;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// Mathematische Builtins für HypnoScript (ausgelagert aus HypnoBuiltins)
    /// </summary>
    public static class MathBuiltins
    {
        /// <summary>Gibt den Absolutwert einer Zahl zurück.</summary>
        public static double Abs(double x) => Math.Abs(x);
        /// <summary>Sinus (im Gradmaß, nicht Radiant).</summary>
        public static double Sin(double x) => Math.Sin(x * Math.PI / 180.0); // Grad zu Radiant
        /// <summary>Kosinus (im Gradmaß, nicht Radiant).</summary>
        public static double Cos(double x) => Math.Cos(x * Math.PI / 180.0);
        /// <summary>Tangens (im Gradmaß, nicht Radiant).</summary>
        public static double Tan(double x) => Math.Tan(x * Math.PI / 180.0);
        /// <summary>Quadratwurzel.</summary>
        public static double Sqrt(double x) => Math.Sqrt(x);
        /// <summary>Potenzfunktion (x^y).</summary>
        public static double Pow(double x, double y) => Math.Pow(x, y);
        /// <summary>Rundet ab.</summary>
        public static double Floor(double x) => Math.Floor(x);
        /// <summary>Rundet auf.</summary>
        public static double Ceiling(double x) => Math.Ceiling(x);
        /// <summary>Rundet auf die nächste Ganzzahl.</summary>
        public static double Round(double x) => Math.Round(x);
        /// <summary>Natürlicher Logarithmus.</summary>
        public static double Log(double x) => Math.Log(x);
        /// <summary>Zehner-Logarithmus.</summary>
        public static double Log10(double x) => Math.Log10(x);
        /// <summary>Exponentialfunktion (e^x).</summary>
        public static double Exp(double x) => Math.Exp(x);
        /// <summary>Maximum zweier Zahlen.</summary>
        public static double Max(double x, double y) => Math.Max(x, y);
        /// <summary>Minimum zweier Zahlen.</summary>
        public static double Min(double x, double y) => Math.Min(x, y);
        /// <summary>Zufallszahl zwischen 0 und 1 (nicht kryptografisch).</summary>
        public static double Random() => HypnoBuiltins._random.NextDouble();
        /// <summary>Zufallszahl im Bereich [min, max] (nicht kryptografisch).</summary>
        public static int RandomInt(int min, int max) => HypnoBuiltins._random.Next(min, max + 1);
    }
}
