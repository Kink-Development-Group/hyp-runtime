param(
    [Parameter(Position = 0, ValueFromRemainingArguments = $true)]
    [string[]]
    $CliArgs
)

if (-not $CliArgs -or $CliArgs.Length -eq 0) {
    $CliArgs = @("help")
}

$projectRoot = Split-Path -Parent $PSScriptRoot
$entry = Join-Path $projectRoot "src/main.hyp"

if (-not (Test-Path $entry)) {
    throw "Entry script not found at $entry"
}

$runner = Get-Command hypnoscript -ErrorAction SilentlyContinue
if (-not $runner) {
    throw "hypnoscript CLI not found. Please build or install it first."
}

$commandName = $CliArgs[0]
$payloadArgs = if ($CliArgs.Length -gt 1) { $CliArgs[1..($CliArgs.Length - 1)] } else { @() }

$previousCommand = $env:HYPNO_COMMAND
$previousPayload = $env:HYPNO_PAYLOAD

try {
    $env:HYPNO_COMMAND = $commandName
    if ($payloadArgs.Count -gt 0) {
        $env:HYPNO_PAYLOAD = ($payloadArgs -join " ")
    } else {
        Remove-Item Env:HYPNO_PAYLOAD -ErrorAction SilentlyContinue | Out-Null
    }

    $arguments = @("run", $entry)

    Write-Host "== Hypno CLI Dev Loop ==" -ForegroundColor Cyan
    Write-Host "HYPNO_COMMAND=$($env:HYPNO_COMMAND)"
    if ($env:HYPNO_PAYLOAD) {
        Write-Host "HYPNO_PAYLOAD=$($env:HYPNO_PAYLOAD)"
    }
    Write-Host "Executing:" ($runner.Source + " " + ($arguments -join " "))
    Write-Host ""

    & $runner.Source @arguments
}
finally {
    if ($null -ne $previousCommand) {
        $env:HYPNO_COMMAND = $previousCommand
    } else {
        Remove-Item Env:HYPNO_COMMAND -ErrorAction SilentlyContinue | Out-Null
    }

    if ($null -ne $previousPayload) {
        $env:HYPNO_PAYLOAD = $previousPayload
    } else {
        Remove-Item Env:HYPNO_PAYLOAD -ErrorAction SilentlyContinue | Out-Null
    }
}
