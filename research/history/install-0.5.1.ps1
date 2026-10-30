$ErrorActionPreference='Stop'
$root=(Resolve-Path 'outputs/SpiritAshResummon').Path
$dist=Join-Path $root 'dist'
$stage=Join-Path $root 'pending-0.5.1-install'
$build=(Resolve-Path 'work/prototype-build/x86_64-pc-windows-msvc/release').Path
function Hash($p) { (Get-FileHash -LiteralPath $p -Algorithm SHA256).Hash }
function AssertClosed { if(Get-Process -Name eldenring,start_protected_game -ErrorAction SilentlyContinue) { throw 'Game is still running; installation deferred.' } }
AssertClosed
$oldDll='7E242FA3077450362C84858D2F4021654A82293D4AD00F79948C1A8110F0D9AA'
if((Hash (Join-Path $dist 'SpiritAshResummon.dll')) -ne $oldDll) { throw 'Unexpected installed DLL; refusing overwrite.' }
$patch=Get-Content (Join-Path $dist 'patch-manifest.json') -Raw | ConvertFrom-Json
if($patch.shop -ne 100521 -or $patch.spell -ne 990001) { throw 'Corrected shop assets are not installed.' }
if(Test-Path -LiteralPath $stage) { throw 'Stage already exists; inspect before replacing.' }
New-Item -ItemType Directory -Path (Join-Path $stage 'dist'),(Join-Path $stage 'tools') | Out-Null
foreach($name in @('SpiritAshResummon.dll','SpiritAshResummon.pdb')) { Copy-Item -LiteralPath (Join-Path $build $name) -Destination (Join-Path $stage 'dist') }
foreach($name in @('README.md','IMPLEMENTATION.md','VALIDATION.md','CHANGELOG.md','SPIRIT-MEND.md','START-HERE.txt','UPSTREAM.md','THIRD-PARTY-NOTICES.txt','LICENSE-MIT','LICENSE-APACHE')) { Copy-Item -LiteralPath (Join-Path $root $name) -Destination (Join-Path $stage 'dist') }
foreach($name in @('Launch-Test.ps1','Launch-SpiritMend.ps1')) {
 $path=Join-Path $root "tools/$name"
 # Only the displayed release label changes; preserve existing mod/profile paths.
 $new=[IO.File]::ReadAllText($path).Replace('0.5.0','0.5.1')
 [IO.File]::WriteAllText((Join-Path $stage "tools/$name"),$new)
}
$protected=@(Get-ChildItem -LiteralPath (Join-Path $dist 'mod') -File -Recurse | ForEach-Object { [ordered]@{path=$_.FullName;sha256=(Hash $_.FullName)} })
foreach($path in @((Join-Path $dist 'SpiritAshResummon.ini'),(Join-Path $dist 'SpiritAshResummon.log'),(Join-Path $dist 'patch-manifest.json'),(Join-Path $root 'config_spiritash_test.toml'),$patch.sourceRegulation)) {
 $protected+= [ordered]@{path=$path;sha256=(Hash $path)}
}
$backup=Join-Path $root ('evidence/installed-0.5.0-before-0.5.1-'+(Get-Date -Format yyyyMMdd-HHmmss))
New-Item -ItemType Directory -Path $backup,(Join-Path $backup 'tools') | Out-Null
Copy-Item -LiteralPath $dist -Destination (Join-Path $backup 'dist') -Recurse
foreach($name in @('Launch-Test.ps1','Launch-SpiritMend.ps1')) { Copy-Item -LiteralPath (Join-Path $root "tools/$name") -Destination (Join-Path $backup 'tools') }
Copy-Item -LiteralPath (Join-Path $root 'config_spiritash_test.toml') -Destination $backup
$backupFiles=@(Get-ChildItem -LiteralPath $backup -File -Recurse)
foreach($f in $backupFiles) {
 $rel=[IO.Path]::GetRelativePath($backup,$f.FullName)
 if((Hash $f.FullName) -ne (Hash (Join-Path $root $rel))) { throw "Backup mismatch: $rel" }
}
$payload=@(Get-ChildItem -LiteralPath $stage -File -Recurse | ForEach-Object { [ordered]@{path=[IO.Path]::GetRelativePath($stage,$_.FullName);sha256=(Hash $_.FullName);bytes=$_.Length} })
AssertClosed
if((Hash (Join-Path $dist 'SpiritAshResummon.dll')) -ne $oldDll) { throw 'Installed DLL changed during preparation.' }
foreach($f in $protected) { if((Hash $f.path) -ne $f.sha256) { throw "Protected file changed: $($f.path)" } }
try {
 foreach($f in $payload) { Copy-Item -LiteralPath (Join-Path $stage $f.path) -Destination (Join-Path $root $f.path) }
 foreach($f in $payload) { if((Hash (Join-Path $root $f.path)) -ne $f.sha256) { throw "Installed mismatch: $($f.path)" } }
 foreach($f in $protected) { if((Hash $f.path) -ne $f.sha256) { throw "Protected file changed: $($f.path)" } }
} catch {
 foreach($f in $payload) {
  $prior=Join-Path $backup $f.path
  if(Test-Path -LiteralPath $prior) { Copy-Item -LiteralPath $prior -Destination (Join-Path $root $f.path) }
 }
 throw
}
$sources=@('src/mend_cast.rs','src/spirit_mend.rs','Cargo.toml','Cargo.lock','vendor/fromsoftware-rs/crates/eldenring/src/cs/bullet_manager.rs','vendor/fromsoftware-rs/crates/eldenring/src/cs/bullet_ins.rs') | ForEach-Object { [ordered]@{path=$_;sha256=(Hash (Join-Path $root $_))} }
[ordered]@{
 version='0.5.1';timestamp=(Get-Date -Format o);installed=$dist;backup=$backup
 priorDllSHA256=$oldDll;newDllSHA256=(Hash (Join-Path $dist 'SpiritAshResummon.dll'))
 installedFilesVerified=$payload.Count;backupFilesVerified=$backupFiles.Count;protectedFilesVerified=$protected.Count
 gameClosedBeforeInstall=$true;gameLaunched=$false;automatedTestsExecuted=$false;gameplayTestsExecuted=$false
 assetChange='none; corrected shop100521 retained';configChange='none';logPreserved=$true
 files=$payload;protected=$protected;sources=$sources
} | ConvertTo-Json -Depth 8 | Set-Content (Join-Path $root 'evidence/install-0.5.1.json') -Encoding utf8
Write-Output "Installed 0.5.1; verified $($payload.Count) files; protected $($protected.Count) files; backup=$backup"
Write-Output "DLL SHA256=$(Hash (Join-Path $dist 'SpiritAshResummon.dll'))"
