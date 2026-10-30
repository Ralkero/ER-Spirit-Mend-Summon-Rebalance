exec(open('work/registry_cfg.py').read().split('seeds=')[0])
for r in range(0x493000,0x497500):
 try:x=next(md.disasm(pe.get_data(r,15),base+r,count=1))
 except:continue
 if x.mnemonic in ['mov','cmp','movzx','test'] and any(o.type==capstone.CS_OP_MEM and o.mem.disp in (8,9) and o.size==1 and o.mem.base not in (capstone.x86.X86_REG_RSP,capstone.x86.X86_REG_RBP) for o in x.operands):print(hex(r),x.mnemonic,x.op_str)
