// SPDX-License-Identifier: GPL-3.0-or-later
// Offline asset tooling only; never linked into the Rust runtime DLL.
using SoulsFormats;
using SoulsFormats.Cryptography;
using System.Text.Json;
using System.Security.Cryptography;

if (args.Length != 3 && args.Length != 4) throw new ArgumentException("Usage: SpiritMendAssets <game-directory> <Paramdex-ER-directory> <new-output-directory> [extracted-engus-message-directory-to-build-assets]");
var game = Path.GetFullPath(args[0]); var defs = Path.Combine(Path.GetFullPath(args[1]), "Defs"); var output = Path.GetFullPath(args[2]);
if (Directory.Exists(output)) throw new IOException("Output already exists; use a new directory.");
Directory.CreateDirectory(output);
Oodle.SearchPaths.Add(game);
var source = Path.Combine(game, "regulation.bin");
var bnd = RegulationDecryptor.DecryptERRegulation(source);
var rows = new Dictionary<string, object>();
var tables = new Dictionary<string,(PARAM Param, BinderFile File)>();
foreach (var request in new[] { ("Magic", "MagicParam", new[]{4010,2006400,6420}), ("EquipParamGoods", "EquipParamGoods", new[]{4010,2006400,6420}), ("Bullet", "BulletParam", new[]{10401000,210640000,210640001,10642000}), ("SpEffectParam", "SpEffect", new[]{21640000,21640001,1642000,1642001}), ("AtkParam_Pc", "AtkParam", new[]{40100,210640001,64200}), ("ShopLineupParam", "ShopLineupParam", new[]{100500,100507}) }) {
 var file = bnd.Files.Single(f=>Path.GetFileNameWithoutExtension(f.Name)==request.Item1);
 var param = PARAM.Read(file.Bytes); Console.WriteLine(request.Item1+" size="+param.DetectedSize+" regulation="+bnd.Version); var xml = System.Xml.Linq.XDocument.Load(Path.Combine(defs,request.Item2+".xml"));
 if(request.Item1 == "SpEffectParam") {
  // Preserve the three unnamed high bits at byte 0x353. Upstream currently
  // names only five low bits; strict SoulsFormats correctly rejects data loss.
  var field=xml.Descendants("Field").Single(e=>(string?)e.Attribute("Def")=="u8 unk353_4:1");
  field.AddAfterSelf(new System.Xml.Linq.XElement("Field",new System.Xml.Linq.XAttribute("Def","dummy8 preserved353High:3")));
 }
 using var schema = new MemoryStream(); xml.Save(schema); schema.Position=0;
 var def = PARAMDEF.XmlDeserialize(schema,true);
 if(!param.ApplyRegulationVersionedParamdefCarefully(def,ulong.Parse(bnd.Version))) throw new Exception("Paramdef mismatch "+request.Item1);
 if (request.Item1=="SpEffectParam") {
 var candidates=param.Rows.Where(r=>Convert.ToSingle(r["changeHpRate"].Value)<0).Select(r=>new {id=r.ID,fields=r.Cells.Where(c=>new[]{"changeHpRate","changeHpPoint","effectEndurance","motionInterval","stateInfo","spCategory","effectTargetFriendlyTarget","effectTargetSelfTarget"}.Contains(c.Def.InternalName)).ToDictionary(c=>c.Def.InternalName,c=>c.Value)}).ToArray();
 File.WriteAllText(Path.Combine(output,"percent-healing-effects.json"),JsonSerializer.Serialize(candidates,new JsonSerializerOptions{WriteIndented=true}));
 }
 tables.Add(request.Item1,(param,file));
 rows[request.Item1] = request.Item3.Select(id=> {var row=param.Rows.Single(r=>r.ID==id);return new {id,row.Name,fields=row.Cells.ToDictionary(c=>c.Def.InternalName,c=>c.Value)};}).ToArray();
 File.WriteAllText(Path.Combine(output,"partial-rows.json"),JsonSerializer.Serialize(rows,new JsonSerializerOptions{WriteIndented=true}));
}
File.WriteAllText(Path.Combine(output,"template-rows.json"),JsonSerializer.Serialize(rows,new JsonSerializerOptions{WriteIndented=true}));
File.WriteAllText(Path.Combine(output,"source.json"),JsonSerializer.Serialize(new {source,sha256=Convert.ToHexString(SHA256.HashData(File.ReadAllBytes(source))),regulationVersion=bnd.Version},new JsonSerializerOptions{WriteIndented=true}));
Console.WriteLine("Extracted template rows to "+output+"; regulation version "+bnd.Version);

var textRows = new List<object>();
foreach (var path in (args.Length==4 ? Directory.GetFiles(Path.GetFullPath(args[3]), "*.msgbnd.dcx") : Array.Empty<string>())) {
 var bundle=BND4.Read(path);
 foreach(var file in bundle.Files) {
  var fmg=FMG.Read(file.Bytes);var entry=fmg.Entries.FirstOrDefault(e=>e.ID==4010);
  if(entry!=null) textRows.Add(new {bundle=Path.GetFileName(path),file=file.Name,fileId=file.ID,text=entry.Text});
 }
}
File.WriteAllText(Path.Combine(output,"template-text.json"),JsonSerializer.Serialize(textRows,new JsonSerializerOptions{WriteIndented=true}));

if (args.Length==4) {
 const int spell=990001, bullet=99000100, attack=99000100, marker=99000100, heal=99000101, shop=100521;
 var changes=new List<object>();
 PARAM.Row Clone(string table,int original,int id,Dictionary<string,object> edits) {
  var param=tables[table].Param;
  if(param.Rows.Any(r=>r.ID==id)) throw new InvalidDataException($"ID collision: {table}/{id}");
  var row=new PARAM.Row(param.Rows.Single(r=>r.ID==original)){ID=id,Name="Spirit Mend prototype"};
  foreach(var (key,value) in edits) row[key].Value=value;
  param.Rows.Add(row);param.Rows.Sort((a,b)=>a.ID.CompareTo(b.ID));
  changes.Add(new {table,original,id,edits});return row;
 }
 Clone("Magic",4010,spell,new(){["refId1"]=bullet,["sortId"]=4011,["enable_multi"]=0,["enable_gray"]=0,["enable_white"]=0,["enable_black"]=0});
 Clone("EquipParamGoods",4010,spell,new(){["sortId"]=250011,["maxNum"]=1,["maxRepositoryNum"]=1});
 Clone("Bullet",10401000,bullet,new(){["atkId_Bullet"]=attack,["isHitBothTeam"]=1,["spEffectId0"]=marker});
 Clone("AtkParam_Pc",210640001,attack,new(){["friendlyTarget"]=1,["selfTarget"]=0,["opposeTarget"]=0,["spEffectAtkPowerCorrectRate_byRate"]=100});
 var effectEdits=new Dictionary<string,object>{["changeHpPoint"]=0,["changeHpRate"]=0f,["stateInfo"]=0,["effectEndurance"]=0f,["motionInterval"]=0f,["effectTargetSelfTarget"]=0,["effectTargetOpposeTarget"]=0,["effectTargetFriendlyTarget"]=1,["effectTargetPlayer"]=0,["effectTargetAI"]=1};
 Clone("SpEffectParam",21640001,marker,effectEdits);
 var healingEdits=new Dictionary<string,object>(effectEdits){["changeHpRate"]=-25f};
 Clone("SpEffectParam",21640001,heal,healingEdits);
 Clone("ShopLineupParam",100507,shop,new(){["equipId"]=spell,["value"]=300,["eventFlag_forStock"]=0,["eventFlag_forRelease"]=0,["sellQuantity"]=-1,["nameMsgId"]=-1});
 var mod=Path.Combine(output,"mod");Directory.CreateDirectory(mod);
 foreach(var pair in tables.Values) pair.File.Bytes=pair.Param.Write();
 RegulationDecryptor.EncryptERRegulation(Path.Combine(mod,"regulation.bin"),bnd);
 var texts=new Dictionary<int,string>{[10]="Spirit Mend",[20]="Fires a shard that restores a summoned Spirit's HP on impact",[24]="A glintstone sorcery adapted to mend summoned Spirits.\n\nFires a swift shard that restores 25% of a living Spirit's maximum HP on impact. Requires a sorcery catalyst.\n\nPrototype: keep the intended Spirit locked on until impact. Uses the normal casting cost even if the projectile misses."};
 var msgOut=Path.Combine(mod,"msg","engus");Directory.CreateDirectory(msgOut);
 foreach(var path in Directory.GetFiles(Path.GetFullPath(args[3]),"*.msgbnd.dcx")) {
  var bundle=BND4.Read(path);
  foreach(var file in bundle.Files.Where(f=>texts.ContainsKey(f.ID))) {
   var fmg=FMG.Read(file.Bytes);
   if(!fmg.Entries.Any(e=>e.ID==4010)) throw new InvalidDataException("Missing original text: "+file.Name);
   if(fmg.Entries.Any(e=>e.ID==spell)) throw new InvalidDataException("Text ID collision: "+file.Name);
   fmg.Entries.Add(new FMG.Entry(spell,texts[file.ID]));fmg.Entries.Sort((a,b)=>a.ID.CompareTo(b.ID));file.Bytes=fmg.Write();
  }
  bundle.Write(Path.Combine(msgOut,Path.GetFileName(path)));
 }
 File.WriteAllText(Path.Combine(output,"patch-manifest.json"),JsonSerializer.Serialize(new {schema=1,spell,bullet,attack,marker,heal,shop,changes,
    sourceRegulation=source,sourceSha256=Convert.ToHexString(SHA256.HashData(File.ReadAllBytes(source))),regulationVersion=bnd.Version,
    files=Directory.GetFiles(mod,"*",SearchOption.AllDirectories).Select(f=>new {path=Path.GetRelativePath(mod,f),sha256=Convert.ToHexString(SHA256.HashData(File.ReadAllBytes(f)))})
 },new JsonSerializerOptions{WriteIndented=true}));
 Console.WriteLine("Built isolated Spirit Mend assets; originals untouched: "+mod);
}
