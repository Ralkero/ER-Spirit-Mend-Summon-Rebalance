import pefile,pathlib,capstone,collections
pe=pefile.PE(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe',fast_load=True); base=pe.OPTIONAL_HEADER.ImageBase
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64); md.detail=True
seeds=[0x493e30,0x493dc0,0x493e20,0x493de0,0x493d30,0x493d90,0x493d50,0x493da0,0x495970,0x494870,0x495b80,0x4961b0,0x496340,0x495f20,0x3c3830]
q=collections.deque(seeds); visited={}; edges=[]
while q and len(visited)<20000:
 r=q.popleft()
 if r in visited: continue
 try: x=next(md.disasm(pe.get_data(r,15),base+r,count=1))
 except: continue
 visited[r]=x
 if x.mnemonic=='call' and x.operands[0].type==capstone.CS_OP_IMM:
  t=x.operands[0].imm-base; edges.append((r,t))
  if 0x493000<=t<0x497500 or t==0x3c3830:q.append(t)
 if x.group(capstone.CS_GRP_JUMP):
  if x.operands[0].type==capstone.CS_OP_IMM:
   t=x.operands[0].imm-base
   if 0x493000<=t<0x497500 or 0x3c3830<=t<0x3c3880:q.append(t)
  if x.mnemonic=='jmp':continue
 if x.group(capstone.CS_GRP_RET) or x.mnemonic in ['int3','ud2']:continue
 q.append(r+x.size)
lines=[f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for r,x in sorted(visited.items())]
pathlib.Path('work/registry_cfg.asm').write_text('\n'.join(lines))
print('instructions',len(lines))
print('\n'.join(l for l in lines if ('byte ptr' in l and ('+ 8]' in l or '+ 9]' in l))))
