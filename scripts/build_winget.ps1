# build_winget.ps1
# Erstellt ein self-contained Windows-Binary und bereitet das winget-Paket vor

$ErrorActionPreference = 'Stop'

# 1. Build
Write-Host 'Baue self-contained Windows-Binary...'
dotnet publish ../HypnoScript.CLI -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true -o ../publish/win

# 2. Optional: ZIP für winget
Write-Host 'Erstelle ZIP-Archiv für winget...'
$zipPath = '../publish/HypnoScript-windows-x64.zip'
if (Test-Path $zipPath) { Remove-Item $zipPath }
Compress-Archive -Path ../publish/win/* -DestinationPath $zipPath

# 3. SHA256 berechnen und ins Manifest eintragen
Write-Host 'Berechne SHA256-Hash für ZIP...'
$sha256 = (Get-FileHash $zipPath -Algorithm SHA256).Hash
$manifestPath = 'winget-manifest.yaml'
$manifestContent = Get-Content $manifestPath
$updatedContent = $manifestContent -replace '(InstallerSha256: ).*', "`$1$sha256"
$updatedContent | Set-Content $manifestPath
Write-Host "SHA256 ($sha256) wurde ins Manifest eingetragen."

Write-Host 'Fertig! Release liegt in ../publish/win und als ZIP vor.'
