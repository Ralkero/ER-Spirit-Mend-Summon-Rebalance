from pathlib import Path
from datetime import datetime
import re
p=Path('outputs/SpiritAshResummon/dist/SpiritAshResummon.log');s=p.read_text(encoding='utf-8-sig');lines=s.splitlines()
a=Path('outputs/SpiritAshResummon/evidence')/('0.4.0-elleh-boundary-'+datetime.now().strftime('%Y%m%d-%H%M%S')+'.log');a.write_text(s,encoding='utf-8');print('ARCHIVE',a)
starts=[i for i,l in enumerate(lines) if 'SpiritAshResummon' in l and ('0.4.0' in l or 'starting' in l.lower())];print('STARTS',[(i,lines[i]) for i in starts[-6:]])
start=starts[-1] if starts else 0
session=lines[start:]
for l in session:
 if re.search(r'0\.4\.0|Config \{|BuddyStone rules|Anywhere tile|Summon callback|Native summon returned|Heal activation|Add-group native|Disabled:|panic|error|failed',l,re.I):print(l)
print('LATEST HUD',next((l for l in reversed(session) if 'HUD entity snapshot' in l),'none'))
print('LAST NONEMPTY HUD',next((l for l in reversed(session) if 'HUD entity snapshot: members=[Member' in l),'none'))
