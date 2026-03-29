# Build script for Edit Cookie
Write-Host "Building Edit Cookie..." -ForegroundColor Cyan

# Set up PATH for cargo
$env:Path = "C:\Users\benja\.cargo\bin;" + $env:Path

# Clean build
$distPath = "E:\Github\EditCookieMalaccamax\dist\chromium"
if (Test-Path $distPath) {
    Write-Host "Cleaning previous build..." -ForegroundColor Yellow
    Remove-Item -Recurse -Force $distPath
}

# Build with Oxichrome
Write-Host "Running cargo oxichrome build --release..." -ForegroundColor Yellow
Set-Location E:\Github\EditCookieMalaccamax
cargo oxichrome build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

# Run post-build fixes
Write-Host "Applying post-build fixes..." -ForegroundColor Yellow
pwsh -ExecutionPolicy Bypass -File ./fix-popup.ps1
pwsh -ExecutionPolicy Bypass -File ./fix-manifest.ps1

# Copy static assets
Write-Host "Copying static assets..." -ForegroundColor Yellow
if (Test-Path ".\public\css") {
    New-Item -ItemType Directory -Force -Path "$distPath\css" | Out-Null
    Copy-Item -Recurse -Force ".\public\css\*" "$distPath\css\"
    Write-Host "  [OK] CSS files copied" -ForegroundColor Green
}
if (Test-Path ".\public\icons") {
    New-Item -ItemType Directory -Force -Path "$distPath\icons" | Out-Null
    Copy-Item -Recurse -Force ".\public\icons\*" "$distPath\icons\"
    Write-Host "  [OK] Icons copied" -ForegroundColor Green
}
if (Test-Path ".\public\devtools") {
    New-Item -ItemType Directory -Force -Path "$distPath\devtools" | Out-Null
    Copy-Item -Recurse -Force ".\public\devtools\*" "$distPath\devtools\"
    Write-Host "  [OK] DevTools files copied" -ForegroundColor Green
}

Write-Host "Build complete!" -ForegroundColor Green
Write-Host "Extension is in: $distPath" -ForegroundColor Gray
Write-Host ""
Write-Host "To load in Chrome:" -ForegroundColor Yellow
Write-Host "1. Open chrome://extensions/"
Write-Host "2. Enable Developer mode"
Write-Host "3. Click 'Load unpacked'"
Write-Host "4. Select: $distPath"
