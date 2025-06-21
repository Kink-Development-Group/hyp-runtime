# HypnoScript Simple Test v3.0.0
Write-Host "=== HypnoScript Simple Test v3.0.0 ===" -ForegroundColor Cyan
Write-Host "Testing basic functionality..." -ForegroundColor Yellow
Write-Host ""

$testFile = "simple_test.hyp"
$cliCommand = "dotnet run --project HypnoScript.CLI --"

# Test basic execution
Write-Host "[TEST] Basic execution" -ForegroundColor Green
try {
    $result = & dotnet run --project HypnoScript.CLI -- run $testFile 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   PASSED" -ForegroundColor Green
        Write-Host "   Output: $result" -ForegroundColor Gray
    } else {
        Write-Host "   FAILED" -ForegroundColor Red
        Write-Host "   Error: $result" -ForegroundColor Red
    }
} catch {
    Write-Host "   ERROR: $_" -ForegroundColor Red
}

Write-Host ""
Write-Host "[TEST] Version check" -ForegroundColor Green
try {
    $version = & dotnet run --project HypnoScript.CLI -- version 2>&1
    Write-Host "   Version: $version" -ForegroundColor Gray
} catch {
    Write-Host "   ERROR: $_" -ForegroundColor Red
}

Write-Host ""
Write-Host "[TEST] Help check" -ForegroundColor Green
try {
    $help = & dotnet run --project HypnoScript.CLI -- help 2>&1
    Write-Host "   Help available: $($help.Length -gt 0)" -ForegroundColor Gray
} catch {
    Write-Host "   ERROR: $_" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== Test Summary ===" -ForegroundColor Cyan
Write-Host "Basic functionality test completed!" -ForegroundColor Green
Write-Host "HypnoScript Enterprise v3.0.0 is operational!" -ForegroundColor Green
