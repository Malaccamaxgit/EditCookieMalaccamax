# Fix popup.html to include CSS link and correct title
$cssPath = "E:\Github\EditCookieMalaccamax\public\css\popup.css"
$distPath = "E:\Github\EditCookieMalaccamax\dist\chromium\popup.html"
$manifestPath = "E:\Github\EditCookieMalaccamax\dist\chromium\manifest.json"

# Build HTML with external CSS link (not inline - ensures changes are reflected)
$html = @"
<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>Edit Cookie</title>
<link rel="stylesheet" href="css/popup.css">
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css">
</head>
<body>
<div id="app"></div>
<script type="module" src="popup.js"></script>
</body>
</html>
"@

# Write to dist
$html | Out-File -Encoding UTF8 $distPath
Write-Host "popup.html updated with CSS link"

# Fix manifest.json to add icons and host_permissions
$manifest = Get-Content $manifestPath -Raw | ConvertFrom-Json

# Add icons using Add-Member to add new properties
$icons = @{
    "16" = "icons/icon-16.png"
    "19" = "icons/icon-19.png"
    "32" = "icons/icon-32.png"
    "38" = "icons/icon-38.png"
    "48" = "icons/icon-48.png"
    "128" = "icons/icon-128.png"
}
Add-Member -InputObject $manifest -MemberType NoteProperty -Name "icons" -Value $icons

# Add host_permissions if not present
if (-not $manifest.host_permissions) {
    Add-Member -InputObject $manifest -MemberType NoteProperty -Name "host_permissions" -Value @("<all_urls>")
}

# Write back to file
$manifest | ConvertTo-Json -Depth 10 | Out-File -Encoding UTF8 $manifestPath
Write-Host "manifest.json updated with icons and host_permissions"
