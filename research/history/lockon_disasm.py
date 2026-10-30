from pathlib import Path
import pefile,capstone,json
p=Path(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe');pe=pefile.PE(str(p),fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True;base=pe.OPTIONAL_HEADER.ImageBase
out=Path('outputs/SpiritAshResummon/evidence/lockon-static');out.mkdir(exist_ok=True)
for name,rva,size in [('score',0x718300,0x800),('switch',0x719a60,0xb00),('init',0x716500,0x200)]:
 s='\n'.join(f'{i.address-base:x}: {i.bytes.hex():24} {i.mnemonic} {i.op_str}' for i in md.disasm(pe.get_data(rva,size),base+rva));(out/f'{name}.asm').write_text(s)
print((out/'score.asm').read_text()[:22000])
