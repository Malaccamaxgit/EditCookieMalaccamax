# Launch Chrome with remote debugging using default profile
$chromePath = "C:\Program Files\Google\Chrome\Application\chrome.exe"
$userDataDir = "C:\Users\benja\AppData\Local\Google\Chrome\User Data"

Write-Host "Launching Chrome with remote debugging..."
Write-Host "Chrome path: $chromePath"
Write-Host "User data dir: $userDataDir"

Start-Process $chromePath -ArgumentList "--remote-debugging-port=9222", "--user-data-dir=$userDataDir", "--no-first-run"

Write-Host "Waiting for Chrome to start..."
Start-Sleep -Seconds 5

# Verify Chrome is running
$chromeProcesses = Get-Process chrome -ErrorAction SilentlyContinue
if ($chromeProcesses) {
    Write-Host "Chrome is running ($($chromeProcesses.Count) processes)"

    # Test debug endpoint
    try {
        $response = Invoke-WebRequest -Uri "http://127.0.0.1:9222/json" -TimeoutSec 3 -UseBasicParsing
        Write-Host "Debug endpoint is ready!"
        Write-Host "Response: $($response.Content.Substring(0, [Math]::Min(100, $response.Content.Length)))..."
    } catch {
        Write-Host "Debug endpoint not responding yet"
    }
} else {
    Write-Host "ERROR: Chrome failed to start"
}
