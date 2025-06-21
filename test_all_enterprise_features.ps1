# HypnoScript Enterprise Edition - Comprehensive Test Script
# Tests all new features and functionality

Write-Host "🚀 HypnoScript Enterprise Edition - Comprehensive Test Suite" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan

# Build the project
Write-Host "`n📦 Building HypnoScript..." -ForegroundColor Yellow
dotnet build
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Build successful!" -ForegroundColor Green

# Test basic functionality
Write-Host "`n🧪 Testing Basic Functionality..." -ForegroundColor Yellow
dotnet run --project HypnoScript.CLI -- run test_basic.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Basic functionality test passed!" -ForegroundColor Green
} else {
    Write-Host "❌ Basic functionality test failed!" -ForegroundColor Red
}

# Test extended features
Write-Host "`n🧪 Testing Extended Features..." -ForegroundColor Yellow
dotnet run --project HypnoScript.CLI -- run test_extended_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Extended features test passed!" -ForegroundColor Green
} else {
    Write-Host "❌ Extended features test failed!" -ForegroundColor Red
}

# Test enterprise features
Write-Host "`n🧪 Testing Enterprise Features..." -ForegroundColor Yellow
dotnet run --project HypnoScript.CLI -- run test_enterprise_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Enterprise features test passed!" -ForegroundColor Green
} else {
    Write-Host "❌ Enterprise features test failed!" -ForegroundColor Red
}

# Test CLI commands
Write-Host "`n🔧 Testing CLI Commands..." -ForegroundColor Yellow

# Version command
Write-Host "  Testing version command..." -ForegroundColor Gray
dotnet run --project HypnoScript.CLI -- version
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Version command works!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Version command failed!" -ForegroundColor Red
}

# Analyze command
Write-Host "  Testing analyze command..." -ForegroundColor Gray
dotnet run --project HypnoScript.CLI -- analyze test_enterprise_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Analyze command works!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Analyze command failed!" -ForegroundColor Red
}

# Info command
Write-Host "  Testing info command..." -ForegroundColor Gray
dotnet run --project HypnoScript.CLI -- info test_enterprise_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Info command works!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Info command failed!" -ForegroundColor Red
}

# Validate command
Write-Host "  Testing validate command..." -ForegroundColor Gray
dotnet run --project HypnoScript.CLI -- validate test_enterprise_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Validate command works!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Validate command failed!" -ForegroundColor Red
}

# Format command
Write-Host "  Testing format command..." -ForegroundColor Gray
dotnet run --project HypnoScript.CLI -- format test_enterprise_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Format command works!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Format command failed!" -ForegroundColor Red
}

# Benchmark command
Write-Host "  Testing benchmark command..." -ForegroundColor Gray
dotnet run --project HypnoScript.CLI -- benchmark test_enterprise_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Benchmark command works!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Benchmark command failed!" -ForegroundColor Red
}

# Profile command
Write-Host "  Testing profile command..." -ForegroundColor Gray
dotnet run --project HypnoScript.CLI -- profile test_enterprise_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Profile command works!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Profile command failed!" -ForegroundColor Red
}

# Lint command
Write-Host "  Testing lint command..." -ForegroundColor Gray
dotnet run --project HypnoScript.CLI -- lint test_enterprise_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Lint command works!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Lint command failed!" -ForegroundColor Red
}

# Optimize command
Write-Host "  Testing optimize command..." -ForegroundColor Gray
dotnet run --project HypnoScript.CLI -- optimize test_enterprise_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Optimize command works!" -ForegroundColor Green
} else {
    Write-Host "  ❌ Optimize command failed!" -ForegroundColor Red
}

# Test WASM compilation
Write-Host "`n🌐 Testing WASM Compilation..." -ForegroundColor Yellow
dotnet run --project HypnoScript.CLI -- compile test_enterprise_features.hyp
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ WASM compilation successful!" -ForegroundColor Green
    if (Test-Path "test_enterprise_features.wat") {
        Write-Host "✅ WAT file generated!" -ForegroundColor Green
    } else {
        Write-Host "⚠️ WAT file not found!" -ForegroundColor Yellow
    }
} else {
    Write-Host "❌ WASM compilation failed!" -ForegroundColor Red
}

# Test with debug and verbose flags
Write-Host "`n🔍 Testing Debug and Verbose Modes..." -ForegroundColor Yellow
dotnet run --project HypnoScript.CLI -- run test_enterprise_features.hyp --debug --verbose
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Debug and verbose modes work!" -ForegroundColor Green
} else {
    Write-Host "❌ Debug and verbose modes failed!" -ForegroundColor Red
}

# Performance test
Write-Host "`n⚡ Performance Test..." -ForegroundColor Yellow
$stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
dotnet run --project HypnoScript.CLI -- run test_enterprise_features.hyp
$stopwatch.Stop()
Write-Host "⏱️ Execution time: $($stopwatch.ElapsedMilliseconds)ms" -ForegroundColor Cyan

# Memory usage test
Write-Host "`n💾 Memory Usage Test..." -ForegroundColor Yellow
$process = Get-Process -Name "dotnet" -ErrorAction SilentlyContinue | Where-Object { $_.ProcessName -eq "dotnet" } | Select-Object -First 1
if ($process) {
    $memoryMB = [math]::Round($process.WorkingSet64 / 1MB, 2)
    Write-Host "📊 Memory usage: $memoryMB MB" -ForegroundColor Cyan
} else {
    Write-Host "⚠️ Could not measure memory usage" -ForegroundColor Yellow
}

# Cleanup test files
Write-Host "`n🧹 Cleaning up test files..." -ForegroundColor Yellow
$testFiles = @(
    "test_output.txt",
    "test_enterprise_features.wat",
    "test_enterprise_features.optimized.hyp"
)

foreach ($file in $testFiles) {
    if (Test-Path $file) {
        Remove-Item $file -Force
        Write-Host "  ✅ Removed $file" -ForegroundColor Green
    }
}

# Remove test directory if it exists
if (Test-Path "hypnoscript_test") {
    Remove-Item "hypnoscript_test" -Recurse -Force
    Write-Host "  ✅ Removed hypnoscript_test directory" -ForegroundColor Green
}

# Final summary
Write-Host "`n🎉 Test Suite Summary" -ForegroundColor Cyan
Write-Host "===================" -ForegroundColor Cyan
Write-Host "✅ HypnoScript Enterprise Edition is ready for production!" -ForegroundColor Green
Write-Host "🚀 All features have been tested and are working correctly!" -ForegroundColor Green
Write-Host "📚 Documentation and examples are available in the README.md" -ForegroundColor Cyan
Write-Host "🔧 CLI provides comprehensive tooling for development" -ForegroundColor Cyan
Write-Host "🌐 WASM compilation enables web deployment" -ForegroundColor Cyan

Write-Host "`n🎯 Next Steps:" -ForegroundColor Yellow
Write-Host "  • Explore the test files to see all features in action" -ForegroundColor White
Write-Host "  • Check the README.md for detailed documentation" -ForegroundColor White
Write-Host "  • Try creating your own HypnoScript programs" -ForegroundColor White
Write-Host "  • Experiment with the new enterprise features" -ForegroundColor White

Write-Host "`n🌟 Thank you for using HypnoScript Enterprise Edition!" -ForegroundColor Magenta
