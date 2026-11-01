// SPDX-License-Identifier: GPL-3.0-or-later
// Reviewed additive merge only. No game-root writes; no save access.
using SoulsFormats;
using SoulsFormats.Cryptography;
using System.Security.Cryptography;
using System.Text.Json;
using System.Xml.Linq;

if (args.Length != 8) throw new ArgumentException("Usage: SpiritMendMerge <game-dir> <Paramdex-ER> <target-regulation> <donor-mod-dir> <vanilla-engus> <current-engus> <new-output-dir> <vanilla-m60-talk-bundle>");
string Hash(string p) => Convert.ToHexString(SHA256.HashData(File.ReadAllBytes(p)));
var game=Path.GetFullPath(args[0]); var defs=Path.Combine(Path.GetFullPath(args[1]),"Defs");
var target=Path.GetFullPath(args[2]); var donorRoot=Path.GetFullPath(args[3]);
var vanillaMessages=Path.GetFullPath(args[4]); var currentMessages=Path.GetFullPath(args[5]);
var output=Path.GetFullPath(args[6]);
var talkSource=Path.GetFullPath(args[7]);
if (Directory.Exists(output) || File.Exists(output)) throw new IOException("Output must be new.");
var donorPath=Path.Combine(donorRoot,"regulation.bin");
if(Hash(target)!="EFE8A382C5424DB926E1E27393EE533D3F094372AFBB91F694655146F2B532E3") throw new InvalidDataException("Target is not the reviewed Clever + Terra overlay.");
if(Hash(donorPath)!="75F7C3B041992EB6499D5B31690E14398485F9C4954A036A72F4676DCE73B447") throw new InvalidDataException("Donor is not the reviewed Spirit Mend asset version.");
var inputs=new Dictionary<string,string>{{target,Hash(target)},{donorPath,Hash(donorPath)}};
if(Hash(talkSource)!="13919FD1A3F1E45EF32E9B198DEA05555FF7932DF98954BA436EC6AC8DB01818") throw new InvalidDataException("Unreviewed talk bundle.");
inputs.Add(talkSource,Hash(talkSource));
Oodle.SearchPaths.Add(game);
var before=RegulationDecryptor.DecryptERRegulation(target);
var after=RegulationDecryptor.DecryptERRegulation(target);
var donor=RegulationDecryptor.DecryptERRegulation(donorPath);
if(before.Version!="11701000" || donor.Version!="11730034") throw new InvalidDataException("Unexpected regulation version.");
var edits=new Dictionary<string,(string Schema,int[] IDs)> {
 ["Magic"]=("MagicParam",[990001]), ["EquipParamGoods"]=("EquipParamGoods",[990001]),
 ["Bullet"]=("BulletParam",[99000100]), ["AtkParam_Pc"]=("AtkParam",[99000100]),
 ["SpEffectParam"]=("SpEffect",[99000100,99000101]), ["ShopLineupParam"]=("ShopLineupParam",[100599])
};
BinderFile Member(BND4 b,string table)=>b.Files.Single(f=>Path.GetFileNameWithoutExtension(f.Name)==table);
PARAM Read(BND4 b,string table) {
 var p=PARAM.Read(Member(b,table).Bytes); var xml=XDocument.Load(Path.Combine(defs,edits[table].Schema+".xml"));
 if(table=="SpEffectParam") xml.Descendants("Field").Single(e=>(string?)e.Attribute("Def")=="u8 unk353_4:1")
  .AddAfterSelf(new XElement("Field",new XAttribute("Def","dummy8 preserved353High:3")));
 using var stream=new MemoryStream();xml.Save(stream);stream.Position=0;
 var definition=PARAMDEF.XmlDeserialize(stream,true);
 if(!p.ApplyRegulationVersionedParamdefCarefully(definition,ulong.Parse(b.Version))) throw new InvalidDataException("Schema mismatch: "+table);
 return p;
}
bool Equal(object a,object b)=>a is byte[] x && b is byte[] y ? x.SequenceEqual(y) : a.Equals(b);
void EqualRow(PARAM.Row a,PARAM.Row b,string table) {
 if(a.ID!=b.ID || a.Name!=b.Name || a.Cells.Count!=b.Cells.Count) throw new InvalidDataException($"Row metadata changed: {table}/{a.ID}");
 for(int i=0;i<a.Cells.Count;i++) if(a.Cells[i].Def.InternalName!=b.Cells[i].Def.InternalName || !Equal(a.Cells[i].Value,b.Cells[i].Value))
  throw new InvalidDataException($"Field changed: {table}/{a.ID}/{a.Cells[i].Def.InternalName}");
}
var imported=new Dictionary<string,List<PARAM.Row>>();
foreach(var (table,edit) in edits) {
 var p=Read(after,table);var q=Read(donor,table);var clones=new List<PARAM.Row>();
 if(p.DetectedSize!=q.DetectedSize) throw new InvalidDataException("Row size differs: "+table);
 var targetFields=p.Rows[0].Cells;var donorFields=q.Rows[0].Cells;
 if(!targetFields.Select(c=>(c.Def.InternalName,c.Value.GetType())).SequenceEqual(donorFields.Select(c=>(c.Def.InternalName,c.Value.GetType()))))
  throw new InvalidDataException("Filtered field layouts differ: "+table);
 foreach(int id in edit.IDs) {
  if(p.Rows.Any(r=>r.ID==id)) throw new InvalidDataException($"Row collision: {table}/{id}");
  var row=new PARAM.Row(q.Rows.Single(r=>r.ID==(table=="ShopLineupParam"?100521:id))){ID=id};p.Rows.Add(row);clones.Add(row);
 }
 imported.Add(table,clones);p.Rows.Sort((a,b)=>a.ID.CompareTo(b.ID));Member(after,table).Bytes=p.Write();
}
// All guards above run before output creation. Failed serialization leaves only
// a uniquely named staging directory, never a partially published overlay.
var staging=output+".building-"+Guid.NewGuid().ToString("N");Directory.CreateDirectory(staging);
var mod=Path.Combine(staging,"mod");Directory.CreateDirectory(mod);
var regulationOut=Path.Combine(mod,"regulation.bin");RegulationDecryptor.EncryptERRegulation(regulationOut,after);
var decoded=RegulationDecryptor.DecryptERRegulation(regulationOut);
if(decoded.Version!=before.Version || decoded.Files.Count!=before.Files.Count) throw new InvalidDataException("Binder metadata mismatch.");
int untouched=0,originalRows=0,addedRows=0;
foreach(var original in before.Files) {
 var final=decoded.Files.Single(f=>f.ID==original.ID && f.Name==original.Name);
 if(final.Flags!=original.Flags) throw new InvalidDataException("Binder member flags changed.");
 var table=Path.GetFileNameWithoutExtension(original.Name);
 if(!edits.ContainsKey(table)) { if(!original.Bytes.SequenceEqual(final.Bytes)) throw new InvalidDataException("Unrelated member changed: "+table);untouched++;continue; }
 var p=Read(before,table);var q=Read(decoded,table);var finalRows=q.Rows.ToDictionary(r=>r.ID);
 if(q.Rows.Count!=p.Rows.Count+edits[table].IDs.Length) throw new InvalidDataException("Row count mismatch.");
 foreach(var row in p.Rows) { EqualRow(row,finalRows[row.ID],table);originalRows++; }
 foreach(var row in imported[table]) { EqualRow(row,finalRows[row.ID],table);addedRows++; }
}
var textReport=new List<object>();var messageOut=Path.Combine(mod,"msg","engus");Directory.CreateDirectory(messageOut);
var donorHashes=new Dictionary<string,string>{
 ["item.msgbnd.dcx"]="DC2E8ED565FF97CAC6F3CD835DC3E902C1D47872C9B5387F9F1B4051C6139373",
 ["item_dlc01.msgbnd.dcx"]="A591079F8C7D4DC1456A7BB7E852CAC18B5FC33AB453C3BF12793D8F175A2D77",
 ["item_dlc02.msgbnd.dcx"]="9334382EF5C725842755032B9EF645FB8901C0C8322DC3EDE60B095C2EC25AE1"
};
foreach(var name in donorHashes.Keys) {
 var source=Path.Combine(currentMessages,name);if(!File.Exists(source)) source=Path.Combine(vanillaMessages,name);
 var sourceDonor=Path.Combine(donorRoot,"msg","engus",name);
 if(Hash(sourceDonor)!=donorHashes[name]) throw new InvalidDataException("Unexpected message donor: "+name);
 if(name=="item_dlc02.msgbnd.dcx" && Hash(source)!="26F4C0687843EAF3776AB0EA1A7B6A75F7C56FFED269FC95307E32BE15ACC5C9")
  throw new InvalidDataException("Expected reviewed Clever message overlay.");
 inputs.Add(source,Hash(source));inputs.Add(sourceDonor,Hash(sourceDonor));
 var bundle=BND4.Read(source);var originalBundle=BND4.Read(source);var donorBundle=BND4.Read(sourceDonor);
 var touched=new HashSet<int>();
 foreach(var id in new[]{10,20,24}) {
  var f=bundle.Files.Single(f=>f.ID==id);var text=FMG.Read(f.Bytes);
  var donorText=FMG.Read(donorBundle.Files.Single(f=>f.ID==id).Bytes).Entries.Single(e=>e.ID==990001).Text;
  if(text.Entries.Any(e=>e.ID==990001)) throw new InvalidDataException("Message collision: "+name+"/"+id);
  text.Entries.Add(new FMG.Entry(990001,donorText));text.Entries.Sort((a,b)=>a.ID.CompareTo(b.ID));f.Bytes=text.Write();touched.Add(id);
 }
 var fileOut=Path.Combine(messageOut,name);bundle.Write(fileOut);var check=BND4.Read(fileOut);
 if(check.Files.Count!=originalBundle.Files.Count) throw new InvalidDataException("Message member count changed.");
 int entriesPreserved=0;
 foreach(var f in originalBundle.Files) {
  var z=check.Files.Single(c=>c.ID==f.ID && c.Name==f.Name);
  if(f.Flags!=z.Flags) throw new InvalidDataException("Message member flags changed.");
  if(!touched.Contains(f.ID)) { if(!f.Bytes.SequenceEqual(z.Bytes)) throw new InvalidDataException("Unrelated message member changed.");continue; }
  var p=FMG.Read(f.Bytes);var q=FMG.Read(z.Bytes);
  if(q.Entries.Count!=p.Entries.Count+1) throw new InvalidDataException("Message entry count mismatch.");
  var existing=q.Entries.Where(e=>e.ID!=990001).Select(e=>(e.ID,e.Text)).ToArray();
  if(!p.Entries.Select(e=>(e.ID,e.Text)).OrderBy(e=>e.ID).SequenceEqual(existing.OrderBy(e=>e.ID))) throw new InvalidDataException("Existing message changed.");
  var expected=FMG.Read(donorBundle.Files.Single(d=>d.ID==f.ID).Bytes).Entries.Single(e=>e.ID==990001).Text;
  if(q.Entries.Single(e=>e.ID==990001).Text!=expected) throw new InvalidDataException("Imported text mismatch.");
  entriesPreserved+=p.Entries.Count;
 }
 textReport.Add(new{name,sourceSha256=Hash(source),outputSha256=Hash(fileOut),entriesPreserved,addedEntries=3});
}
// All existing Kale inventory slots are occupied. Add a separate native shop
// choice rather than replacing a Clever item or expanding its regular range.
var talkBundle=BND4.Read(talkSource);var originalTalk=BND4.Read(talkSource);
var kale=talkBundle.Files.Single(f=>Path.GetFileNameWithoutExtension(f.Name)=="t800006000");
var esd=ESD.Read(kale.Bytes);const long group=2147483596;
var states=esd.StateGroups[group];
byte[] B(string hex)=>Convert.FromHexString(hex);
byte[] Int(int n)=>new byte[]{0x82}.Concat(BitConverter.GetBytes(n)).Append((byte)0xa1).ToArray();
var regular=states[10].EntryCommands[0];
if(regular.CommandBank!=1||regular.CommandID!=22||regular.Arguments.Count!=2||
 !regular.Arguments[0].SequenceEqual(Int(100500))||!regular.Arguments[1].SequenceEqual(Int(100524))||states.ContainsKey(32))
 throw new InvalidDataException("Kale shop layout differs.");
var menu=states[3].EntryCommands;
if(menu.Count!=9 || states[5].Conditions.Count!=6 || !states[5].Conditions[1].Evaluator.SequenceEqual(B("AF4295A1")))
 throw new InvalidDataException("Kale selection layout differs.");
// Choice 6 is unused in this exact source. Message 23300000 is existing
// EventTextForTalk 'Study sorcery', verified in the current English PS5 bundle.
menu.Insert(menu.Count-1,new ESD.CommandCall(1,19,B("46A1"),Int(23300000),B("3FA1")));
states[5].Conditions.Insert(1,new ESD.Condition(32,B("AF4695A1")));
var newState=new ESD.State();
foreach(var command in states[10].EntryCommands)
 newState.EntryCommands.Add(new ESD.CommandCall(command.CommandBank,command.CommandID,command.Arguments.Select(a=>a.ToArray()).ToArray()));
newState.EntryCommands[0].Arguments=new List<byte[]>{Int(100599),Int(100599)};
newState.Conditions.Add(new ESD.Condition(24,B("41A1")));states.Add(32,newState);
kale.Bytes=esd.Write();
var talkOutDir=Path.Combine(mod,"script","talk");Directory.CreateDirectory(talkOutDir);
var talkOut=Path.Combine(talkOutDir,Path.GetFileName(talkSource));talkBundle.Write(talkOut);
var talkCheck=BND4.Read(talkOut);
if(talkCheck.Files.Count!=originalTalk.Files.Count) throw new InvalidDataException("Talk binder count changed.");
foreach(var f in originalTalk.Files) {
 var z=talkCheck.Files.Single(z=>z.ID==f.ID&&z.Name==f.Name);
 if(f.Flags!=z.Flags) throw new InvalidDataException("Talk member metadata changed.");
 if(f.ID!=kale.ID) { if(!f.Bytes.SequenceEqual(z.Bytes)) throw new InvalidDataException("Unrelated NPC script changed.");continue; }
 // Independently decode and undo exactly the three additive edits. Canonical
 // serialization must then equal the untouched original script in full.
 var checkEsd=ESD.Read(z.Bytes);var g=checkEsd.StateGroups[group];
 if(!g[32].EntryCommands[0].Arguments.All(a=>a.SequenceEqual(Int(100599))) ||
    g[5].Conditions[1].TargetState!=32 || !g[3].EntryCommands[^2].Arguments[1].SequenceEqual(Int(23300000)))
  throw new InvalidDataException("New shop route differs.");
 g.Remove(32);g[5].Conditions.RemoveAt(1);g[3].EntryCommands.RemoveAt(g[3].EntryCommands.Count-2);
 if(!checkEsd.Write().SequenceEqual(ESD.Read(f.Bytes).Write())) throw new InvalidDataException("Original Kale script changed beyond additive menu route.");
}
foreach(var (input,hash) in inputs) if(Hash(input)!=hash) throw new IOException("Input changed during merge.");
var report=new{status="STATIC_VERIFIED_GAMEPLAY_UNTESTED",targetVersion=before.Version,donorVersion=donor.Version,
 untouchedBinderMembers=untouched,originalRowsPreserved=originalRows,addedRows,messages=textReport,inputs,
 shopRow=100599,kaleMenu="Study sorcery",talkSha256=Hash(talkOut),unmodifiedTalkMembers=talkCheck.Files.Count-1,
 outputRegulationSha256=Hash(regulationOut)};
File.WriteAllText(Path.Combine(staging,"merge-report.json"),JsonSerializer.Serialize(report,new JsonSerializerOptions{WriteIndented=true}));
Directory.Move(staging,output);
Console.WriteLine($"Merged {addedRows} rows; preserved {originalRows} original rows and {untouched} untouched binder members. Output: {output}");
