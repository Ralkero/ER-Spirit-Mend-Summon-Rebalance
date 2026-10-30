from pathlib import Path
import struct,json
b=Path(r'E:\Elden Ring Unpacked Files\menu\01_000_fe.gfx').read_bytes()
class Bits:
 def __init__(self,b,p):self.b=b;self.p=p*8
 def u(self,n):
  v=0
  for _ in range(n):v=(v<<1)|((self.b[self.p//8]>>(7-self.p%8))&1);self.p+=1
  return v
 def s(self,n):
  v=self.u(n);return v-(1<<n) if n and v&(1<<(n-1)) else v
 def end(self):return (self.p+7)//8
def matrix(d,p):
 q=Bits(d,p);scale=[1,1];rot=None
 if q.u(1):n=q.u(5);scale=[q.s(n)/65536,q.s(n)/65536]
 if q.u(1):n=q.u(5);rot=[q.s(n),q.s(n)]
 n=q.u(5);xy=[q.s(n)/20,q.s(n)/20]
 return {'scale':scale,'xy':xy,'rotation':rot},q.end()
def tags(d,p):
 while p+2<=len(d):
  h=int.from_bytes(d[p:p+2],'little');p+=2;n=h&63;c=h>>6
  if n==63:n=int.from_bytes(d[p:p+4],'little');p+=4
  v=d[p:p+n];p+=n;yield c,v
  if c==0:return
def place(d):
 f=d[0];p=3;r={'depth':int.from_bytes(d[1:3],'little')}
 if f&2:r['character']=int.from_bytes(d[p:p+2],'little');p+=2
 if f&4:r['matrix'],p=matrix(d,p)
 if f&8:
  q=Bits(d,p);a=q.u(1);m=q.u(1);n=q.u(4)
  if m:r['color_mult']=[q.s(n) for _ in range(4)]
  if a:r['color_add']=[q.s(n) for _ in range(4)]
  p=q.end()
 if f&16:p+=2
 if f&32:r['name']=d[p:d.index(0,p)].decode(errors='replace')
 return r
header=8+(5+4*(b[8]>>3)+7)//8+4
alltags=list(tags(b,header));rows=[]
for c,d in alltags:
 if c==26:
  r=place(d)
  if r.get('name')=='Buddy' or r.get('name','').startswith('RemotePCTag'):rows.append(r)
ids={r.get('character') for r in rows};children={}
for c,d in alltags:
 if c==39 and int.from_bytes(d[:2],'little') in ids:
  children[str(int.from_bytes(d[:2],'little'))]=[place(v) for t,v in tags(d,4) if t==26]
out={'file':'E:/Elden Ring Unpacked Files/menu/01_000_fe.gfx','placements':rows,'sprite_children':children,'static_analysis_only':True}
Path('outputs/SpiritAshResummon/evidence/hud-layout-analysis.json').write_text(json.dumps(out,indent=2))
print(json.dumps({'placements':rows,'children':{k:[x.get('name') for x in v] for k,v in children.items()}},indent=2))
