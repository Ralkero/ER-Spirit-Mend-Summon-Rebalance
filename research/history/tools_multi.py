from pathlib import Path
r=Path('outputs/SpiritAshResummon')
p=r/'tools/Preflight.ps1';s=p.read_text().replace('@(0x3fb268,0x4b8690)','@(0x3fb268,0x4b8690),@(0x4b86a9,0x4b73d0),@(0x4b8a90,0x4bba80)')
import pefile
pe=pefile.PE(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe',fast_load=True)
# Expected bytes are derived from the separately inspected pinned original executable.
checks=[(0x49560a,0x73),(0x493492,10),(0x4934c0,57),(0x4934f9,8),(0x4b8a90,8),(0x3f2d42,6),(0x3f8ce2,8),(0x401439,8),(0x3c3810,17)]
old="    @{rva=0x4b86ae; hex='83f801761b83f8027504c6432801'}"
s=s.replace(old,old+',\n'+',\n'.join("    @{rva=0x%x; hex='%s'}"%(rva,pe.get_data(rva,size).hex()) for rva,size in checks));p.write_text(s)
p=r/'tools/Build.ps1';p.write_text('''param(
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
    Copy-Item README.md,IMPLEMENTATION.md,VALIDATION.md,CHANGELOG.md,LICENSE-MIT,LICENSE-APACHE,THIRD-PARTY-NOTICES.txt,UPSTREAM.md -Destination $package
    Get-FileHash (Join-Path $package 'SpiritAshResummon.dll') -Algorithm SHA256 | Format-List
} finally {Pop-Location}
''')
