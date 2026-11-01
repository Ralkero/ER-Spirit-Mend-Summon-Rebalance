// SPDX-License-Identifier: GPL-3.0-or-later
// Offline shop-row and dialogue migration. Input assets, regulation and saves are read-only.
using SoulsFormats;
using SoulsFormats.Cryptography;
using System.Security.Cryptography;
using System.Text.Json;

if(args.Length != 6) throw new ArgumentException("Usage: SpiritMendShopMove <game-dir> <Paramdex-ER> <current-regulation> <current-m60-talk> <vanilla-m60-talk> <new-output-dir>");
string Hash(string p)=>Convert.ToHexString(SHA256.HashData(File.ReadAllBytes(p)));
byte[] B(string s)=>Convert.FromHexString(s);
byte[] Int(int n)=>new byte[]{0x82}.Concat(BitConverter.GetBytes(n)).Append((byte)0xa1).ToArray();
void Require(bool ok,string message){if(!ok)throw new InvalidDataException(message);}
var game=Path.GetFullPath(args[0]);var defs=Path.GetFullPath(args[1]);var regulation=Path.GetFullPath(args[2]);
var current=Path.GetFullPath(args[3]);var vanilla=Path.GetFullPath(args[4]);var output=Path.GetFullPath(args[5]);
Require(!Directory.Exists(output)&&!File.Exists(output),"Output must be new.");
Require(Hash(current)=="28B7B31185D6FACD4DB694D3EDF648663E99001D30FFEBFD846C520E8EA9B394","Current dialogue differs from reviewed solo overlay.");
Require(Hash(vanilla)=="13919FD1A3F1E45EF32E9B198DEA05555FF7932DF98954BA436EC6AC8DB01818","Vanilla dialogue differs from reviewed extraction.");
Require(Hash(regulation)=="A4CDE09BA41258CA766D15510FA50943034DD333336451596EC6455457AE63B5","Expected reviewed Clever/Terra/Spirit Mend regulation.");
var inputs=new[]{regulation,current,vanilla}.ToDictionary(p=>p,Hash);
Oodle.SearchPaths.Add(game);
var reg=RegulationDecryptor.DecryptERRegulation(regulation);
Require(reg.Version=="11701000","Unexpected parameter version.");
var shop=PARAM.Read(reg.Files.Single(f=>Path.GetFileNameWithoutExtension(f.Name)=="ShopLineupParam").Bytes);
Require(shop.ApplyRegulationVersionedParamdefCarefully(PARAMDEF.XmlDeserialize(Path.Combine(defs,"Defs","ShopLineupParam.xml"),true),ulong.Parse(reg.Version)),"Shop schema mismatch.");
var spell=shop.Rows.Single(r=>r.ID==100599);
Require(Convert.ToInt32(spell["equipId"].Value)==990001 && Convert.ToInt32(spell["value"].Value)==300,"Existing Spirit Mend listing differs.");
var bundle=BND4.Read(current);var before=BND4.Read(current);var original=BND4.Read(vanilla);
BinderFile FileByName(BND4 b,string name)=>b.Files.Single(f=>Path.GetFileNameWithoutExtension(f.Name)==name);
bool Args(ESD.CommandCall c,int bank,int id,params byte[][] args)=>c.CommandBank==bank&&c.CommandID==id&&c.Arguments.Count==args.Length&&c.Arguments.Zip(args).All(p=>p.First.SequenceEqual(p.Second));
bool Condition(ESD.Condition c,long target,string eval)=>c.TargetState==target&&c.Evaluator.SequenceEqual(B(eval))&&c.PassCommands.Count==0&&c.Subconditions.Count==0;

// Undo precisely the previous additive Kale route, retaining all native dialogue.
var kaleFile=FileByName(bundle,"t800006000");var kale=ESD.Read(kaleFile.Bytes);
var kg=kale.StateGroups[2147483596];
Require(kg[3].EntryCommands.Count==10&&kg[5].Conditions.Count==7&&kg.ContainsKey(32),"Kale menu shape differs.");
Require(Args(kg[3].EntryCommands[^2],1,19,B("46A1"),Int(23300000),B("3FA1")),"Kale added choice differs.");
Require(Condition(kg[5].Conditions[1],32,"AF4695A1"),"Kale added selection differs.");
Require(kg[32].EntryCommands.Count==2&&Args(kg[32].EntryCommands[0],1,22,Int(100599),Int(100599))&&Args(kg[32].EntryCommands[1],1,141,B("45A1"))&&kg[32].Conditions.Count==1&&Condition(kg[32].Conditions[0],24,"41A1"),"Kale shop route differs.");
kg.Remove(32);kg[5].Conditions.RemoveAt(1);kg[3].EntryCommands.RemoveAt(kg[3].EntryCommands.Count-2);
Require(kale.Write().SequenceEqual(ESD.Read(FileByName(original,"t800006000").Bytes).Write()),"Removing Kale route did not reproduce original dialogue.");
kaleFile.Bytes=kale.Write();

// This merchant's root passes Academy shop range 100675..100699 into the
// merchant flow, which forwards those two args into group 2147483603.
var merchantFile=FileByName(bundle,"t801176000");var merchant=ESD.Read(merchantFile.Bytes);
Require(merchantFile.Bytes.SequenceEqual(FileByName(original,"t801176000").Bytes),"Merchant already modified.");
var root=merchant.StateGroups[1000][3].EntryCommands.Single();
Require(root.CommandBank==6&&root.CommandID==2147483610&&root.Arguments.Count==4&&root.Arguments[1].SequenceEqual(Int(100675))&&root.Arguments[2].SequenceEqual(Int(100699)),"Academy merchant identity/range differs.");
Require(Args(merchant.StateGroups[2147483610][2].EntryCommands.Single(),6,2147483603,B("41B8A1"),B("42B8A1")),"Merchant argument forwarding differs.");
var mg=merchant.StateGroups[2147483603];
Require(mg[6].EntryCommands.Count==1&&Args(mg[6].EntryCommands[0],1,22,B("40B8A1"),B("41B8A1")),"Merchant Purchase shop arguments differ.");
// Reuse a vacant slot INSIDE this merchant's native Purchase range. No new menu
// entry or alteration of the destination merchant script is necessary.
Require(!shop.Rows.Any(r=>r.ID==100699),"Destination shop slot is occupied; refuse to overwrite.");
var moved=new PARAM.Row(spell){ID=100699};shop.Rows.Remove(spell);shop.Rows.Add(moved);shop.Rows.Sort((a,b)=>a.ID.CompareTo(b.ID));
reg.Files.Single(f=>Path.GetFileNameWithoutExtension(f.Name)=="ShopLineupParam").Bytes=shop.Write();

var staging=output+".building-"+Guid.NewGuid().ToString("N");Directory.CreateDirectory(staging);
var outDir=Path.Combine(staging,"mod","script","talk");Directory.CreateDirectory(outDir);
var outFile=Path.Combine(outDir,Path.GetFileName(current));bundle.Write(outFile);
var decoded=BND4.Read(outFile);
Require(decoded.Files.Count==before.Files.Count&&decoded.Version==before.Version,"Binder metadata changed.");
int preserved=0;
foreach(var prior in before.Files){
 var final=decoded.Files.Single(f=>f.ID==prior.ID&&f.Name==prior.Name);Require(final.Flags==prior.Flags,"Binder flags changed.");
 if(prior.ID==kaleFile.ID){Require(final.Bytes.SequenceEqual(kaleFile.Bytes),"Kale readback differs.");continue;}
 Require(prior.Bytes.SequenceEqual(final.Bytes),"Unrelated NPC script changed: "+prior.Name);preserved++;
}
// Reopen regulation and compare every unrelated binder entry and every original
// shop row/field; only the row identity 100599 -> 100699 is authorized.
var regulationOut=Path.Combine(staging,"mod","regulation.bin");RegulationDecryptor.EncryptERRegulation(regulationOut,reg);
var checkReg=RegulationDecryptor.DecryptERRegulation(regulationOut);var originalReg=RegulationDecryptor.DecryptERRegulation(regulation);
Require(checkReg.Version==originalReg.Version&&checkReg.Files.Count==originalReg.Files.Count,"Regulation binder metadata changed.");
int preservedMembers=0,preservedShopRows=0;
bool Equal(object a,object b)=>a is byte[] x&&b is byte[] y?x.SequenceEqual(y):a.Equals(b);
PARAM ReadShop(byte[] data){var p=PARAM.Read(data);Require(p.ApplyRegulationVersionedParamdefCarefully(PARAMDEF.XmlDeserialize(Path.Combine(defs,"Defs","ShopLineupParam.xml"),true),ulong.Parse(reg.Version)),"Readback schema mismatch.");return p;}
foreach(var prior in originalReg.Files){
 var final=checkReg.Files.Single(f=>f.ID==prior.ID&&f.Name==prior.Name);Require(final.Flags==prior.Flags,"Regulation member flags changed.");
 if(Path.GetFileNameWithoutExtension(prior.Name)!="ShopLineupParam"){Require(prior.Bytes.SequenceEqual(final.Bytes),"Unrelated regulation member changed.");preservedMembers++;continue;}
 var oldRows=ReadShop(prior.Bytes).Rows;var newRows=ReadShop(final.Bytes).Rows.ToDictionary(r=>r.ID);
 Require(oldRows.Count==newRows.Count&&!newRows.ContainsKey(100599)&&newRows.ContainsKey(100699),"Shop row relocation failed.");
 foreach(var old in oldRows){var now=newRows[old.ID==100599?100699:old.ID];Require(old.Name==now.Name&&old.Cells.Count==now.Cells.Count,"Shop row metadata changed.");
  for(int i=0;i<old.Cells.Count;i++)Require(old.Cells[i].Def.InternalName==now.Cells[i].Def.InternalName&&Equal(old.Cells[i].Value,now.Cells[i].Value),$"Shop field changed: {old.ID}/{old.Cells[i].Def.InternalName}");
  preservedShopRows++;
 }
}
foreach(var (path,hash) in inputs)Require(Hash(path)==hash,"Input changed during build.");
var report=new{version="shop-raya-lucaria-2",status="STRUCTURAL_READBACK_VERIFIED_GAMEPLAY_PENDING",inputs,
 sourceTalkSha256=inputs[current],talkSha256=Hash(outFile),unchangedNpcScripts=preserved,changedScripts=new[]{"t800006000"},
 oldShopRow=100599,shopRow=100699,spellId=990001,price=300,stock=spell["sellQuantity"].Value,stockFlag=spell["eventFlag_forStock"].Value,
 merchant="Isolated Merchant - Raya Lucaria Academy bridge",menu="Purchase",regulationChanged=true,regulationSha256=Hash(regulationOut),unchangedRegulationMembers=preservedMembers,shopRowsFieldsPreserved=preservedShopRows};
File.WriteAllText(Path.Combine(staging,"manifest.json"),JsonSerializer.Serialize(report,new JsonSerializerOptions{WriteIndented=true}));
Directory.Move(staging,output);
Console.WriteLine($"Moved Spirit Mend to Purchase; {preserved} NPC scripts and {preservedMembers} regulation members byte-identical; {preservedShopRows} shop rows field-preserved. Output: {output}");
