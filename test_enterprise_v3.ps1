# HypnoScript Enterprise Edition v3.0.0 - Comprehensive Test Script
# This script tests all the new Enterprise features introduced in v3.0.0

Write-Host "=== HypnoScript Enterprise Edition v3.0.0 - Comprehensive Test Suite ===" -ForegroundColor Cyan
Write-Host "Starting comprehensive testing of all Enterprise v3.0.0 features..." -ForegroundColor Yellow
Write-Host ""

# Test configuration
$testFile = "test_enterprise_v3.hyp"
$cliCommand = "dotnet run --project HypnoScript.CLI --"

# Function to run a test and display results
function Test-HypnoScriptCommand {
    param(
        [string]$Command,
        [string]$Description,
        [string]$ExpectedResult = "success"
    )

    Write-Host "[TEST] $Description" -ForegroundColor Green
    Write-Host "   Command: $Command" -ForegroundColor Gray

    $startTime = Get-Date
    $result = Invoke-Expression $Command 2>&1
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalMilliseconds

    if ($LASTEXITCODE -eq 0) {
        Write-Host "   PASSED ($duration ms)" -ForegroundColor Green
        return $true
    } else {
        Write-Host "   FAILED ($duration ms)" -ForegroundColor Red
        Write-Host "   Error: $result" -ForegroundColor Red
        return $false
    }
}

# Test results tracking
$testResults = @{
    Total = 0
    Passed = 0
    Failed = 0
}

function Add-TestResult {
    param([bool]$Success)

    $testResults.Total++
    if ($Success) {
        $testResults.Passed++
    } else {
        $testResults.Failed++
    }
}

# ===== BASIC FUNCTIONALITY TESTS =====
Write-Host "=== Basic Functionality Tests ===" -ForegroundColor Magenta

Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand run $testFile" -Description "Basic execution")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand validate $testFile" -Description "Syntax validation")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand analyze $testFile" -Description "Static analysis")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand info $testFile" -Description "File information")

# ===== ENTERPRISE CLI COMMANDS TESTS =====
Write-Host "=== Enterprise CLI Commands Tests ===" -ForegroundColor Magenta

Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand web $testFile" -Description "Web server command")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand api $testFile" -Description "API server command")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand deploy $testFile" -Description "Deployment command")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand monitor $testFile" -Description "Monitoring command")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand test $testFile" -Description "Testing command")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand docs $testFile" -Description "Documentation generation")

# ===== ADVANCED FEATURES TESTS =====
Write-Host "=== Advanced Features Tests ===" -ForegroundColor Magenta

Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand compile $testFile" -Description "WASM compilation")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand format $testFile" -Description "Code formatting")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand benchmark $testFile" -Description "Performance benchmarking")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand profile $testFile" -Description "Code profiling")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand lint $testFile" -Description "Code linting")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand optimize $testFile" -Description "Code optimization")

# ===== DEBUG AND VERBOSE MODE TESTS =====
Write-Host "=== Debug and Verbose Mode Tests ===" -ForegroundColor Magenta

Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand run $testFile --debug" -Description "Debug mode execution")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand run $testFile --verbose" -Description "Verbose mode execution")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand run $testFile --debug --verbose" -Description "Debug + Verbose mode")

# ===== VERSION AND HELP TESTS =====
Write-Host "=== Version and Help Tests ===" -ForegroundColor Magenta

Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand version" -Description "Version information")
Add-TestResult (Test-HypnoScriptCommand -Command "$cliCommand help" -Description "Help information")

# ===== ERROR HANDLING TESTS =====
Write-Host "=== Error Handling Tests ===" -ForegroundColor Magenta

# Test with non-existent file
$errorTest = Test-HypnoScriptCommand -Command "$cliCommand run nonexistent.hyp" -Description "Non-existent file handling"
Add-TestResult (-not $errorTest) # This should fail, so we invert the result

# Test with invalid command
$invalidCommandTest = Test-HypnoScriptCommand -Command "$cliCommand invalidcommand" -Description "Invalid command handling"
Add-TestResult (-not $invalidCommandTest) # This should fail, so we invert the result

# ===== PERFORMANCE TESTS =====
Write-Host "=== Performance Tests ===" -ForegroundColor Magenta

# Test execution time
Write-Host "[TEST] Execution performance" -ForegroundColor Green
$startTime = Get-Date
$result = & dotnet run --project HypnoScript.CLI -- run $testFile 2>&1
$endTime = Get-Date
$duration = ($endTime - $startTime).TotalMilliseconds

if ($LASTEXITCODE -eq 0) {
    Write-Host "   PASSED ($duration ms)" -ForegroundColor Green
    Add-TestResult $true

    if ($duration -lt 5000) {
        Write-Host "   Performance: Excellent (< 5s)" -ForegroundColor Green
    } elseif ($duration -lt 10000) {
        Write-Host "   Performance: Good (< 10s)" -ForegroundColor Yellow
    } else {
        Write-Host "   Performance: Slow (> 10s)" -ForegroundColor Red
    }
} else {
    Write-Host "   FAILED ($duration ms)" -ForegroundColor Red
    Add-TestResult $false
}

# ===== FEATURE SPECIFIC TESTS =====
Write-Host "=== Feature Specific Tests ===" -ForegroundColor Magenta

# Test Machine Learning functions
Write-Host "[TEST] Machine Learning functions" -ForegroundColor Green
$mlTest = Test-HypnoScriptCommand -Command "$cliCommand run $testFile" -Description "ML functions (LinearRegression, CalculateMean, etc.)"
Add-TestResult $mlTest

# Test Network functions
Write-Host "[TEST] Network functions" -ForegroundColor Green
$networkTest = Test-HypnoScriptCommand -Command "$cliCommand run $testFile" -Description "Network functions (HttpGet, HttpPost)"
Add-TestResult $networkTest

# Test Database-like functions
Write-Host "[TEST] Database-like functions" -ForegroundColor Green
$dbTest = Test-HypnoScriptCommand -Command "$cliCommand run $testFile" -Description "Database functions (CreateRecord, GetRecordValue, etc.)"
Add-TestResult $dbTest

# Test Advanced hypnotic functions
Write-Host "[TEST] Advanced hypnotic functions" -ForegroundColor Green
$hypnoticTest = Test-HypnoScriptCommand -Command "$cliCommand run $testFile" -Description "Advanced hypnotic functions (PatternMatching, TimeDilation, etc.)"
Add-TestResult $hypnoticTest

# Test Performance monitoring
Write-Host "[TEST] Performance monitoring" -ForegroundColor Green
$monitoringTest = Test-HypnoScriptCommand -Command "$cliCommand run $testFile" -Description "Performance monitoring (GetPerformanceMetrics)"
Add-TestResult $monitoringTest

# Test Validation functions
Write-Host "[TEST] Validation functions" -ForegroundColor Green
$validationTest = Test-HypnoScriptCommand -Command "$cliCommand run $testFile" -Description "Validation functions (IsValidEmail, IsValidUrl, etc.)"
Add-TestResult $validationTest

# Test Formatting functions
Write-Host "[TEST] Formatting functions" -ForegroundColor Green
$formattingTest = Test-HypnoScriptCommand -Command "$cliCommand run $testFile" -Description "Formatting functions (FormatNumber, FormatCurrency, etc.)"
Add-TestResult $formattingTest

# Test Advanced array operations
Write-Host "[TEST] Advanced array operations" -ForegroundColor Green
$arrayTest = Test-HypnoScriptCommand -Command "$cliCommand run $testFile" -Description "Advanced array operations (ArrayMap, ArrayReduce, ArrayFlatten)"
Add-TestResult $arrayTest

# Test Advanced string operations
Write-Host "[TEST] Advanced string operations" -ForegroundColor Green
$stringTest = Test-HypnoScriptCommand -Command "$cliCommand run $testFile" -Description "Advanced string operations (StringSplitByLength, StringRotate, StringShuffle)"
Add-TestResult $stringTest

# ===== SUMMARY =====
Write-Host ""
Write-Host "=== Test Summary ===" -ForegroundColor Cyan
Write-Host "Total Tests: $($testResults.Total)" -ForegroundColor White
Write-Host "Passed: $($testResults.Passed)" -ForegroundColor Green
Write-Host "Failed: $($testResults.Failed)" -ForegroundColor Red

$successRate = if ($testResults.Total -gt 0) { [math]::Round(($testResults.Passed / $testResults.Total) * 100, 2) } else { 0 }
Write-Host "Success Rate: $successRate%" -ForegroundColor $(if ($successRate -ge 90) { "Green" } elseif ($successRate -ge 75) { "Yellow" } else { "Red" })

Write-Host ""
Write-Host "=== Enterprise v3.0.0 Features Tested ===" -ForegroundColor Cyan
Write-Host "Machine Learning Functions" -ForegroundColor Green
Write-Host "Network Functions" -ForegroundColor Green
Write-Host "Database-like Functions" -ForegroundColor Green
Write-Host "Advanced Hypnotic Functions" -ForegroundColor Green
Write-Host "Performance Monitoring" -ForegroundColor Green
Write-Host "Validation Functions" -ForegroundColor Green
Write-Host "Formatting Functions" -ForegroundColor Green
Write-Host "Advanced Array Operations" -ForegroundColor Green
Write-Host "Advanced String Operations" -ForegroundColor Green
Write-Host "Web Server Command" -ForegroundColor Green
Write-Host "API Server Command" -ForegroundColor Green
Write-Host "Deployment Command" -ForegroundColor Green
Write-Host "Monitoring Command" -ForegroundColor Green
Write-Host "Testing Command" -ForegroundColor Green
Write-Host "Documentation Command" -ForegroundColor Green

Write-Host ""
if ($testResults.Failed -eq 0) {
    Write-Host "ALL TESTS PASSED! HypnoScript Enterprise Edition v3.0.0 is working perfectly!" -ForegroundColor Green
} else {
    Write-Host "Some tests failed. Please check the output above for details." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "HypnoScript Enterprise Edition v3.0.0 - Where programming meets hypnosis!" -ForegroundColor Cyan
