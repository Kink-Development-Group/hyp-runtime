using Xunit;
using HypnoScript.Runtime.Builtins;

namespace HypnoScript.Runtime.Tests
{
    public class NetworkBuiltinsTests
    {
        [Fact]
        public void IsValidEmail_Works()
        {
            Assert.True(NetworkBuiltins.IsValidEmail("test@example.com"));
            Assert.False(NetworkBuiltins.IsValidEmail("invalid-email"));
        }

        [Fact]
        public void IsValidUrl_Works()
        {
            Assert.True(NetworkBuiltins.IsValidUrl("https://example.com"));
            Assert.False(NetworkBuiltins.IsValidUrl("not a url"));
        }

        [Fact]
        public void IsValidIPAddress_Works()
        {
            Assert.True(NetworkBuiltins.IsValidIPAddress("127.0.0.1"));
            Assert.False(NetworkBuiltins.IsValidIPAddress("notanip"));
        }

        [Fact]
        public void IsValidPort_Works()
        {
            Assert.True(NetworkBuiltins.IsValidPort(80));
            Assert.False(NetworkBuiltins.IsValidPort(70000));
        }

        [Fact]
        public void UrlEncodeDecode_Works()
        {
            var encoded = NetworkBuiltins.UrlEncode("a b");
            Assert.Equal("a+b", encoded);
            Assert.Equal("a b", NetworkBuiltins.UrlDecode(encoded));
        }

        [Fact]
        public void HtmlEncodeDecode_Works()
        {
            var encoded = NetworkBuiltins.HtmlEncode("<b>");
            Assert.Equal("&lt;b&gt;", encoded);
            Assert.Equal("<b>", NetworkBuiltins.HtmlDecode(encoded));
        }

        [Fact]
        public void ExtractDomain_And_Path_Works()
        {
            Assert.Equal("example.com", NetworkBuiltins.ExtractDomain("https://example.com/test"));
            Assert.Equal("/test", NetworkBuiltins.ExtractPath("https://example.com/test"));
        }

        [Fact]
        public void IsLocalhost_Works()
        {
            Assert.True(NetworkBuiltins.IsLocalhost("http://localhost:8080"));
            Assert.False(NetworkBuiltins.IsLocalhost("https://example.com"));
        }
    }
}
