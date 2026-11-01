param([Parameter(Mandatory)][string]$ProfileRoot,[Parameter(Mandatory)][string]$PackageDirectory,[switch]$Apply)
$ErrorActionPreference='Stop'
function Hash([string]$p){(Get-FileHash -LiteralPath $p -Algorithm SHA256).Hash}
function Closed {if(Get-Process eldenring,ersc_launcher,modengine2_launcher -ErrorAction SilentlyContinue){throw 'Close Elden Ring and its launchers before installing shop changes.'}}
$root=(Resolve-Path -LiteralPath $ProfileRoot).Path
$package=(Resolve-Path -LiteralPath $PackageDirectory).Path
$m=Get-Content -LiteralPath (Join-Path $package 'manifest.json') -Raw | ConvertFrom-Json
if($m.version -ne 'shop-raya-lucaria-2' -or $m.shopRow -ne 100699 -or $m.oldShopRow -ne 100599){throw 'Unexpected shop package'}
$overlay=Join-Path $root 'mod/spirit-mend-solo-0.6.1'
$rows=@(
 [pscustomobject]@{name='regulation.bin';target=(Join-Path $overlay 'regulation.bin');source=(Join-Path $package 'mod/regulation.bin');before='A4CDE09BA41258CA766D15510FA50943034DD333336451596EC6455457AE63B5';after=$m.regulationSha256},
 [pscustomobject]@{name='m60_00_00_00.talkesdbnd.dcx';target=(Join-Path $overlay 'script/talk/m60_00_00_00.talkesdbnd.dcx');source=(Join-Path $package 'mod/script/talk/m60_00_00_00.talkesdbnd.dcx');before='28B7B31185D6FACD4DB694D3EDF648663E99001D30FFEBFD846C520E8EA9B394';after=$m.talkSha256}
)
foreach($r in $rows){if((Hash $r.target) -ne $r.before -or (Hash $r.source) -ne $r.after){throw "Source/installed hash mismatch: $($r.name)"}}
$protected=@((Join-Path $root 'config_eldenring.toml'),(Join-Path $root 'launchmod_eldenring.bat'),(Join-Path $root 'dll/spirit-mend-solo-0.6.1/SpiritAshResummon.dll'),(Join-Path $root 'dll/spirit-mend-solo-0.6.1/SpiritAshResummon.ini')) + @((Get-ChildItem -LiteralPath (Join-Path $root 'mod') -Recurse -File | Where-Object {($_.Name -eq 'regulation.bin' -or $_.Name -like '*.msgbnd.dcx' -or $_.Name -like '*.talkesdbnd.dcx') -and $_.FullName -notin $rows.target}).FullName)
$baseline=@($protected | ForEach-Object {[pscustomobject]@{path=$_;sha256=(Hash $_)}})
if(-not $Apply){"Preview: move Spirit Mend to Isolated Merchant's Purchase menu (300 runes); replace two overlay files, preserve $($baseline.Count) other files; close game before -Apply.";return}
Closed
$backup=Join-Path $root ('backups/spirit-mend-shop-raya-'+(Get-Date -Format 'yyyyMMdd-HHmmss'))
New-Item -ItemType Directory -Path $backup | Out-Null
foreach($r in $rows){Copy-Item -LiteralPath $r.target -Destination (Join-Path $backup $r.name);if((Hash (Join-Path $backup $r.name)) -ne $r.before){throw 'Backup mismatch'};Copy-Item -LiteralPath $r.source -Destination (Join-Path $backup ($r.name+'.new'));if((Hash (Join-Path $backup ($r.name+'.new'))) -ne $r.after){throw 'Prepared payload mismatch'}}
[ordered]@{version=$m.version;rows=$rows;protected=$baseline;gameplayTested=$false} | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $backup 'installation.json') -Encoding utf8
@'
param([switch]$Apply)
$ErrorActionPreference='Stop'
$r=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'installation.json') -Raw | ConvertFrom-Json
if(Get-Process eldenring,ersc_launcher,modengine2_launcher -ErrorAction SilentlyContinue){throw 'Close game and launchers.'}
foreach($f in $r.rows){if((Get-FileHash -LiteralPath $f.target).Hash -ne $f.after){throw 'Installed assets changed; review rollback.'};if((Get-FileHash -LiteralPath (Join-Path $PSScriptRoot $f.name)).Hash -ne $f.before){throw 'Backup changed.'}}
if(-not $Apply){'Preview: restore previous regulation and dialogue bundle; other files unchanged.';return}
foreach($f in $r.rows){Copy-Item -LiteralPath (Join-Path $PSScriptRoot $f.name) -Destination $f.target;if((Get-FileHash -LiteralPath $f.target).Hash -ne $f.before){throw 'Rollback readback failed.'}}
'Previous shop assets restored.'
'@ | Set-Content -LiteralPath (Join-Path $backup 'Restore.ps1') -Encoding utf8
Closed
foreach($r in $rows){if((Hash $r.target) -ne $r.before){throw 'Installed files changed during preparation'}}
$changed=@()
try {
 foreach($r in $rows){[IO.File]::Replace((Join-Path $backup ($r.name+'.new')),$r.target,(Join-Path $backup ($r.name+'.atomic-old')));$changed+=,$r;if((Hash $r.target) -ne $r.after){throw 'Installed readback mismatch'}}
 foreach($f in $baseline){if((Hash $f.path) -ne $f.sha256){throw "Protected file changed: $($f.path)"}}
} catch {
 foreach($r in $changed){Copy-Item -LiteralPath (Join-Path $backup $r.name) -Destination $r.target;if((Hash $r.target) -ne $r.before){throw "Rollback failed for $($r.name). Backup: $backup"}}
 throw
}
"Installed shop migration; two assets updated, $($baseline.Count) other files unchanged. Backup: $backup"
