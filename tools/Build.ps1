param(
    [string]$TargetDirectory = (Join-Path $PSScriptRoot '../target'),
    [string]$OutputDirectory = (Join-Path $PSScriptRoot '../package'),
    [switch]$RunTests
)
$ErrorActionPreference='Stop'
$root=(Resolve-Path (Join-Path $PSScriptRoot '..')).Path
Push-Location $root
try {
    $target=[IO.Path]::GetFullPath($TargetDirectory)
    $package=[IO.Path]::GetFullPath($OutputDirectory)
    if (Test-Path -LiteralPath (Join-Path $package 'SpiritAshResummon.dll')) {
        throw 'Package DLL already exists; select a new OutputDirectory to preserve existing files.'
    }
    if ($package -eq (Join-Path $root 'dist')) { throw 'Use a separate staging folder, not the installed dist directory.' }
    New-Item -ItemType Directory -Force $package,evidence | Out-Null
    if ($RunTests) {
        & cargo test --locked --target x86_64-pc-windows-msvc --target-dir $target 2>&1 | Tee-Object evidence/tests.txt
        if ($LASTEXITCODE -ne 0) {throw 'Tests failed'}
    }
    & cargo build --lib --locked --release --target x86_64-pc-windows-msvc --target-dir $target 2>&1 | Tee-Object evidence/build.txt
    if ($LASTEXITCODE -ne 0) {throw 'Build failed'}
    Copy-Item -LiteralPath (Join-Path $target 'x86_64-pc-windows-msvc/release/SpiritAshResummon.dll') -Destination $package
    Copy-Item -LiteralPath config/SpiritAshResummon.ini -Destination $package
    Copy-Item README.md,ANALYSIS.md,PUBLICATION.md,SPIRIT-MEND.md,IMPLEMENTATION.md,VALIDATION.md,CHANGELOG.md,LICENSE-MIT,LICENSE-APACHE,THIRD-PARTY-NOTICES.txt,UPSTREAM.md -Destination $package
    Copy-Item -LiteralPath config/SpiritMend-Prototype.ini -Destination $package
    Get-FileHash (Join-Path $package 'SpiritAshResummon.dll') -Algorithm SHA256 | Format-List
} finally {Pop-Location}
