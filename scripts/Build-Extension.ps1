#!/usr/bin/env pwsh
# Build the extension and copy additional assets that Oxichrome doesn't handle.

param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

$buildArgs = @("oxichrome", "build")
if ($Release) { $buildArgs += "--release" }

Write-Host "[build] Running cargo $($buildArgs -join ' ')..." -ForegroundColor Cyan
& cargo @buildArgs
if ($LASTEXITCODE -ne 0) { throw "cargo build failed (exit $LASTEXITCODE)" }

$root = Split-Path $PSScriptRoot -Parent
$dist = [IO.Path]::Combine($root, "dist", "chromium")
$pub  = [IO.Path]::Combine($root, "public")

# Re-copy page CSS files so Oxichrome's @import stripping is undone
foreach ($css in @("popup.css", "options.css", "devtools.css", "fontawesome.min.css")) {
    $src = [IO.Path]::Combine($pub, "css", $css)
    if (Test-Path $src) {
        Copy-Item $src -Destination ([IO.Path]::Combine($dist, "css", $css)) -Force
        Write-Host "[build] Copied css/$css" -ForegroundColor Green
    }
}

# Copy webfonts
$webfonts = [IO.Path]::Combine($pub, "webfonts")
if (Test-Path $webfonts) {
    $destFonts = [IO.Path]::Combine($dist, "webfonts")
    if (!(Test-Path $destFonts)) { New-Item -ItemType Directory -Path $destFonts -Force | Out-Null }
    Copy-Item "$webfonts\*" -Destination $destFonts -Force
    Write-Host "[build] Copied webfonts/" -ForegroundColor Green
}

# Copy extension icons
$icons = [IO.Path]::Combine($pub, "icons")
if (Test-Path $icons) {
    $destIcons = [IO.Path]::Combine($dist, "icons")
    if (!(Test-Path $destIcons)) { New-Item -ItemType Directory -Path $destIcons -Force | Out-Null }
    Copy-Item "$icons\*" -Destination $destIcons -Force
    Write-Host "[build] Copied icons/" -ForegroundColor Green
}

# Refresh devtools panel.html (Oxichrome may cache stale copies)
$panelSrc  = [IO.Path]::Combine($pub, "devtools", "panel.html")
$panelDest = [IO.Path]::Combine($dist, "devtools", "panel.html")
if (Test-Path $panelSrc) {
    Copy-Item $panelSrc -Destination $panelDest -Force
    Write-Host "[build] Refreshed devtools/panel.html" -ForegroundColor Green
}

# Refresh devtools-page.js
$jsSrc  = [IO.Path]::Combine($pub, "js", "devtools-page.js")
$jsDest = [IO.Path]::Combine($dist, "js", "devtools-page.js")
if (Test-Path $jsSrc) {
    $destJsDir = [IO.Path]::Combine($dist, "js")
    if (!(Test-Path $destJsDir)) { New-Item -ItemType Directory -Path $destJsDir -Force | Out-Null }
    Copy-Item $jsSrc -Destination $jsDest -Force
    Write-Host "[build] Refreshed js/devtools-page.js" -ForegroundColor Green
}

# Patch manifest.json: Oxichrome doesn't emit host_permissions or icons
$manifestPath = [IO.Path]::Combine($dist, "manifest.json")
$manifest = Get-Content $manifestPath -Raw | ConvertFrom-Json

$changed = $false

# Remove web_accessible_resources — extension pages can always load their
# own bundled WASM; exposing resources to <all_urls> is a fingerprinting vector.
if ($manifest.PSObject.Properties["web_accessible_resources"]) {
    $manifest.PSObject.Properties.Remove("web_accessible_resources")
    $changed = $true
}

if (-not $manifest.PSObject.Properties["host_permissions"]) {
    Add-Member -InputObject $manifest -MemberType NoteProperty -Name "host_permissions" -Value @("<all_urls>")
    $changed = $true
}

if (-not $manifest.PSObject.Properties["icons"]) {
    $iconMap = @{
        "16"  = "icons/icon-16.png"
        "19"  = "icons/icon-19.png"
        "32"  = "icons/icon-32.png"
        "38"  = "icons/icon-38.png"
        "48"  = "icons/icon-48.png"
        "128" = "icons/icon-128.png"
    }
    Add-Member -InputObject $manifest -MemberType NoteProperty -Name "icons" -Value $iconMap
    $changed = $true
}

if (-not $manifest.action.PSObject.Properties["default_icon"]) {
    $actionIcons = @{
        "16" = "icons/icon-16.png"
        "32" = "icons/icon-32.png"
        "48" = "icons/icon-48.png"
    }
    Add-Member -InputObject $manifest.action -MemberType NoteProperty -Name "default_icon" -Value $actionIcons
    $changed = $true
}

if ($changed) {
    $manifest | ConvertTo-Json -Depth 10 | Set-Content -Encoding UTF8 $manifestPath
    Write-Host "[build] Patched manifest.json (host_permissions, icons)" -ForegroundColor Green
}

Write-Host "[build] Done. Load dist/chromium/ as unpacked extension." -ForegroundColor Cyan
