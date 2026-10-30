import pefile,pathlib,capstone
p=pathlib.Path(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe'); pe=pefile.PE(str(p),fast_load=True); pe.parse_data_directories(directories=[3]); base=pe.OPTIONAL_HEADER.ImageBase
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
lines=[]
for e in pe.DIRECTORY_ENTRY_EXCEPTION:
 s=e.struct.BeginAddress; end=e.struct.EndAddress
 if not 0x493a00<=s<0x497400: continue
 lines.append(f'FUNCTION {s:x}-{end:x}')
 for x in md.disasm(pe.get_data(s,end-s),base+s): lines.append(f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}')
pathlib.Path('work/registry_functions.asm').write_text('\n'.join(lines))
print('\n'.join(l for l in lines if l.startswith('FUNCTION') or ('byte ptr' in l and ('+ 8]' in l or '+ 9]' in l))))
