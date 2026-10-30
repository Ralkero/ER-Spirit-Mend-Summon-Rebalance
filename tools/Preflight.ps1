param([string]$GameExe = 'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe')
$ErrorActionPreference = 'Stop'
$file = Get-Item -LiteralPath $GameExe
$bytes = [IO.File]::ReadAllBytes($file.FullName)
$pe = [BitConverter]::ToInt32($bytes,0x3c)
if ([Text.Encoding]::ASCII.GetString($bytes,$pe,4) -ne "PE`0`0") { throw 'Not a PE executable' }
$count = [BitConverter]::ToUInt16($bytes,$pe+6)
$optional = [BitConverter]::ToUInt16($bytes,$pe+20)
$sections = for ($i=0;$i -lt $count;$i++) {
    $at=$pe+24+$optional+40*$i
    [pscustomobject]@{ RVA=[BitConverter]::ToUInt32($bytes,$at+12); Size=[BitConverter]::ToUInt32($bytes,$at+16); Offset=[BitConverter]::ToUInt32($bytes,$at+20) }
}
function Get-Offset([uint32]$Rva) {
    foreach ($s in $sections) { if ($Rva -ge $s.RVA -and $Rva -lt $s.RVA+$s.Size) { return [int]($s.Offset+$Rva-$s.RVA) } }
    throw ('Unmapped RVA {0:x}' -f $Rva)
}
$specs=@(
    @(0x772fbd,0x775480),
    @(0x7c4154,0x4b73d0),@(0x847ed0,0x4b73d0),@(0x848194,0x4b73d0),
    @(0x69063a,0x4b73d0),@(0x6906dd,0x4b73d0),@(0x3c0b79,0x4b73d0),@(0x3c0c2f,0x4b73d0),@(0x3fb268,0x4b8690),@(0x4b86a9,0x4b73d0),@(0x4b8a90,0x4bba80),@(0x690833,0x4b73d0)
)
$calls=foreach ($spec in $specs) {
    $at=Get-Offset $spec[0]
    $target=[int64]$spec[0]+5+[BitConverter]::ToInt32($bytes,$at+1)
    [pscustomobject]@{rva=('{0:x}' -f $spec[0]); expectedTarget=('{0:x}' -f $spec[1]); observedTarget=('{0:x}' -f $target); matches=($bytes[$at] -eq 0xe8 -and $target -eq $spec[1])}
}
$signatures=@(
    @{rva=0x775480; hex='488bc45556574154415541564157488d'},
    @{rva=0xd6e970; hex='4055535657415441564157488d6c24d9'},
    @{rva=0x2671370; hex='3b51107329443b411473'},
    @{rva=0x3f4740; hex='488b4158488b88c80000008b412483e001c3'},
    @{rva=0x4b7ca5; hex='e896caf3ff84c07505384331'},
    @{rva=0x690830; hex='418bd5e8986be2ff83f8ff410f95c5'},
    @{rva=0x4b73d0; hex='48895c24084889742410'},
    @{rva=0x4b8690; hex='48895c24084889742410574883ec20'},
    @{rva=0x4b86ae; hex='83f801761b83f8027504c6432801'},
    @{rva=0x49560a; hex='488b0d6f8d8f03488b01ba0005000041b808000000ff5050488946184885c0744abf50000000897e1033d241b800050000488bc8e84dac080233db6666660f1f840000000000488b4e184803cb48894c245848894c2460740833d2e8a6e1f2ff904883c3104883ef0175db4883c4305f5e5bc3'},
    @{rva=0x493492; hex='83c0028d0480448d1400'},
    @{rva=0x4934c0; hex='4183f90a7d0a438d3c11498d1c12eb0e418d7af64103f9498d5af64803da4803d948c1e3044903d84c393b741441ffc148ffc14883f90a7cc7'},
    @{rva=0x4934f9; hex='498bc7e99d010000'},
    @{rva=0x4b8a90; hex='e8eb2f00004183bf'},
    @{rva=0x3f2d42; hex='80780804753a'},
    @{rva=0x3f8ce2; hex='488b4310c6400804'},
    @{rva=0x401439; hex='488b4610c6400904'},
    @{rva=0x3c3810; hex='80610af8488bc148891166c741080000c3'}
)
$checks=foreach($sig in $signatures) {
    $at=Get-Offset $sig.rva
    $actual=([BitConverter]::ToString($bytes,$at,($sig.hex.Length/2))).Replace('-','').ToLowerInvariant()
    [pscustomobject]@{rva=('{0:x}' -f $sig.rva);matches=($actual -eq $sig.hex);observed=$actual}
}
$versionOK=$file.VersionInfo.FileVersion -eq '2.7.2.0'
[pscustomobject]@{
    path=$file.FullName; fileVersion=$file.VersionInfo.FileVersion
    sha256=(Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    supportedVersion=$versionOK; calls=$calls; signatures=$checks
    staticPreflightPassed=($versionOK -and @($calls|Where-Object {-not $_.matches}).Count -eq 0 -and @($checks|Where-Object {-not $_.matches}).Count -eq 0)
    gameExecuted=$false; runtimeHooksTested=$false
} | ConvertTo-Json -Depth 6
if (-not $versionOK -or @($calls|Where-Object {-not $_.matches}).Count -gt 0 -or @($checks|Where-Object {-not $_.matches}).Count -gt 0) {exit 1}

