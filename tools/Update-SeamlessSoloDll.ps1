param(
 [Parameter(Mandatory)][string]$ProfileRoot,
 [Parameter(Mandatory)][string]$PackageDirectory,
 [switch]$Apply
)
$ErrorActionPreference='Stop'
function Hash([string]$Path){ (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash }
function Closed { if(Get-Process eldenring,ersc_launcher,modengine2_launcher -ErrorAction SilentlyContinue){throw 'Close Elden Ring and its launchers before applying the DLL update.'} }
$root=(Resolve-Path -LiteralPath $ProfileRoot).Path
$package=(Resolve-Path -LiteralPath $PackageDirectory).Path
$target=Join-Path $root 'dll/spirit-mend-solo-0.6.1/SpiritAshResummon.dll'
$source=Join-Path $package 'SpiritAshResummon.dll'
$manifest=Get-Content -LiteralPath (Join-Path $package 'manifest.json') -Raw | ConvertFrom-Json
if($manifest.version -ne '0.6.1-solo.5'){throw 'Unexpected update package.'}
if((Hash $source) -ne $manifest.dllSha256){throw 'Package hash mismatch.'}
$before=Hash $target
if($before -ne '9166CD937A0B5A96DE69C1B34F1810E8CDBC52F263384E6F13695024C263D183'){throw 'Expected the inspected 0.6.1-solo.4 DLL; review before updating another build.'}
$protected=@(
 (Join-Path $root 'config_eldenring.toml')
 (Join-Path $root 'launchmod_eldenring.bat')
 (Join-Path (Split-Path -Parent $target) 'SpiritAshResummon.ini')
 (Get-ChildItem -LiteralPath (Join-Path $root 'mod') -Recurse -File | Where-Object { $_.Name -eq 'regulation.bin' -or $_.Name -like '*.msgbnd.dcx' -or $_.Name -like '*.talkesdbnd.dcx' }).FullName
) | Where-Object {$_}
$baseline=@($protected | ForEach-Object{[pscustomobject]@{path=$_;sha256=(Hash $_)}})
if(-not $Apply){Write-Output "Preview: replace only $target; preserve $($baseline.Count) configuration/asset files. Game closure required for -Apply.";return}
Closed
$backup=Join-Path $root ('backups/spirit-mend-solo5-'+(Get-Date -Format 'yyyyMMdd-HHmmss'))
New-Item -ItemType Directory -Path $backup | Out-Null
Copy-Item -LiteralPath $target -Destination (Join-Path $backup 'SpiritAshResummon.dll')
if((Hash (Join-Path $backup 'SpiritAshResummon.dll')) -ne $before){throw 'Backup hash mismatch.'}
$log=Join-Path (Split-Path -Parent $target) 'SpiritAshResummon.log'
if(Test-Path -LiteralPath $log){Copy-Item -LiteralPath $log -Destination $backup}
$baseline | ConvertTo-Json -Depth 3 | Set-Content -LiteralPath (Join-Path $backup 'preservation-before.json') -Encoding utf8
$prepared=Join-Path $backup 'replacement.dll'
Copy-Item -LiteralPath $source -Destination $prepared
if((Hash $prepared) -ne $manifest.dllSha256){throw 'Prepared DLL hash mismatch.'}
Closed
if((Hash $target) -ne $before){throw 'Installed DLL changed during preparation.'}
[IO.File]::Replace($prepared,$target,(Join-Path $backup 'atomic-previous.dll'))
if((Hash $target) -ne $manifest.dllSha256){throw 'Installed DLL readback mismatch.'}
foreach($file in $baseline){if((Hash $file.path) -ne $file.sha256){throw "Protected file changed: $($file.path)"}}
[ordered]@{version=$manifest.version;sourceCommit=$manifest.sourceCommit;target=$target;before=$before;after=$manifest.dllSha256;protected=$baseline;gameplayTested=$false} |
 ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $backup 'installation.json') -Encoding utf8
@'
param([switch]$Apply)
$ErrorActionPreference='Stop'
$r=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'installation.json') -Raw | ConvertFrom-Json
if(Get-Process eldenring,ersc_launcher,modengine2_launcher -ErrorAction SilentlyContinue){throw 'Close game and launchers.'}
if((Get-FileHash -LiteralPath $r.target).Hash -ne $r.after){throw 'Installed DLL changed; review rollback.'}
$source=Join-Path $PSScriptRoot 'SpiritAshResummon.dll'
if((Get-FileHash -LiteralPath $source).Hash -ne $r.before){throw 'Backup changed.'}
if(-not $Apply){'Preview: restore the previous DLL; preserve all other files.';return}
Copy-Item -LiteralPath $source -Destination $r.target
if((Get-FileHash -LiteralPath $r.target).Hash -ne $r.before){throw 'Rollback verification failed.'}
'Previous DLL restored.'
'@ | Set-Content -LiteralPath (Join-Path $backup 'Restore.ps1') -Encoding utf8
Write-Output "Updated DLL to $($manifest.version); $($baseline.Count) configuration/asset files unchanged. Record: $backup/installation.json"
