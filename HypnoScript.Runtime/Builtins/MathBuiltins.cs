using System;
using System.Linq;
using System.Numerics;

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

        /// <summary>
        /// Calculates the factorial of a number
        /// </summary>
        public static double Factorial(int n)
        {
            if (n < 0) throw new ArgumentException("Factorial is not defined for negative numbers");
            if (n == 0 || n == 1) return 1;

            double result = 1;
            for (int i = 2; i <= n; i++)
            {
                result *= i;
            }
            return result;
        }

        /// <summary>
        /// Calculates the greatest common divisor of two numbers
        /// </summary>
        public static double GCD(double a, double b)
        {
            a = Math.Abs(a);
            b = Math.Abs(b);

            while (b != 0)
            {
                double temp = b;
                b = a % b;
                a = temp;
            }
            return a;
        }

        /// <summary>
        /// Calculates the least common multiple of two numbers
        /// </summary>
        public static double LCM(double a, double b)
        {
            return Math.Abs(a * b) / GCD(a, b);
        }

        /// <summary>
        /// Converts degrees to radians
        /// </summary>
        public static double DegreesToRadians(double degrees) => degrees * Math.PI / 180.0;

        /// <summary>
        /// Converts radians to degrees
        /// </summary>
        public static double RadiansToDegrees(double radians) => radians * 180.0 / Math.PI;

        /// <summary>
        /// Arc sine in degrees
        /// </summary>
        public static double Asin(double x) => Math.Asin(x) * 180.0 / Math.PI;

        /// <summary>
        /// Arc cosine in degrees
        /// </summary>
        public static double Acos(double x) => Math.Acos(x) * 180.0 / Math.PI;

        /// <summary>
        /// Arc tangent in degrees
        /// </summary>
        public static double Atan(double x) => Math.Atan(x) * 180.0 / Math.PI;

        /// <summary>
        /// Arc tangent of y/x in degrees
        /// </summary>
        public static double Atan2(double y, double x) => Math.Atan2(y, x) * 180.0 / Math.PI;

        /// <summary>
        /// Clamps a value between min and max
        /// </summary>
        public static double Clamp(double value, double min, double max) => Math.Max(min, Math.Min(max, value));

        /// <summary>
        /// Returns the sign of a number (-1, 0, or 1)
        /// </summary>
        public static int Sign(double value) => Math.Sign(value);

        /// <summary>
        /// Checks if a number is even
        /// </summary>
        public static bool IsEven(int value) => value % 2 == 0;

        /// <summary>
        /// Checks if a number is odd
        /// </summary>
        public static bool IsOdd(int value) => value % 2 != 0;

        /// <summary>
        /// Checks if a number is prime
        /// </summary>
        public static bool IsPrime(int n)
        {
            if (n < 2) return false;
            if (n == 2) return true;
            if (n % 2 == 0) return false;

            for (int i = 3; i <= Math.Sqrt(n); i += 2)
            {
                if (n % i == 0) return false;
            }
            return true;
        }

        /// <summary>
        /// Calculates factorial for large numbers using BigInteger
        /// </summary>
        public static BigInteger FactorialBig(int n)
        {
            if (n < 0) throw new ArgumentException("Factorial is not defined for negative numbers");
            if (n == 0 || n == 1) return 1;

            BigInteger result = 1;
            for (int i = 2; i <= n; i++)
            {
                result *= i;
            }
            return result;
        }

        /// <summary>
        /// Converts a number to hexadecimal string
        /// </summary>
        public static string ToHex(long n) => n.ToString("X");

        /// <summary>
        /// Converts a number to binary string
        /// </summary>
        public static string ToBinary(long n) => Convert.ToString(n, 2);

        /// <summary>
        /// Rounds a number to specified decimal places
        /// </summary>
        public static double RoundToDecimal(double x, int decimals) => Math.Round(x, decimals);

        /// <summary>
        /// Ceilings a number to specified decimal places
        /// </summary>
        public static double CeilingToDecimal(double x, int decimals) => Math.Ceiling(x * Math.Pow(10, decimals)) / Math.Pow(10, decimals);

        /// <summary>
        /// Floors a number to specified decimal places
        /// </summary>
        public static double FloorToDecimal(double x, int decimals) => Math.Floor(x * Math.Pow(10, decimals)) / Math.Pow(10, decimals);

        /// <summary>
        /// Calculates modulo operation
        /// </summary>
        public static double Modulo(double a, double b) => a % b;

        /// <summary>
        /// Checks if a number is a power of 2
        /// </summary>
        public static bool PowerOf2(int n) => n > 0 && (n & (n - 1)) == 0;

        /// <summary>
        /// Finds the next power of 2 greater than or equal to n
        /// </summary>
        public static int NextPowerOf2(int n)
        {
            if (n <= 0) return 1;
            n--;
            n |= n >> 1;
            n |= n >> 2;
            n |= n >> 4;
            n |= n >> 8;
            n |= n >> 16;
            return n + 1;
        }

        /// <summary>
        /// Checks if a number is a perfect square
        /// </summary>
        public static bool IsPerfectSquare(int n)
        {
            if (n < 0) return false;
            int root = (int)Math.Sqrt(n);
            return root * root == n;
        }

        /// <summary>
        /// Integer square root
        /// </summary>
        public static int SqrtInt(int n) => (int)Math.Sqrt(n);

        /// <summary>
        /// Calculates GCD of an array of numbers
        /// </summary>
        public static int GCDArray(object[] arr)
        {
            if (arr.Length == 0) return 0;
            if (arr.Length == 1) return Convert.ToInt32(arr[0]);

            int result = Convert.ToInt32(arr[0]);
            for (int i = 1; i < arr.Length; i++)
            {
                result = (int)GCD(result, Convert.ToDouble(arr[i]));
            }
            return result;
        }

        /// <summary>
        /// Calculates LCM of an array of numbers
        /// </summary>
        public static int LCMArray(object[] arr)
        {
            if (arr.Length == 0) return 0;
            if (arr.Length == 1) return Convert.ToInt32(arr[0]);

            int result = Convert.ToInt32(arr[0]);
            for (int i = 1; i < arr.Length; i++)
            {
                result = (int)LCM(result, Convert.ToDouble(arr[i]));
            }
            return result;
        }

        /// <summary>
        /// Calculates sum of digits in a number
        /// </summary>
        public static int SumOfDigits(long n) => n.ToString().Sum(c => c - '0');

        /// <summary>
        /// Reverses the digits of a number
        /// </summary>
        public static long ReverseNumber(long n) => long.Parse(new string(n.ToString().Reverse().ToArray()));
    }
}
