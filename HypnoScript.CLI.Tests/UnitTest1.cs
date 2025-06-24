using System;
using System.IO;
using Xunit;
using HypnoScript.CLI.Commands;

public class LintCommandIntegrationTests
{
    [Fact]
    public void LintCommand_ValidScript_ReturnsZero()
    {
        // Arrange: Nutze ein valides Skript aus TestData
        var scriptPath = Path.Combine("TestData", "valid.hyp");
        // Act
        int exitCode = LintCommand.Execute(scriptPath, debug: false, verbose: false);
        // Assert
        Assert.Equal(0, exitCode);
    }

    [Fact]
    public void LintCommand_InvalidScript_ReturnsError()
    {
        // Arrange: Nutze ein fehlerhaftes Skript aus TestData
        var scriptPath = Path.Combine("TestData", "invalid.hyp");
        // Act
        int exitCode = LintCommand.Execute(scriptPath, debug: false, verbose: false);
        // Assert
        Assert.Equal(1, exitCode);
    }

    [Fact]
    public void BenchmarkCommand_ValidScript_ReturnsZero()
    {
        var scriptPath = Path.Combine("TestData", "valid.hyp");
        int exitCode = HypnoScript.CLI.Commands.BenchmarkCommand.Execute(scriptPath, debug: false, verbose: false);
        Assert.Equal(0, exitCode);
    }

    [Fact]
    public void ProfileCommand_ValidScript_ReturnsZero()
    {
        var scriptPath = Path.Combine("TestData", "valid.hyp");
        int exitCode = HypnoScript.CLI.Commands.ProfileCommand.Execute(scriptPath, debug: false, verbose: false);
        Assert.Equal(0, exitCode);
    }

    [Fact]
    public void OptimizeCommand_ValidScript_ReturnsZero()
    {
        var scriptPath = Path.Combine("TestData", "valid.hyp");
        int exitCode = HypnoScript.CLI.Commands.OptimizeCommand.Execute(scriptPath, debug: false, verbose: false);
        Assert.Equal(0, exitCode);
    }
}
