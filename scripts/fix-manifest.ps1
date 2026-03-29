# Fix manifest.json to add host_permissions, icons, and correct name
$manifestPath = "E:\Github\EditCookieMalaccamax\dist\chromium\manifest.json"

# Read manifest
$manifest = Get-Content $manifestPath -Raw | ConvertFrom-Json

# Update name
$manifest.name = "Edit Cookie"

# Add host_permissions
$manifest.host_permissions = @("<all_urls>")

# Add icons
$manifest.icons = @{
    "16" = "icons/icon-16.png"
    "19" = "icons/icon-19.png"
    "32" = "icons/icon-32.png"
    "38" = "icons/icon-38.png"
    "48" = "icons/icon-48.png"
    "128" = "icons/icon-128.png"
}

# Write back
$manifest | ConvertTo-Json -Depth 10 | Out-File -Encoding UTF8 $manifestPath
Write-Host "manifest.json updated with name, host_permissions, and icons"
