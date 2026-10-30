$ErrorActionPreference='Stop'
$root='E:\Elden Ring Unpacked Files'
$game='C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game'
$selection=Get-Content -LiteralPath "$root\_extraction-records\selected-files.txt"
foreach($relative in $selection) {
 if(Test-Path -LiteralPath ($root+$relative.Replace('/','\'))) { throw "Destination exists: $relative" }
}
$before=(Get-FileHash -LiteralPath "$game\eldenring.exe").Hash
Add-Type -TypeDefinition @'
using System;
using System.IO;
using System.Reflection;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
public class UxmProgress : IProgress<ValueTuple<double,string>> {
 public void Report(ValueTuple<double,string> p) { Console.WriteLine(p.Item2); }
}
public static class UxmExtract {
 public static void Run(string exe,string game,string selection) {
  var a=Assembly.LoadFrom(exe);
  var t=a.GetType("UXM.ArchiveUnpacker",true);
  t.GetMethod("SetSkip").Invoke(null,new object[]{true});
  var m=t.GetMethod("UnpackArchive",BindingFlags.NonPublic|BindingFlags.Static);
  var kind=Enum.Parse(m.GetParameters()[5].ParameterType,"EldenRing");
  var dict=Activator.CreateInstance(a.GetType("UXM.ArchiveDictionary",true),new object[]{File.ReadAllText(selection),kind});
  var keys=(Dictionary<string,string>)a.GetType("UXM.ArchiveKeys",true).GetField("EldenRingKeys").GetValue(null);
  string[] archives={"Data0","Data1","Data2","Data3","DLC"};
  for(int i=0;i<archives.Length;i++) {
   var task=(Task<string>)m.Invoke(null,new object[]{game,archives[i],keys[archives[i]],i,archives.Length,kind,dict,new UxmProgress(),CancellationToken.None});
   var error=task.GetAwaiter().GetResult();
   if(error!=null) throw new Exception(error);
  }
 }
}
'@
$links=@()
try {
 foreach($name in @('menu','msg')) {
  $link=Join-Path $game $name
  if(Test-Path -LiteralPath $link) { throw "Existing game path: $link" }
  $target=Join-Path $root $name
  New-Item -ItemType Directory -Path $target -Force | Out-Null
  New-Item -ItemType Junction -Path $link -Target $target | Out-Null
  $links+=$link
 }
 [UxmExtract]::Run("$root\_tools\UXM-2.4.2\UXM Selective Unpack.exe",$game,"$root\_extraction-records\selected-files.txt")
 $manifest=foreach($relative in $selection) {
  $p=$root+$relative.Replace('/','\')
  $f=Get-Item -LiteralPath $p -ErrorAction Stop
  [pscustomobject]@{path=$relative;bytes=$f.Length;sha256=(Get-FileHash -LiteralPath $p).Hash}
 }
 $manifest | ConvertTo-Json | Set-Content -LiteralPath "$root\_extraction-records\files.json"
} finally {
 foreach($link in $links) {
  $entry=Get-Item -LiteralPath $link -Force
  if($entry.LinkType -ne 'Junction' -or @($entry.Target)[0] -ne (Join-Path $root $entry.Name)) { throw "Unexpected link target: $link" }
  # Delete only the verified junction itself, never recurse into the destination.
  [IO.Directory]::Delete($link)
 }
 $after=(Get-FileHash -LiteralPath "$game\eldenring.exe").Hash
 [pscustomobject]@{before=$before;after=$after;unchanged=($before -eq $after);junctionsRemoved=(@($links|Where-Object {Test-Path -LiteralPath $_}).Count -eq 0)} | ConvertTo-Json | Set-Content -LiteralPath "$root\_extraction-records\game-preservation.json"
 if($before -ne $after) {throw 'Game executable hash changed'}
}
'Extraction finished.'
