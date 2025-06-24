using Xunit;
using HypnoScript.Runtime.Builtins;

namespace HypnoScript.Runtime.Tests
{
    public class ArrayBuiltinsTests
    {
        [Fact]
        public void ArrayLength_Works()
        {
            var arr = new object[] { 1, 2, 3 };
            Assert.Equal(3, ArrayBuiltins.ArrayLength(arr));
        }

        [Fact]
        public void ArrayGet_Works_And_Errors()
        {
            var arr = new object[] { "a", "b" };
            Assert.Equal("a", ArrayBuiltins.ArrayGet(arr, 0));
            Assert.Null(ArrayBuiltins.ArrayGet(arr, 2)); // out of bounds
            Assert.Null(ArrayBuiltins.ArrayGet(null, 0)); // null
        }

        [Fact]
        public void ArraySet_Works_And_Errors()
        {
            var arr = new object[] { 1, 2 };
            ArrayBuiltins.ArraySet(arr, 1, 99);
            Assert.Equal(99, arr[1]);
            ArrayBuiltins.ArraySet(arr, 2, 5); // out of bounds, should not throw
            ArrayBuiltins.ArraySet(null, 0, 5); // null, should not throw
        }

        [Fact]
        public void ArraySlice_Works_And_Errors()
        {
            var arr = new object[] { 1, 2, 3, 4 };
            var slice = ArrayBuiltins.ArraySlice(arr, 1, 2);
            Assert.Equal(new object[] { 2, 3 }, slice);
            Assert.Empty(ArrayBuiltins.ArraySlice(arr, 3, 5)); // out of bounds
            Assert.Empty(ArrayBuiltins.ArraySlice(null, 0, 1)); // null
        }

        [Fact]
        public void ArrayConcat_Works()
        {
            var arr1 = new object[] { 1, 2 };
            var arr2 = new object[] { 3, 4 };
            var result = ArrayBuiltins.ArrayConcat(arr1, arr2);
            Assert.Equal(new object[] { 1, 2, 3, 4 }, result);
        }

        [Fact]
        public void ArrayIndexOf_And_Contains_Works()
        {
            var arr = new object[] { "x", "y", "z" };
            Assert.Equal(1, ArrayBuiltins.ArrayIndexOf(arr, "y"));
            Assert.True(ArrayBuiltins.ArrayContains(arr, "z"));
            Assert.False(ArrayBuiltins.ArrayContains(arr, "a"));
        }
    }
}
