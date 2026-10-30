$ErrorActionPreference = 'Stop'
$root = (Resolve-Path 'outputs\SpiritAshResummon').Path
$dist = Join-Path $root 'dist'
$stage = Join-Path $root 'pending-0.4.2'
$expectedOld = '71596B937B72A5A15EFAB5EBB734E6616A4DA6996467BCD8372211B60B7E52CE'
if ((Get-FileHash -LiteralPath (Join-Path $dist 'SpiritAshResummon.dll') -Algorithm SHA256).Hash -ne $expectedOld) { throw 'Installed DLL changed; refusing overwrite' }
if (Test-Path -LiteralPath $stage) { throw 'Stage exists; preserve it and review' }
New-Item -ItemType Directory -Path $stage | Out-Null
$build = (Resolve-Path 'work\prototype-build\x86_64-pc-windows-msvc\release').Path
foreach ($name in @('SpiritAshResummon.dll','SpiritAshResummon.pdb')) {
    Copy-Item -LiteralPath (Join-Path $build $name) -Destination $stage
}
foreach ($name in @('README.md','IMPLEMENTATION.md','VALIDATION.md','CHANGELOG.md','LICENSE-MIT','LICENSE-APACHE','THIRD-PARTY-NOTICES.txt','UPSTREAM.md','START-HERE.txt')) {
    Copy-Item -LiteralPath (Join-Path $root $name) -Destination $stage
}
$ini = [IO.File]::ReadAllText((Join-Path $dist 'SpiritAshResummon.ini'))
if ($ini -match '(?im)^EnableFriendlySpiritLockOn\s*=|^\[Targeting\]') { throw 'Targeting config already present; review it' }
if ($ini -notmatch '(?im)^EnableTargetDiagnostics\s*=\s*true' -or $ini -notmatch '(?im)^EnableLogging\s*=\s*true') { throw 'Expected target diagnostic logging enabled' }
$ini = $ini.TrimEnd() + "`r`n`r`n[Targeting]`r`nEnableFriendlySpiritLockOn = true`r`n"
[IO.File]::WriteAllText((Join-Path $stage 'SpiritAshResummon.ini'), $ini, [Text.UTF8Encoding]::new($false))
$newHash = (Get-FileHash -LiteralPath (Join-Path $stage 'SpiritAshResummon.dll') -Algorithm SHA256).Hash
$payload = @(Get-ChildItem -LiteralPath $stage -File | ForEach-Object { [ordered]@{name=$_.Name;sha256=(Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash;bytes=$_.Length} })
[ordered]@{version='0.4.2';timestamp=(Get-Date -Format o);staged=$stage;stagedDllSHA256=$newHash;priorInstalledDllSHA256=$expectedOld;compileSucceeded=$true;automatedTestsExecuted=$false;gameplayTestsExecuted=$false;configurationChange='Targeting.EnableFriendlySpiritLockOn=true; all other settings retained';files=$payload} | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $root 'evidence\staged-0.4.2.json') -Encoding utf8
if (Get-Process -Name eldenring,start_protected_game -ErrorAction SilentlyContinue) { throw 'Game running; 0.4.2 staged only, no install' }
$backup = Join-Path $root ('evidence\installed-0.4.1-before-0.4.2-' + (Get-Date -Format yyyyMMdd-HHmmss))
New-Item -ItemType Directory -Path $backup | Out-Null
Get-ChildItem -LiteralPath $dist -File | Copy-Item -Destination $backup
foreach ($f in Get-ChildItem -LiteralPath $dist -File) {
    if ((Get-FileHash -LiteralPath $f.FullName).Hash -ne (Get-FileHash -LiteralPath (Join-Path $backup $f.Name)).Hash) { throw "Backup mismatch: $($f.Name)" }
}
$logHash = (Get-FileHash -LiteralPath (Join-Path $dist 'SpiritAshResummon.log')).Hash
if (Get-Process -Name eldenring,start_protected_game -ErrorAction SilentlyContinue) { throw 'Game started during preparation; no install' }
foreach ($f in $payload) { Copy-Item -LiteralPath (Join-Path $stage $f.name) -Destination (Join-Path $dist $f.name) }
foreach ($f in $payload) {
    if ((Get-FileHash -LiteralPath (Join-Path $dist $f.name)).Hash -ne $f.sha256) { throw "Installed hash mismatch: $($f.name)" }
}
if ((Get-FileHash -LiteralPath (Join-Path $dist 'SpiritAshResummon.log')).Hash -ne $logHash) { throw 'Log changed during installation' }
[ordered]@{version='0.4.2';timestamp=(Get-Date -Format o);installed=$dist;backup=$backup;priorDllSHA256=$expectedOld;newDllSHA256=$newHash;configurationChange='Targeting.EnableFriendlySpiritLockOn=true; all other settings retained';logPreserved=$true;installedFilesVerified=$payload.Count;gameLaunched=$false;automatedTestsExecuted=$false;gameplayTestsExecuted=$false;files=$payload} | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $root 'evidence\install-0.4.2.json') -Encoding utf8
Write-Output "Installed 0.4.2; SHA256=$newHash; verified files=$($payload.Count)"
Write-Output "Backup=$backup"
Get-Content -LiteralPath (Join-Path $dist 'SpiritAshResummon.ini')
