using Xunit;
using HypnoScript.Runtime.Builtins;

namespace HypnoScript.Runtime.Tests
{
    public class SystemBuiltinsTests
    {
        [Fact]
        public void GetEnvironmentVariable_Works()
        {
            var path = SystemBuiltins.GetEnvironmentVariable("PATH");
            Assert.False(string.IsNullOrEmpty(path));
        }

        [Fact]
        public void GetCurrentDirectory_Works()
        {
            var dir = SystemBuiltins.GetCurrentDirectory();
            Assert.False(string.IsNullOrEmpty(dir));
        }

        [Fact]
        public void GetMachineName_Works()
        {
            var name = SystemBuiltins.GetMachineName();
            Assert.False(string.IsNullOrEmpty(name));
        }

        [Fact]
        public void GetUserName_Works()
        {
            var user = SystemBuiltins.GetUserName();
            Assert.False(string.IsNullOrEmpty(user));
        }

        [Fact]
        public void GetOSVersion_Works()
        {
            var os = SystemBuiltins.GetOSVersion();
            Assert.False(string.IsNullOrEmpty(os));
        }

        [Fact]
        public void GetProcessorCount_Works()
        {
            Assert.True(SystemBuiltins.GetProcessorCount() > 0);
        }

        [Fact]
        public void GetWorkingSet_Works()
        {
            Assert.True(SystemBuiltins.GetWorkingSet() > 0);
        }
    }
}
