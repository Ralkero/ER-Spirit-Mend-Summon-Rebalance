$ErrorActionPreference='Stop'
$root=(Resolve-Path 'outputs/SpiritAshResummon').Path
$dist=Join-Path $root 'dist'
$stage=Join-Path $root 'development-0.5.0-shopfix'
$old=Get-Content (Join-Path $dist 'patch-manifest.json') -Raw | ConvertFrom-Json
$new=Get-Content (Join-Path $stage 'patch-manifest.json') -Raw | ConvertFrom-Json
function Hash($p) {(Get-FileHash -LiteralPath $p -Algorithm SHA256).Hash}
if(Get-Process -Name eldenring,start_protected_game -ErrorAction SilentlyContinue) {throw 'Game still running; correction remains staged.'}
if($old.shop -ne 100599 -or $new.shop -ne 100521) {throw 'Unexpected shop manifest IDs.'}
foreach($f in $old.files) {if((Hash (Join-Path $dist ('mod/'+$f.path))) -ne $f.sha256) {throw "Installed asset changed: $($f.path)"}}
foreach($f in $new.files) {if((Hash (Join-Path $stage ('mod/'+$f.path))) -ne $f.sha256) {throw "Staged asset changed: $($f.path)"}}
$dllHash=Hash (Join-Path $dist 'SpiritAshResummon.dll')
$logHash=Hash (Join-Path $dist 'SpiritAshResummon.log')
$backup=Join-Path $root ('evidence/installed-0.5.0-before-shopfix-'+(Get-Date -Format yyyyMMdd-HHmmss))
New-Item -ItemType Directory $backup | Out-Null
Copy-Item -LiteralPath (Join-Path $dist 'mod') -Destination (Join-Path $backup 'mod') -Recurse
foreach($n in @('patch-manifest.json','SPIRIT-MEND.md','VALIDATION.md','CHANGELOG.md','SpiritAshResummon.log')) {Copy-Item -LiteralPath (Join-Path $dist $n) -Destination $backup}
foreach($f in Get-ChildItem $backup -File -Recurse) {
 $rel=[IO.Path]::GetRelativePath($backup,$f.FullName)
 if((Hash $f.FullName) -ne (Hash (Join-Path $dist $rel))) {throw "Backup mismatch: $rel"}
}
if(Get-Process -Name eldenring,start_protected_game -ErrorAction SilentlyContinue) {throw 'Game started during preparation; no replacement made.'}
foreach($f in $new.files) {Copy-Item -LiteralPath (Join-Path $stage ('mod/'+$f.path)) -Destination (Join-Path $dist ('mod/'+$f.path))}
Copy-Item -LiteralPath (Join-Path $stage 'patch-manifest.json') -Destination $dist
foreach($n in @('SPIRIT-MEND.md','VALIDATION.md','CHANGELOG.md')) {Copy-Item -LiteralPath (Join-Path $root $n) -Destination $dist}
foreach($f in $new.files) {if((Hash (Join-Path $dist ('mod/'+$f.path))) -ne $f.sha256) {throw 'Installed asset verification failed.'}}
if((Hash (Join-Path $dist 'SpiritAshResummon.dll')) -ne $dllHash -or (Hash (Join-Path $dist 'SpiritAshResummon.log')) -ne $logHash) {throw 'DLL/log changed during asset update.'}
[ordered]@{version='0.5.0-shopfix';timestamp=(Get-Date -Format o);backup=$backup;oldShopRow=$old.shop;newShopRow=$new.shop;dllUnchanged=$true;logPreserved=$true;files=$new.files;gameLaunched=$false;automatedTestsExecuted=$false;gameplayTestsExecuted=$false} | ConvertTo-Json -Depth 6 | Set-Content (Join-Path $root 'evidence/install-0.5.0-shopfix.json') -Encoding utf8
Write-Output "Installed asset-only correction: shop100521; backup=$backup"
