using Xunit;
using HypnoScript.Runtime.Builtins;

namespace HypnoScript.Runtime.Tests
{
    public class MathBuiltinsTests
    {
        [Fact]
        public void Abs_Works()
        {
            Assert.Equal(5, MathBuiltins.Abs(-5));
            Assert.Equal(5, MathBuiltins.Abs(5));
        }

        [Fact]
        public void Sin_Cos_Tan_Works()
        {
            Assert.Equal(0, MathBuiltins.Sin(0), 5);
            Assert.Equal(1, MathBuiltins.Sin(90), 5);
            Assert.Equal(0, MathBuiltins.Cos(90), 5);
            Assert.Equal(1, MathBuiltins.Cos(0), 5);
            Assert.Equal(0, MathBuiltins.Tan(0), 5);
        }

        [Fact]
        public void Sqrt_Works()
        {
            Assert.Equal(3, MathBuiltins.Sqrt(9), 5);
        }

        [Fact]
        public void Pow_Works()
        {
            Assert.Equal(8, MathBuiltins.Pow(2, 3), 5);
        }

        [Fact]
        public void Floor_Ceiling_Round_Works()
        {
            Assert.Equal(1, MathBuiltins.Floor(1.9));
            Assert.Equal(2, MathBuiltins.Ceiling(1.1));
            Assert.Equal(2, MathBuiltins.Round(1.5));
        }

        [Fact]
        public void Log_Log10_Exp_Works()
        {
            Assert.Equal(1, MathBuiltins.Log(Math.E), 5);
            Assert.Equal(2, MathBuiltins.Log10(100), 5);
            Assert.Equal(Math.E, MathBuiltins.Exp(1), 5);
        }

        [Fact]
        public void Max_Min_Works()
        {
            Assert.Equal(5, MathBuiltins.Max(5, 3));
            Assert.Equal(3, MathBuiltins.Min(5, 3));
        }

        [Fact]
        public void Random_ReturnsValueInRange()
        {
            var value = MathBuiltins.Random();
            Assert.InRange(value, 0, 1);
        }

        [Fact]
        public void RandomInt_ReturnsValueInRange()
        {
            var value = MathBuiltins.RandomInt(1, 10);
            Assert.InRange(value, 1, 10);
        }
    }
}
