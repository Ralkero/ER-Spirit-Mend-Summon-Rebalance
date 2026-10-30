param(
    [Parameter(Mandatory=$true)][string]$Loader,
    [string]$GameExe = 'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe'
)
$ErrorActionPreference='Stop'
$root=(Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$dll=Join-Path $root 'dist/SpiritAshResummon.dll'
foreach($path in @($Loader,$GameExe,$dll,(Join-Path $root 'dist/mod/regulation.bin'))) {if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {throw "Missing: $path"}}
if (Get-Process eldenring,start_protected_game -ErrorAction SilentlyContinue) {throw 'Exit Elden Ring completely before using this clean test profile.'}
$profile=Join-Path $root 'config_spiritash_test.toml'
$modToml=(Join-Path $root 'dist/mod').Replace('\','/')
$dllToml=$dll.Replace('\','/')
@"
[modengine]
debug = false
external_dlls = ["$dllToml"]

[extension.mod_loader]
enabled = true
loose_params = false
mods = [{ enabled = true, name = "Spirit Mend prototype", path = "$modToml" }]

[extension.scylla_hide]
enabled = false
"@ | Set-Content -LiteralPath $profile -Encoding utf8
Write-Host 'Starting SpiritAshResummon 0.5.1 with Spirit Mend assets. Use the offline test character; see SPIRIT-MEND.md.'
Write-Host 'This profile has no save redirection. Do not load a valuable character; follow VALIDATION.md.'
Push-Location (Split-Path -Parent $Loader)
try { & $Loader -t er -c $profile -p $GameExe; if($LASTEXITCODE -ne 0) {throw "Loader exited with code $LASTEXITCODE"} }
finally {Pop-Location}
