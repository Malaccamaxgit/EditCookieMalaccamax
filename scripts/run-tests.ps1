# Run tests for Edit Cookie Malaccamax
$env:Path = "C:\Users\benja\.cargo\bin;" + $env:Path
Set-Location E:\Github\EditCookieMalaccamax

Write-Host "Running cargo test..." -ForegroundColor Cyan
cargo test
