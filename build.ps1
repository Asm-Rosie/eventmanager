$currentPath = Get-Location

Copy-Item "$currentPath\rust_core\Cargo-win.toml" -Destination "$currentPath\rust_core\Cargo.toml"

Write-Host "copied necessary files"

Write-Host "Building Windows QtWidget application.."

& rustup target add x86_64-pc-windows-msvc
& rustup default stable-x86_64-pc-windows-gnu

Write-Host "targets up to date"
Write-Host "building for windows x64.."

& cargo build --release --target x86_64-pc-windows-msvc --manifest-path ./rust_core/Cargo.toml

Write-Host "completed building.."

$currentPath = Get-Location

Copy-Item -Path "$currentPath\rust_core\target\x86_64-pc-windows-msvc\release\eventmanager_core.dll.lib" -Destination "$currentPath\qt-eventmanager\eventmanager_core.dll.lib"
Copy-Item -Path "$currentPath\rust_core\target\x86_64-pc-windows-msvc\release\eventmanager_core.dll" -Destination "$currentPath\qt-eventmanager\eventmanager_core.dll"
Copy-Item -Path "$currentPath\rust_core\target\x86_64-pc-windows-msvc\release\eventmanager_core.pdb" -Destination "$currentPath\qt-eventmanager\eventmanager_core.pdb"
