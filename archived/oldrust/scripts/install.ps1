# Dryad Automatic Installer (Windows)
# Usage: iwr -useb https://dryadlang.org/install.ps1 | iex

$DRYAD_HOME = Join-Path $HOME ".dryad"
$DRYAD_BIN = Join-Path $DRYAD_HOME "bin"
$BASE_URL = "https://github.com/dryad-lang/source/releases/latest/download"

Write-Host "Installing Dryad for Windows..." -ForegroundColor Cyan

# Create directory structure
if (!(Test-Path $DRYAD_BIN)) {
    New-Item -ItemType Directory -Force -Path $DRYAD_BIN | Out-Null
}

# Download binaries
Write-Host "Downloading dryad.exe..."
Invoke-WebRequest -Uri "$BASE_URL/dryad-windows-x86_64.exe" -OutFile (Join-Path $DRYAD_BIN "dryad.exe")

Write-Host "Downloading oak.exe..."
Invoke-WebRequest -Uri "$BASE_URL/oak-windows-x86_64.exe" -OutFile (Join-Path $DRYAD_BIN "oak.exe")

# Update PATH
Write-Host "Updating PATH environment variable..."
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$DRYAD_BIN*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$DRYAD_BIN", "User")
    $env:Path += ";$DRYAD_BIN"
}

Write-Host "--------------------------------------------------------" -ForegroundColor Green
Write-Host "Dryad has been installed to $DRYAD_BIN" -ForegroundColor Green
Write-Host "The PATH has been updated. You may need to restart your terminal." -ForegroundColor Yellow
Write-Host "--------------------------------------------------------" -ForegroundColor Green
Write-Host "Run 'dryad --version' to verify the installation."
