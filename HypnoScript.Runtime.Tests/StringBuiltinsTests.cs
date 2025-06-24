using Xunit;
using HypnoScript.Runtime.Builtins;

namespace HypnoScript.Runtime.Tests
{
    public class StringBuiltinsTests
    {
        [Fact]
        public void Length_Works()
        {
            Assert.Equal(4, StringBuiltins.Length("test"));
        }

        [Fact]
        public void Substring_Works()
        {
            Assert.Equal("es", StringBuiltins.Substring("test", 1, 2));
        }

        [Fact]
        public void ToUpper_ToLower_Works()
        {
            Assert.Equal("TEST", StringBuiltins.ToUpper("test"));
            Assert.Equal("test", StringBuiltins.ToLower("TEST"));
        }

        [Fact]
        public void Contains_Replace_Works()
        {
            Assert.True(StringBuiltins.Contains("abc", "b"));
            Assert.Equal("axc", StringBuiltins.Replace("abc", "b", "x"));
        }

        [Fact]
        public void Trim_TrimStart_TrimEnd_Works()
        {
            Assert.Equal("abc", StringBuiltins.Trim(" abc "));
            Assert.Equal("abc ", StringBuiltins.TrimStart(" abc "));
            Assert.Equal(" abc", StringBuiltins.TrimEnd(" abc "));
        }

        [Fact]
        public void IndexOf_LastIndexOf_Works()
        {
            Assert.Equal(1, StringBuiltins.IndexOf("abcab", "b"));
            Assert.Equal(4, StringBuiltins.LastIndexOf("abcab", "b"));
        }

        [Fact]
        public void Split_Join_Works()
        {
            var arr = StringBuiltins.Split("a,b,c", ",");
            Assert.Equal(new[] { "a", "b", "c" }, arr);
            Assert.Equal("a-b-c", StringBuiltins.Join(arr, "-"));
        }

        [Fact]
        public void StartsWith_EndsWith_Works()
        {
            Assert.True(StringBuiltins.StartsWith("abc", "a"));
            Assert.True(StringBuiltins.EndsWith("abc", "c"));
        }

        [Fact]
        public void PadLeft_PadRight_Works()
        {
            Assert.Equal("  ab", StringBuiltins.PadLeft("ab", 4));
            Assert.Equal("ab  ", StringBuiltins.PadRight("ab", 4));
        }
    }
}
