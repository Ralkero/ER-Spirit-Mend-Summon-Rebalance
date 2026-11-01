param(
    [Parameter(Mandatory)][string]$ProfileRoot,
    [Parameter(Mandatory)][string]$PackageDirectory,
    [Parameter(Mandatory)][string]$GameDirectory,
    [switch]$Apply
)
$ErrorActionPreference='Stop'
function Hash([string]$Path) { (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash }
function RequireClosed {
    if(Get-Process eldenring,ersc_launcher,modengine2_launcher -ErrorAction SilentlyContinue) { throw 'Close the game and launchers first.' }
}
RequireClosed
$root=(Resolve-Path -LiteralPath $ProfileRoot).Path
$package=(Resolve-Path -LiteralPath $PackageDirectory).Path
$game=(Resolve-Path -LiteralPath $GameDirectory).Path
$config=Join-Path $root 'config_eldenring.toml'
$dllDirectory=Join-Path $root 'dll/spirit-mend-solo-0.6.1'
$modDirectory=Join-Path $root 'mod/spirit-mend-solo-0.6.1'
if((Test-Path -LiteralPath $dllDirectory) -or (Test-Path -LiteralPath $modDirectory)) { throw 'Solo destination already exists; refusing overwrite.' }
if((Hash (Join-Path $game 'eldenring.exe')) -ne 'F9118FEA2CFD8FA3901E8060BFADB1552CB2471771DED8A5B662C4E298FFB328') { throw 'Unreviewed executable.' }
if((Hash (Join-Path $game 'SeamlessCoop/ersc.dll')) -ne '0A093D8490967BF19C62F895FDF1FDA21840282D2B2B371B9B4FC24A0CFE146E') { throw 'Unreviewed Seamless DLL.' }
if((Hash (Join-Path $root 'mod/terra-self-buff/regulation.bin')) -ne 'EFE8A382C5424DB926E1E27393EE533D3F094372AFBB91F694655146F2B532E3') { throw 'Reviewed parameter base changed.' }
if(Get-ChildItem -LiteralPath (Join-Path $root 'mod') -Recurse -File -Filter 'm60_00_00_00.talkesdbnd.dcx') { throw 'An existing mod overrides the required talk bundle; merge review needed.' }
$beforeHash=Hash $config
$before=[IO.File]::ReadAllText($config)
$probe='dll/spirit-mend-probe/SpiritMendCompatibilityProbe.dll'
if([regex]::Matches($before,[regex]::Escape($probe)).Count -ne 1 -or $before -match '(?i)SpiritAshResummon\.dll') { throw 'Expected the diagnostic-only profile.' }
$after=$before.Replace($probe,'dll/spirit-mend-solo-0.6.1/SpiritAshResummon.dll')
$pattern='(?m)^mods\s*=\s*\[\r?$'
if([regex]::Matches($after,$pattern).Count -ne 1) { throw 'Unexpected mods array.' }
$after=[regex]::Replace($after,$pattern,"mods = [`r`n  { enabled = true, name = `"spirit-mend-solo-0.6.1`", path = `"mod/spirit-mend-solo-0.6.1`" },")
$manifest=Get-Content -LiteralPath (Join-Path $package 'manifest.json') -Raw | ConvertFrom-Json
if($manifest.kind -ne 'spirit-mend-seamless-solo-0.6.1') { throw 'Unexpected package.' }
$payload=@(foreach($file in $manifest.files) {
    if($file.path -notmatch '^(dll|mod)/(.+)$') { throw 'Unexpected payload path.' }
    $sub=$Matches[2];$base=if($Matches[1] -eq 'dll'){$dllDirectory}else{$modDirectory}
    $source=[IO.Path]::GetFullPath((Join-Path $package $file.path));$destination=[IO.Path]::GetFullPath((Join-Path $base $sub))
    if(-not $source.StartsWith($package.TrimEnd('\')+'\',[StringComparison]::OrdinalIgnoreCase) -or -not $destination.StartsWith($base.TrimEnd('\')+'\',[StringComparison]::OrdinalIgnoreCase)) { throw 'Payload escapes directory.' }
    if((Hash $source) -ne $file.sha256) { throw 'Package hash mismatch.' }
    [pscustomobject]@{source=$source;destination=$destination;sha256=$file.sha256}
})
foreach($required in @('dll/SpiritAshResummon.dll','dll/SpiritAshResummon.ini','mod/regulation.bin','mod/script/talk/m60_00_00_00.talkesdbnd.dcx')) {
    if(@($manifest.files | Where-Object path -eq $required).Count -ne 1) { throw "Missing package entry: $required" }
}
$protected=@(
    (Get-ChildItem -LiteralPath (Join-Path $root 'mod') -Recurse -File | Where-Object { $_.Name -eq 'regulation.bin' -or $_.Name -like '*.msgbnd.dcx' -or $_.Name -like '*.talkesdbnd.dcx' }).FullName
    (Get-ChildItem -LiteralPath (Join-Path $root 'dll') -Recurse -File | Where-Object Extension -eq '.dll').FullName
    (Join-Path $root 'launchmod_eldenring.bat')
    (Join-Path $game 'eldenring.exe')
    (Join-Path $game 'SeamlessCoop/ersc.dll')
    (Join-Path $game 'SeamlessCoop/ersc_settings.ini')
) | Where-Object { $_ }
$baseline=@($protected | ForEach-Object { [pscustomobject]@{path=$_;sha256=(Hash $_)} })
if(-not $Apply) { Write-Output "Preview: replace diagnostic entry, add merged overlay, preserve $($baseline.Count) files. Use -Apply."; return }
RequireClosed
if((Hash $config) -ne $beforeHash) { throw 'Profile changed during preparation.' }
$backup=Join-Path $root ('backups/spirit-mend-solo-'+(Get-Date -Format 'yyyyMMdd-HHmmss'))
New-Item -ItemType Directory -Path $backup | Out-Null
Copy-Item -LiteralPath $config -Destination (Join-Path $backup 'config_eldenring.toml')
if((Hash (Join-Path $backup 'config_eldenring.toml')) -ne $beforeHash) { throw 'Backup mismatch.' }
$baseline | ConvertTo-Json -Depth 3 | Set-Content -LiteralPath (Join-Path $backup 'preservation-before.json') -Encoding utf8
foreach($file in $payload) {
    New-Item -ItemType Directory -Path (Split-Path -Parent $file.destination) -Force | Out-Null
    Copy-Item -LiteralPath $file.source -Destination $file.destination
    if((Hash $file.destination) -ne $file.sha256) { throw 'Copied payload hash mismatch.' }
}
$prepared=Join-Path $backup 'config.prepared.toml'
[IO.File]::WriteAllText($prepared,$after,[Text.UTF8Encoding]::new($false))
RequireClosed
if((Hash $config) -ne $beforeHash) { throw 'Profile changed before activation.' }
[IO.File]::Replace($prepared,$config,(Join-Path $backup 'config.atomic-backup.toml'))
if([IO.File]::ReadAllText($config) -ne $after) { throw 'Config readback mismatch.' }
foreach($file in $baseline) { if((Hash $file.path) -ne $file.sha256) { throw "Protected file changed: $($file.path)" } }
[ordered]@{version='0.6.1-solo.1';sourceCommit=$manifest.sourceCommit;config=$config;configBefore=$beforeHash;configAfter=(Hash $config);backup=$backup;payload=$payload;protected=$baseline;gameplayTested=$false} |
    ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $backup 'installation.json') -Encoding utf8
@'
param([switch]$Apply)
$ErrorActionPreference='Stop'
$r=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'installation.json') -Raw | ConvertFrom-Json
if(Get-Process eldenring,ersc_launcher,modengine2_launcher -ErrorAction SilentlyContinue) { throw 'Close game and launchers.' }
if((Get-FileHash -LiteralPath $r.config).Hash -ne $r.configAfter) { throw 'Profile changed; review before rollback.' }
$source=Join-Path $PSScriptRoot 'config_eldenring.toml'
if((Get-FileHash -LiteralPath $source).Hash -ne $r.configBefore) { throw 'Backup changed.' }
if(-not $Apply) { 'Preview: restore the pre-solo profile; leave new folders inactive.';return }
Copy-Item -LiteralPath $source -Destination $r.config
if((Get-FileHash -LiteralPath $r.config).Hash -ne $r.configBefore) { throw 'Rollback hash mismatch.' }
'Pre-solo profile restored. New files retained but inactive.'
'@ | Set-Content -LiteralPath (Join-Path $backup 'Restore.ps1') -Encoding utf8
Write-Output "Installed solo build; $($payload.Count) payload files verified, $($baseline.Count) existing files preserved. Record: $backup/installation.json"
