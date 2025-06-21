Write-Host "=== HypnoScript CLI Test ===" -ForegroundColor Green

# Prüfe, ob .NET verfügbar ist
try {
    $dotnetVersion = dotnet --version
    Write-Host ".NET Version: $dotnetVersion" -ForegroundColor Yellow
} catch {
    Write-Host "Fehler: .NET ist nicht verfügbar" -ForegroundColor Red
    exit 1
}

# Prüfe, ob die Testdatei existiert
if (-not (Test-Path "test_simple.hyp")) {
    Write-Host "Fehler: test_simple.hyp nicht gefunden" -ForegroundColor Red
    exit 1
}

# Zeige den Inhalt der Testdatei
Write-Host "Testdatei Inhalt:" -ForegroundColor Yellow
Get-Content "test_simple.hyp" | ForEach-Object { Write-Host "  $_" }

# Versuche die CLI zu starten
Write-Host "`nStarte CLI..." -ForegroundColor Yellow
try {
    dotnet run --project HypnoScript.CLI run test_simple.hyp --debug
    Write-Host "CLI erfolgreich ausgeführt!" -ForegroundColor Green
} catch {
    Write-Host "Fehler beim Ausführen der CLI: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "Test beendet." -ForegroundColor Green
