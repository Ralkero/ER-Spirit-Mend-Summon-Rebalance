param(
    [Parameter(Mandatory)][string]$ProfileRoot,
    [Parameter(Mandatory)][string]$PackageDirectory,
    [Parameter(Mandatory)][string]$GameExe,
    [Parameter(Mandatory)][string]$SeamlessDll,
    [switch]$Apply
)
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path -LiteralPath $ProfileRoot).Path
$package = (Resolve-Path -LiteralPath $PackageDirectory).Path
$config = Join-Path $root 'config_eldenring.toml'
$destination = Join-Path $root 'dll/spirit-mend-probe'
$entry = 'dll/spirit-mend-probe/SpiritMendCompatibilityProbe.dll'
function Hash([string]$Path) { (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash }
function RequireClosed {
    if (Get-Process eldenring,ersc_launcher,modengine2_launcher -ErrorAction SilentlyContinue) {
        throw 'Close Elden Ring and its launchers first.'
    }
}
RequireClosed
if ((Hash $GameExe) -ne 'F9118FEA2CFD8FA3901E8060BFADB1552CB2471771DED8A5B662C4E298FFB328') { throw 'Unsupported game executable.' }
if ((Hash $SeamlessDll) -ne '0A093D8490967BF19C62F895FDF1FDA21840282D2B2B371B9B4FC24A0CFE146E') { throw 'Unsupported Seamless binary.' }
$manifest = Get-Content -LiteralPath (Join-Path $package 'manifest.json') -Raw | ConvertFrom-Json
if ($manifest.kind -ne 'seamless-read-only-probe-1') { throw 'Not a diagnostic package.' }
foreach ($file in $manifest.files) {
    if ($file.name -notin @('SpiritMendCompatibilityProbe.dll','SEAMLESS-COMPATIBILITY.md')) { throw 'Unexpected package file.' }
    if ((Hash (Join-Path $package $file.name)) -ne $file.sha256) { throw "Package hash mismatch: $($file.name)" }
}
if (-not ($manifest.files | Where-Object name -eq 'SpiritMendCompatibilityProbe.dll')) { throw 'Missing DLL manifest entry.' }
$beforeHash = Hash $config
$before = [IO.File]::ReadAllText($config)
if ($before -match '(?i)SpiritAshResummon|SpiritMendCompatibilityProbe|MultipleSpiritAshes|NativeTweaks') { throw 'Profile already contains a gameplay conflict or probe.' }
if ($before -notmatch '(?i)ersc\.dll') { throw 'Seamless must remain configured.' }
$pattern = '(?m)^(external_dlls\s*=\s*\[)([^\r\n]*)(\])\s*$'
if ([regex]::Matches($before, $pattern).Count -ne 1) { throw 'Expected one single-line external_dlls array; no edits made.' }
$after = [regex]::Replace($before, $pattern, { param($match) $match.Groups[1].Value + $match.Groups[2].Value + ', "' + $entry + '"]' })
if (Test-Path -LiteralPath $destination) { throw 'Probe destination already exists; refusing overwrite.' }
# Guard current parameter, message, launcher and other DLL payloads. No saves read.
$protected = @(
    (Get-ChildItem -LiteralPath (Join-Path $root 'mod') -File -Recurse | Where-Object { $_.Name -eq 'regulation.bin' -or $_.Name -like '*.msgbnd.dcx' }).FullName
    (Get-ChildItem -LiteralPath (Join-Path $root 'dll') -File -Recurse | Where-Object Extension -eq '.dll').FullName
    (Join-Path $root 'launchmod_eldenring.bat')
    $SeamlessDll
    (Join-Path (Split-Path -Parent $SeamlessDll) 'ersc_settings.ini')
) | Where-Object { $_ }
$baseline = @($protected | ForEach-Object { [pscustomobject]@{path=$_; sha256=(Hash $_)} })
if (-not $Apply) { Write-Output "Preview: add only $entry to $config; protect $($baseline.Count) existing files. Use -Apply to install."; return }
RequireClosed
if ((Hash $config) -ne $beforeHash) { throw 'Profile changed during preparation.' }
$backup = Join-Path $root ('backups/spirit-mend-probe-' + (Get-Date -Format 'yyyyMMdd-HHmmss'))
New-Item -ItemType Directory -Path $backup -ErrorAction Stop | Out-Null
Copy-Item -LiteralPath $config -Destination (Join-Path $backup 'config_eldenring.toml')
if ((Hash (Join-Path $backup 'config_eldenring.toml')) -ne $beforeHash) { throw 'Backup verification failed.' }
$baseline | ConvertTo-Json -Depth 3 | Set-Content -LiteralPath (Join-Path $backup 'preservation-before.json') -Encoding utf8
New-Item -ItemType Directory -Path $destination -ErrorAction Stop | Out-Null
foreach ($file in $manifest.files) {
    $target = Join-Path $destination $file.name
    Copy-Item -LiteralPath (Join-Path $package $file.name) -Destination $target
    if ((Hash $target) -ne $file.sha256) { throw 'Installed payload hash mismatch; profile still unchanged.' }
}
$temporary = Join-Path $backup 'config.prepared.toml'
[IO.File]::WriteAllText($temporary, $after, [Text.UTF8Encoding]::new($false))
RequireClosed
if ((Hash $config) -ne $beforeHash) { throw 'Profile changed before commit.' }
[IO.File]::Replace($temporary, $config, (Join-Path $backup 'config.atomic-backup.toml'))
if ([IO.File]::ReadAllText($config) -ne $after) { throw 'Profile readback mismatch.' }
foreach ($file in $baseline) { if ((Hash $file.path) -ne $file.sha256) { throw "Protected file changed: $($file.path)" } }
$record = [ordered]@{
    kind=$manifest.kind; sourceCommit=$manifest.sourceCommit; installed=(Get-Date -Format o)
    config=$config; configBefore=$beforeHash; configAfter=(Hash $config)
    backup=$backup; destination=$destination; files=$manifest.files; protected=$baseline
    gameplayEnabled=$false; testsExecuted=$false
}
$record | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $backup 'installation.json') -Encoding utf8
@'
param([switch]$Apply)
$ErrorActionPreference='Stop'
$record=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'installation.json') -Raw | ConvertFrom-Json
if (Get-Process eldenring,ersc_launcher,modengine2_launcher -ErrorAction SilentlyContinue) { throw 'Close the game and launcher first.' }
if ((Get-FileHash -LiteralPath $record.config -Algorithm SHA256).Hash -ne $record.configAfter) { throw 'Profile changed since installation; review before restoring.' }
$source=Join-Path $PSScriptRoot 'config_eldenring.toml'
if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -ne $record.configBefore) { throw 'Backup hash mismatch.' }
if (-not $Apply) { Write-Output 'Preview: restore original profile configuration; keep disabled diagnostic DLL and log.'; return }
Copy-Item -LiteralPath $source -Destination $record.config
if ((Get-FileHash -LiteralPath $record.config -Algorithm SHA256).Hash -ne $record.configBefore) { throw 'Restore verification failed.' }
Write-Output 'Original profile restored. Diagnostic files retained but inactive.'
'@ | Set-Content -LiteralPath (Join-Path $backup 'Restore.ps1') -Encoding utf8
Write-Output "Installed diagnostic only. Verified $($baseline.Count) protected files unchanged. Record: $backup/installation.json"
