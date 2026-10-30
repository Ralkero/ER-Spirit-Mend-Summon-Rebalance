$ErrorActionPreference='Stop'
$root=(Resolve-Path 'outputs/SpiritAshResummon').Path
$dist=Join-Path $root 'dist'
$dev=Join-Path $root 'development-0.5.0'
$stage=Join-Path $root 'pending-0.5.0-install'
$manifest=Get-Content (Join-Path $root 'evidence/staged-0.5.0.json') -Raw | ConvertFrom-Json
$patch=Get-Content (Join-Path $dev 'patch-manifest.json') -Raw | ConvertFrom-Json
function Hash($p) { (Get-FileHash -LiteralPath $p -Algorithm SHA256).Hash }
function AssertClosed { if (Get-Process -Name eldenring,start_protected_game -ErrorAction SilentlyContinue) { throw 'Game running; stop installation.' } }
AssertClosed
if ((Hash (Join-Path $dist 'SpiritAshResummon.dll')) -ne $manifest.installedDllSha256) { throw 'Installed DLL changed.' }
foreach($f in $manifest.files) { if((Hash (Join-Path $dev $f.name)) -ne $f.sha256) { throw "Stage changed: $($f.name)" } }
if ((Hash $patch.sourceRegulation) -ne $patch.sourceSha256) { throw 'Source regulation changed.' }
if ((Test-Path $stage) -or (Test-Path (Join-Path $dist 'mod'))) { throw 'Unexpected existing install stage/assets.' }
New-Item -ItemType Directory -Path (Join-Path $stage 'dist'),(Join-Path $stage 'tools') | Out-Null
foreach($n in @('SpiritAshResummon.dll','SpiritAshResummon.pdb','patch-manifest.json')) { Copy-Item -LiteralPath (Join-Path $dev $n) -Destination (Join-Path $stage 'dist') }
Copy-Item -LiteralPath (Join-Path $dev 'mod') -Destination (Join-Path $stage 'dist/mod') -Recurse
$ini=[IO.File]::ReadAllText((Join-Path $dist 'SpiritAshResummon.ini'))
if($ini -match '(?im)^\[Sorcery\]|^EnableSpiritMend\s*=') { throw 'Unexpected Sorcery config.' }
$ini=$ini.TrimEnd()+"`r`n`r`n[Sorcery]`r`nEnableSpiritMend = true`r`n"
[IO.File]::WriteAllText((Join-Path $stage 'dist/SpiritAshResummon.ini'),$ini)
foreach($n in @('README.md','IMPLEMENTATION.md','VALIDATION.md','CHANGELOG.md','LICENSE-MIT','LICENSE-APACHE','THIRD-PARTY-NOTICES.txt','UPSTREAM.md','START-HERE.txt','SPIRIT-MEND.md')) { Copy-Item -LiteralPath (Join-Path $root $n) -Destination (Join-Path $stage 'dist') }
$launcher=[IO.File]::ReadAllText((Join-Path $root 'tools/Launch-Test.ps1'))
if($launcher -notmatch 'mods = \[\]') { throw 'Unexpected launcher contents.' }
$launcher=$launcher.Replace('$dllToml=$dll.Replace','$modToml=(Join-Path $root ''dist/mod'').Replace(''\'',''/'')'+"`r`n"+'$dllToml=$dll.Replace')
$launcher=$launcher.Replace('mods = []','mods = [{ enabled = true, name = "Spirit Mend prototype", path = "$modToml" }]')
$launcher=$launcher.Replace('foreach($path in @($Loader,$GameExe,$dll))','foreach($path in @($Loader,$GameExe,$dll,(Join-Path $root ''dist/mod/regulation.bin'')))')
$launcher=$launcher.Replace('Starting the clean SpiritAshResummon profile. Use offline mode and stop at the title screen for Stage 1.','Starting SpiritAshResummon 0.5.0 with Spirit Mend assets. Use the offline test character; see SPIRIT-MEND.md.')
[IO.File]::WriteAllText((Join-Path $stage 'tools/Launch-Test.ps1'),$launcher)
[IO.File]::WriteAllText((Join-Path $stage 'tools/Launch-SpiritMend.ps1'),'# Uses the installed 0.5.0 profile.'+"`r`n"+'& (Join-Path $PSScriptRoot ''Launch-Test.ps1'') @args'+"`r`n")
$profile=[IO.File]::ReadAllText((Join-Path $dev 'config_spirit_mend.toml')).Replace($dev.Replace('\','/'),$dist.Replace('\','/'))
[IO.File]::WriteAllText((Join-Path $stage 'config_spiritash_test.toml'),$profile)
foreach($n in @('Launch-Test.ps1','Launch-SpiritMend.ps1')) {
 $tokens=$null; $parseErrors=$null
 [System.Management.Automation.Language.Parser]::ParseFile((Join-Path $stage "tools/$n"),[ref]$tokens,[ref]$parseErrors) | Out-Null
 if($parseErrors.Count) { throw "Launcher syntax error: $parseErrors" }
}
$backup=Join-Path $root ('evidence/installed-0.4.2-before-0.5.0-'+(Get-Date -Format yyyyMMdd-HHmmss))
New-Item -ItemType Directory -Path $backup,(Join-Path $backup 'tools') | Out-Null
Copy-Item -LiteralPath $dist -Destination (Join-Path $backup 'dist') -Recurse
Copy-Item -LiteralPath (Join-Path $root 'tools/Launch-Test.ps1'),(Join-Path $root 'tools/Launch-SpiritMend.ps1') -Destination (Join-Path $backup 'tools')
Copy-Item -LiteralPath (Join-Path $root 'config_spiritash_test.toml') -Destination $backup
$oldFiles=@(Get-ChildItem -LiteralPath $backup -File -Recurse)
foreach($f in $oldFiles) {
 $rel=[IO.Path]::GetRelativePath($backup,$f.FullName)
 if((Hash $f.FullName) -ne (Hash (Join-Path $root $rel))) { throw "Backup mismatch: $rel" }
}
$logHash=Hash (Join-Path $dist 'SpiritAshResummon.log')
$payload=@(Get-ChildItem -LiteralPath $stage -File -Recurse | ForEach-Object { [ordered]@{path=[IO.Path]::GetRelativePath($stage,$_.FullName);sha256=(Hash $_.FullName);bytes=$_.Length} })
AssertClosed
if((Hash (Join-Path $dist 'SpiritAshResummon.dll')) -ne $manifest.installedDllSha256) { throw 'DLL changed during preparation.' }
foreach($f in $payload) {
 $destination=Join-Path $root $f.path
 New-Item -ItemType Directory -Path (Split-Path -Parent $destination) -Force | Out-Null
 Copy-Item -LiteralPath (Join-Path $stage $f.path) -Destination $destination
}
foreach($f in $payload) { if((Hash (Join-Path $root $f.path)) -ne $f.sha256) { throw "Install mismatch: $($f.path)" } }
if((Hash (Join-Path $dist 'SpiritAshResummon.log')) -ne $logHash) { throw 'Log changed.' }
if((Hash $patch.sourceRegulation) -ne $patch.sourceSha256) { throw 'Game regulation changed.' }
[ordered]@{version='0.5.0';timestamp=(Get-Date -Format o);installed=$dist;backup=$backup;newDllSHA256=$manifest.dllSha256;priorDllSHA256=$manifest.installedDllSha256;installedFilesVerified=$payload.Count;backupFilesVerified=$oldFiles.Count;logPreserved=$true;gameRegulationUnchanged=$true;gameLaunched=$false;automatedTestsExecuted=$false;gameplayTestsExecuted=$false;configurationChange='EnableSpiritMend=true; prior settings retained; usual launcher/profile includes dist/mod';files=$payload} | ConvertTo-Json -Depth 7 | Set-Content (Join-Path $root 'evidence/install-0.5.0.json') -Encoding utf8
Write-Output "Installed 0.5.0; verified $($payload.Count) files; backup=$backup"
Write-Output "DLL SHA256=$(Hash (Join-Path $dist 'SpiritAshResummon.dll'))"
