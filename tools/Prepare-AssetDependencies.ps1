param([string]$ExternalDirectory=(Join-Path $PSScriptRoot '../external'))
$ErrorActionPreference='Stop'
$external=[IO.Path]::GetFullPath($ExternalDirectory)
New-Item -ItemType Directory -Path $external -Force | Out-Null
$deps=@(
 @{name='SoulsFormatsNEXT';url='https://github.com/soulsmods/SoulsFormatsNEXT.git';commit='9e1c36f54962f80c143e3c1fe8a709204aefacf9'},
 @{name='Paramdex';url='https://github.com/soulsmods/Paramdex.git';commit='ff7245e524329bc3eab00036723d2bd53384cedf'}
)
foreach($dep in $deps) {
 $path=Join-Path $external $dep.name
 if(Test-Path -LiteralPath $path) { throw "Destination already exists: $path. Use a new ExternalDirectory; existing checkouts will not be reset." }
 & git clone --filter=blob:none --no-checkout $dep.url $path
 if($LASTEXITCODE -ne 0) { throw "Clone failed: $($dep.name)" }
 & git -C $path checkout --detach $dep.commit
 if($LASTEXITCODE -ne 0) { throw "Pinned checkout failed: $($dep.name)" }
 $head=(& git -C $path rev-parse HEAD).Trim()
 if($LASTEXITCODE -ne 0 -or $head -ne $dep.commit) { throw 'Pinned commit mismatch.' }
}
$patch=Join-Path $PSScriptRoot 'SpiritMendAssets/oodle-legacy-printf.patch'
& git -C (Join-Path $external 'SoulsFormatsNEXT') apply --check $patch
if($LASTEXITCODE -ne 0) { throw 'Oodle compatibility patch does not apply.' }
& git -C (Join-Path $external 'SoulsFormatsNEXT') apply $patch
if($LASTEXITCODE -ne 0) { throw 'Oodle compatibility patch failed.' }
Write-Host 'Pinned source dependencies prepared. No game files were accessed or tests executed.'
Write-Host 'If ExternalDirectory is not repository/external, pass -p:SoulsFormatsProject=<absolute project path> to dotnet build.'
