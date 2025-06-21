Write-Host "=== HypnoScript CLI - Vollständiger Test ===" -ForegroundColor Green

# Testdateien definieren
$testFiles = @("test_simple.hyp", "test_advanced.hyp")

foreach ($file in $testFiles) {
    if (-not (Test-Path $file)) {
        Write-Host "⚠ Warnung: $file nicht gefunden, überspringe..." -ForegroundColor Yellow
        continue
    }

    Write-Host "`n📁 Teste Datei: $file" -ForegroundColor Cyan

    # 1. Analyze-Test
    Write-Host "`n🔍 ANALYZE-Test:" -ForegroundColor Yellow
    try {
        dotnet run --project HypnoScript.CLI analyze $file
        Write-Host "✅ Analyze erfolgreich!" -ForegroundColor Green
    } catch {
        Write-Host "❌ Analyze fehlgeschlagen: $($_.Exception.Message)" -ForegroundColor Red
    }

    # 2. Compile-Test
    Write-Host "`n🔨 COMPILE-Test:" -ForegroundColor Yellow
    try {
        dotnet run --project HypnoScript.CLI compile $file
        $watFile = [System.IO.Path]::ChangeExtension($file, ".wat")
        if (Test-Path $watFile) {
            Write-Host "✅ Compile erfolgreich! WAT-Datei erstellt: $watFile" -ForegroundColor Green
        } else {
            Write-Host "⚠ Compile abgeschlossen, aber WAT-Datei nicht gefunden" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "❌ Compile fehlgeschlagen: $($_.Exception.Message)" -ForegroundColor Red
    }

    # 3. Run-Test
    Write-Host "`n▶️ RUN-Test:" -ForegroundColor Yellow
    try {
        dotnet run --project HypnoScript.CLI run $file
        Write-Host "✅ Run erfolgreich!" -ForegroundColor Green
    } catch {
        Write-Host "❌ Run fehlgeschlagen: $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host "`n🎉 Vollständiger Test abgeschlossen!" -ForegroundColor Green
Write-Host "`n📋 Verfügbare Befehle:" -ForegroundColor Cyan
Write-Host "  dotnet run --project HypnoScript.CLI run <datei.hyp> [--debug]" -ForegroundColor White
Write-Host "  dotnet run --project HypnoScript.CLI compile <datei.hyp> [--debug]" -ForegroundColor White
Write-Host "  dotnet run --project HypnoScript.CLI analyze <datei.hyp> [--debug]" -ForegroundColor White
