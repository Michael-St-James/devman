# Build and copy journal DLL to Compose app
param(
    [switch]$Release,
    [switch]$Watch
)

$ErrorActionPreference = "Stop"

$profile = if ($Release) { "release" } else { "debug" }
$profileFlag = if ($Release) { "--release" } else { "" }

function Build-And-Copy {
    Write-Host "Building journal crate ($profile)..." -ForegroundColor Cyan

    if ($profileFlag) {
        cargo build -p journal $profileFlag
    } else {
        cargo build -p journal
    }

    if ($LASTEXITCODE -eq 0) {
        # Check both standard and target-specific directories
        $source = "target\$profile\journal.dll"
        if (!(Test-Path $source)) {
            $source = "target\x86_64-pc-windows-gnu\$profile\journal.dll"
        }

        # Generate Kotlin bindings
        Write-Host "Generating Kotlin bindings..." -ForegroundColor Cyan
        cargo run -p uniffi-gen --release -- crates/journal/src/journal_core.udl $source ./uniffi-kotlin

        if ($LASTEXITCODE -eq 0) {
            Write-Host "Generated Kotlin bindings in uniffi-kotlin/" -ForegroundColor Green
        } else {
            Write-Host "Binding generation failed" -ForegroundColor Red
            exit 1
        }

        # Copy DLL to resources directory (for runtime loading)
        $dllDest = "..\compose-app\src\main\resources\journal.dll"
        $dllDestDir = Split-Path -Parent $dllDest
        if (!(Test-Path $dllDestDir)) {
            New-Item -ItemType Directory -Path $dllDestDir -Force | Out-Null
        }
        Copy-Item $source $dllDest -Force
        Write-Host "Copied journal.dll to compose-app/src/main/resources/" -ForegroundColor Green

        # Copy Kotlin bindings (JAR) to libs directory
        $jarDest = "..\compose-app\libs"
        if (!(Test-Path $jarDest)) {
            New-Item -ItemType Directory -Path $jarDest -Force | Out-Null
        }
        if (Test-Path "uniffi-kotlin\uniffi") {
            Copy-Item "uniffi-kotlin\uniffi" $jarDest -Recurse -Force
            Write-Host "Copied Kotlin bindings to compose-app/libs/" -ForegroundColor Green
        }
    } else {
        Write-Host "Build failed" -ForegroundColor Red
        exit 1
    }
}

if ($Watch) {
    Write-Host "Watching for changes... (Ctrl+C to stop)" -ForegroundColor Yellow
    cargo watch -w crates/journal -s 'pwsh -File build-and-copy.ps1'
} else {
    Build-And-Copy
}
