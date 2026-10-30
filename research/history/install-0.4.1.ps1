$ErrorActionPreference = 'Stop'
$root = (Resolve-Path 'outputs\SpiritAshResummon').Path
$dist = Join-Path $root 'dist'
$stage = Join-Path $root 'pending-0.4.1'
$expectedOld = '3B92A2B3C7DCD4DAE5C2441F8E266333445BF2A2429E2F469AF69763FE472DF6'
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
if ($ini -match '(?im)^EnableTargetDiagnostics\s*=') { throw 'Diagnostic config already present; review it' }
if ([regex]::Matches($ini, '(?m)^EnableLogging\s*=\s*true\s*$').Count -ne 1) { throw 'Expected diagnostic logging enabled' }
$ini = [regex]::Replace($ini, '(?m)^EnableLogging\s*=\s*true[^\S\r\n]*', "EnableLogging = true`r`nEnableTargetDiagnostics = true")
[IO.File]::WriteAllText((Join-Path $stage 'SpiritAshResummon.ini'), $ini, [Text.UTF8Encoding]::new($false))
$newHash = (Get-FileHash -LiteralPath (Join-Path $stage 'SpiritAshResummon.dll') -Algorithm SHA256).Hash
$payload = @(Get-ChildItem -LiteralPath $stage -File | ForEach-Object { [ordered]@{name=$_.Name;sha256=(Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash;bytes=$_.Length} })
[ordered]@{version='0.4.1';timestamp=(Get-Date -Format o);staged=$stage;stagedDllSHA256=$newHash;priorInstalledDllSHA256=$expectedOld;compileSucceeded=$true;automatedTestsExecuted=$false;gameplayTestsExecuted=$false;configurationChange='EnableTargetDiagnostics=true; all other settings retained';files=$payload} | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $root 'evidence\staged-0.4.1.json') -Encoding utf8
if (Get-Process -Name eldenring,start_protected_game -ErrorAction SilentlyContinue) { throw 'Game running; 0.4.1 staged only, no install' }
$backup = Join-Path $root ('evidence\installed-0.4.0-before-0.4.1-' + (Get-Date -Format yyyyMMdd-HHmmss))
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
[ordered]@{version='0.4.1';timestamp=(Get-Date -Format o);installed=$dist;backup=$backup;priorDllSHA256=$expectedOld;newDllSHA256=$newHash;configurationChange='EnableTargetDiagnostics=true; all other settings retained';logPreserved=$true;installedFilesVerified=$payload.Count;gameLaunched=$false;automatedTestsExecuted=$false;gameplayTestsExecuted=$false;files=$payload} | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $root 'evidence\install-0.4.1.json') -Encoding utf8
Write-Output "Installed 0.4.1; SHA256=$newHash; verified files=$($payload.Count)"
Write-Output "Backup=$backup"
Get-Content -LiteralPath (Join-Path $dist 'SpiritAshResummon.ini')
