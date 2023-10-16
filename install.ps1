Write-Host "starting installation"

$cargoCommand = Get-Command cargo -ErrorAction SilentlyContinue

if ($cargoCommand) {
    $cargoVersionOutput = & $cargoCommand -V
    if ($cargoVersionOutput -match 'cargo (.+?)') {
        $cargoVersion = $matches[1]
        Write-Host "Cargo is installed: $cargoVersion" -ForegroundColor Green
    }
} else {
    Write-Host "Cargo is not installed." -ForegroundColor Red
    Write-Host "This script can install cargo and all necessary tools for you."
    Write-Host "Do you want to install these tools?"
    Write-Host "1. Yes"
    Write-Host "2. No"
    $install_tools = Read-Host ":"
    Write-Host "you selected: $install_tools"

    if ($install_tools -eq "1") {
        Write-Host "1."
        $output_location = "rustup-init.exe"
        $download_url = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
        Invoke-WebRequest -Uri $download_url -OutFile $output_location
        Start-Process -FilePath "./rustup-init.exe"
        Write-Host "after the installation is complete restart your powershell enviornment and run the build script using the following command"
        Write-Host "./build.ps1"

    } elseif ($install_tools -eq "2") {
        exit 0
    }

    
}