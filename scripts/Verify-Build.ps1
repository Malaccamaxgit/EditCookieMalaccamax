# Verify-Build.ps1
# Build and verification script for Edit Cookie Malaccamax
# Usage: .\scripts\Verify-Build.ps1 [-Release] [-SkipTests]

param(
    [switch]$Release,
    [switch]$SkipTests,
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"
$ProjectRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$DistPath = Join-Path $ProjectRoot "dist"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Edit Cookie Malaccamax - Build Verify" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Change to project directory
Set-Location $ProjectRoot
Write-Host "Project: $ProjectRoot" -ForegroundColor Gray
Write-Host ""

# 1. Check Rust installation
Write-Host "[1/6] Checking Rust installation..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version 2>&1
    Write-Host "  Rust: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "  ERROR: Rust not found. Install from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Check WASM target
$wasmTarget = rustup target list --installed 2>&1 | Select-String "wasm32-unknown-unknown"
if ($wasmTarget) {
    Write-Host "  WASM target: installed" -ForegroundColor Green
} else {
    Write-Host "  ERROR: WASM target not installed. Run: rustup target add wasm32-unknown-unknown" -ForegroundColor Red
    exit 1
}

# Check Oxichrome
try {
    $oxichromeVersion = cargo oxichrome --version 2>&1
    Write-Host "  Oxichrome: $oxichromeVersion" -ForegroundColor Green
} catch {
    Write-Host "  WARNING: Oxichrome not found. Install with: cargo install oxichrome" -ForegroundColor Yellow
}
Write-Host ""

# 2. Format check
Write-Host "[2/6] Checking code format..." -ForegroundColor Yellow
$formatCheck = cargo fmt --check 2>&1
if ($formatCheck) {
    Write-Host "  Format: Issues found (running cargo fmt)" -ForegroundColor Yellow
    cargo fmt --quiet
    Write-Host "  Format: Fixed" -ForegroundColor Green
} else {
    Write-Host "  Format: OK" -ForegroundColor Green
}
Write-Host ""

# 3. Run tests (if not skipped)
if (-not $SkipTests) {
    Write-Host "[3/6] Running tests..." -ForegroundColor Yellow
    $testResult = cargo test 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  Tests: PASSED" -ForegroundColor Green
    } else {
        Write-Host "  Tests: FAILED" -ForegroundColor Red
        Write-Host $testResult -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "[3/6] Skipping tests (-SkipTests)" -ForegroundColor Yellow
}
Write-Host ""

# 4. Build
Write-Host "[4/6] Building project..." -ForegroundColor Yellow
if ($Release) {
    Write-Host "  Mode: Release (optimized)" -ForegroundColor Gray
    $buildResult = cargo build --release 2>&1
} else {
    Write-Host "  Mode: Debug (faster compilation)" -ForegroundColor Gray
    $buildResult = cargo build 2>&1
}

if ($LASTEXITCODE -ne 0) {
    Write-Host "  Build: FAILED" -ForegroundColor Red
    Write-Host $buildResult -ForegroundColor Red
    exit 1
}
Write-Host "  Build: SUCCESS" -ForegroundColor Green
Write-Host ""

# 5. Verify dist folder
Write-Host "[5/6] Verifying build output..." -ForegroundColor Yellow
$requiredFiles = @(
    "dist/manifest.json",
    "dist/popup/popup_bg.wasm",
    "dist/options/options_bg.wasm",
    "dist/css/popup.css",
    "dist/css/options.css",
    "dist/icons/icon16.png"
)

$allPresent = $true
foreach ($file in $requiredFiles) {
    $fullPath = Join-Path $ProjectRoot $file
    if (Test-Path $fullPath) {
        Write-Host "  [OK] $file" -ForegroundColor Green
    } else {
        Write-Host "  [MISSING] $file" -ForegroundColor Red
        $allPresent = $false
    }
}

if (-not $allPresent) {
    Write-Host ""
    Write-Host "  ERROR: Some required files are missing" -ForegroundColor Red
    exit 1
}
Write-Host ""

# 6. Check file sizes
Write-Host "[6/6] Checking file sizes..." -ForegroundColor Yellow
$wasmFile = Join-Path $DistPath "popup/popup_bg.wasm"
if (Test-Path $wasmFile) {
    $wasmSize = (Get-Item $wasmFile).Length / 1KB
    Write-Host "  popup_bg.wasm: {0:N2} KB" -f $wasmSize -ForegroundColor Cyan

    if ($wasmSize -gt 500) {
        Write-Host "  WARNING: WASM file is large (>500KB). Consider release build." -ForegroundColor Yellow
    }
}
Write-Host ""

# Summary
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Build Verification: PASSED" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "1. Open Chrome and navigate to chrome://extensions/" -ForegroundColor White
Write-Host "2. Enable Developer mode (toggle in top-right)" -ForegroundColor White
Write-Host "3. Click 'Load unpacked' and select: $DistPath" -ForegroundColor White
Write-Host ""

if ($Verbose) {
    Write-Host "Build artifacts:" -ForegroundColor Cyan
    Get-ChildItem -Path $DistPath -Recurse -File |
        Select-Object FullName, @{Name="Size(KB)";Expression={[math]::Round($_.Length/1KB, 2)}} |
        Format-Table -AutoSize
}
